use std::sync::{Arc, Mutex};
use crate::util;
use super::Settings;

// used to store data independently from the server struct
pub struct ServerData {
    pub is_running: bool,
    pub tick_count: u64,
    pub next_tick: u64,
    pub last_warning: u64,
}

pub struct Server {
    settings: Settings,
    data: Arc<Mutex<ServerData>>,
}

impl Server {
    pub fn new(settings: Settings) -> Server {
        let data = Arc::new(Mutex::new(ServerData {
            is_running: false,
            tick_count: 0,
            next_tick: 0,
            last_warning: 0,
        }));

        Server { settings, data }
    }

    pub fn data(&self) -> &Arc<Mutex<ServerData>> {
        &self.data
    }

    pub fn is_running(&self) -> bool {
        self.data.lock().unwrap().is_running
    }

    pub fn tick_count(&self) -> u64 {
        self.data.lock().unwrap().tick_count
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn init(&mut self) {
        let mut data = self.data.lock().unwrap();

        data.is_running = true;
        data.tick_count = 0;
        data.next_tick = util::get_millis();
        data.last_warning = 0;
    }

    pub fn tick(&mut self) {
        {
            let mut data = self.data.lock().unwrap();
            if !data.is_running {
                return;
            }

            data.tick_count += 1;

            println!("tick {}", data.tick_count);
        }

        std::thread::sleep_ms(60000);
    }

}