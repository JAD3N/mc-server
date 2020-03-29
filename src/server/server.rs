use std::sync::{Arc, Mutex};
use std::time::Instant;
use super::Settings;

pub struct Server {
    settings: Settings,
    is_running: bool,
    tick_count: u64,
    next_tick: u64,
    last_warning: u64,
}

pub type ServerRef = Arc<Mutex<Server>>;

impl Server {
    pub fn new(settings: Settings) -> Server {
        Server {
            settings: settings,
            is_running: false,
            tick_count: 0,
            next_tick: 0,
            last_warning: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn next_tick(&self) -> u64 {
        self.next_tick
    }

    pub fn set_next_tick(&mut self, next_tick: u64) {
        self.next_tick = next_tick;
    }

    pub fn last_warning(&self) -> u64 {
        self.last_warning
    }

    pub fn set_last_warning(&mut self, last_warning: u64) {
        self.last_warning = last_warning;
    }

    pub fn init(&mut self) {
        self.is_running = true;
        self.next_tick = crate::util::get_millis();
    }

    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }

        println!("{}", self.tick_count);

        std::thread::sleep_ms(60000);

        self.tick_count += 1;
    }

}