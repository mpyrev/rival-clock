mod gamesense;

use std::time::{Duration, SystemTime};
use std::thread;

use chrono::offset::Local;
use chrono::DateTime;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Clock {
    api: gamesense::GameSense,
}

impl Clock {
    pub fn new(api: gamesense::GameSense) -> Clock {
        Clock {
            api: api,
        }
    }

    pub fn init(&self) {
        let app_register_info = gamesense::AppRegisterInfo {
            game: "CLOCK".to_owned(),
            game_display_name: "Rival Clock".to_owned(),
            developer: "mikhail.pyrev@gmail.com".to_owned(),
        };
        self.api.register_app(app_register_info);

        let mut screen_modifier = Map::new();
        screen_modifier.insert("has-text".to_owned(), Value::Bool(true));
        let event_register_info = gamesense::EventRegisterInfo {
            game: "CLOCK".to_owned(),
            event: "TICK".to_owned(),
            handlers: vec![
                gamesense::EventHandler {
                    device_type: "screened-128x36".to_owned(),
                    zone: "one".to_owned(),
                    mode: "screen".to_owned(),
                    datas: vec![screen_modifier],
                },
            ],
        };
        self.api.bind_game_event(event_register_info);
    }

    pub fn run(&self) {
        loop {
            self.tick();
            thread::sleep(Duration::from_secs(1));
        };
    }

    pub fn tick(&self) {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();

        let mut data = Map::new();
        data.insert("value".to_owned(), Value::String(datetime.format("%H:%M:%S").to_string()));

        let event = gamesense::Event {
            game: "CLOCK".to_owned(),
            event: "TICK".to_owned(),
            data: data,
        };
        self.api.game_event(event);
    }
}

fn main() {
    let gamesense = gamesense::GameSense::load();
    let clock = Clock::new(gamesense);
    clock.init();
    clock.run();
}
