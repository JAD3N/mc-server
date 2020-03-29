use crate::util::get_millis;
use super::ServerRef;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TICK: u64 = 50;
const WARNING_THRESHOLD: u64 = 2000;
const WARNING_DELAY: u64 = 15000;

pub struct Ticker {
    server: ServerRef,
}

impl Ticker {
    pub fn new(server: ServerRef) -> Ticker {
        Ticker { server }
    }

    fn tick(&mut self) -> Option<u64> {
        let mut server = self.server.lock().unwrap();

        if server.is_running() {
            let mut next_tick = server.next_tick();
            let last_warning = server.last_warning();

            let now = get_millis();
            let tick_delta = now - next_tick;

            if tick_delta > WARNING_THRESHOLD && now - last_warning >= WARNING_DELAY {
                let ticks = tick_delta / TICK;

                println!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", tick_delta, ticks);

                next_tick += TICK * ticks;
                server.set_last_warning(next_tick);
            }

            next_tick += TICK;

            server.set_next_tick(next_tick);
            server.tick();

            Some(next_tick - now)
        } else {
            None
        }
    }

    pub fn run(mut self) -> Option<thread::JoinHandle<()>> {
        let mut server = self.server.lock().ok()?;

        if server.is_running() {
            None
        } else {
            server.init();
            drop(server);

            Some(thread::spawn(move || {
                while self.server.lock().unwrap().is_running() {
                    match self.tick() {
                        Some(duration) => thread::sleep(Duration::from_millis(duration)),
                        None => continue,
                    }
                }
            }))
        }
    }
}