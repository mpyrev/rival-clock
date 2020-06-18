use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use serde_json;
use serde_json::{Map, Value};
use reqwest;
use reqwest::header::CONTENT_TYPE;

#[derive(Debug)]
pub struct GameSense {
    address: String,
}

impl GameSense {
    pub fn new(address: String) -> GameSense {
        GameSense {
            address: address,
        }
    }

    pub fn load() -> GameSense {
        GameSense {
            address: get_address(),
        }
    }

    pub fn get_host(&self) -> String {
        format!("http://{}", self.address)
    }

    pub fn request(&self, uri: String, data: String) {
        let url = self.get_host() + uri.as_str();
        let client = reqwest::blocking::Client::new();
        client.post(&url)
            .header(CONTENT_TYPE, "application/json".to_owned())
            .body(data)
            .send()
            .expect("Cannot access gamesense service");
    }

    pub fn register_app(&self, data: AppRegisterInfo) {
        let json = serde_json::to_string(&data).expect("Failed to convert data to JSON");
        self.request("/game_metadata".to_owned(), json);
    }

    pub fn bind_game_event(&self, data: EventRegisterInfo) {
        let json = serde_json::to_string(&data).expect("Failed to convert data to JSON");
        self.request("/bind_game_event".to_owned(), json);
    }

    pub fn game_event(&self, data: Event) {
        let json = serde_json::to_string(&data).expect("Failed to convert data to JSON");
        self.request("/game_event".to_owned(), json);
    }
}

fn get_programdata_path() -> Option<String> {
    let key = "PROGRAMDATA";
    env::var(key).ok()
}

fn get_config_path(base_path: String) -> String {
    let path: PathBuf = [base_path.as_str(), "SteelSeries", "SteelSeries Engine 3", "coreProps.json"].iter().collect();
    String::from(path.to_str().unwrap())
}

fn get_address() -> String {
    let programdata_path = get_programdata_path();
    let programdata_path: String = match programdata_path {
        Some(value) => value,
        None => panic!("Cannot get PROGRAMDATA environment variable"),
    };
    let config_path: String = get_config_path(programdata_path);
    let json_string = load_file(config_path);
    let json: Value = serde_json::from_str(&json_string).expect("Error reading json from config file");
    match json["address"].as_str() {
        None => panic!("Cannot find `address` value in the config file"),
        Some(address) => String::from(address),
    }
}

fn load_file(path: String) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

#[derive(Serialize, Debug)]
pub struct AppRegisterInfo {
    pub game: String,
    pub game_display_name: String,
    pub developer: String,
}

#[derive(Serialize, Debug)]
pub struct EventRegisterInfo {
    pub game: String,
    pub event: String,
    pub handlers: Vec<EventHandler>,
}
#[derive(Serialize, Debug)]
pub struct EventHandler {
    #[serde(rename = "device-type")]
    pub device_type: String,
    pub zone: String,
    pub mode: String,
    pub datas: Vec<Map<String, Value>>,
}

#[derive(Serialize, Debug)]
pub struct Event {
    pub game: String,
    pub event: String,
    pub data: Map<String, Value>,
}
