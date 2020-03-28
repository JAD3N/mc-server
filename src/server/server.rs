use std::sync::{Arc, Mutex};
use super::Settings;

pub struct Server {
    settings: Settings,
    is_running: bool,
    tick: u32,
}

pub type ServerRef = Arc<Mutex<Server>>;

impl Server {
    pub fn new(settings: Settings) -> Server {
        Server {
            settings: settings,
            is_running: false,
            tick: 0,
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

    pub fn init(&mut self) {
        self.is_running = true;
    }

    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }

        println!("{}", self.tick);

        self.tick += 1;
    }

}