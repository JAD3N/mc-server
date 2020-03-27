use std::sync::Mutex;
use super::Settings;

pub struct Server {
    settings: Settings,
    tick: u32,
}

impl Server {

    pub fn new(settings: Settings) -> Server {
        Server {
            settings: settings,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        let settings = &self.settings;
        println!("Tick {}! {:?}", self.tick, settings.game_mode());
        self.tick += 1;
    }

}