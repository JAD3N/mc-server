use crate::util;
use super::{Server, ServerData};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TICK: u64 = 50;
const WARNING_THRESHOLD: u64 = 2000;
const WARNING_DELAY: u64 = 15000;

pub struct Ticker {
    server: Arc<Mutex<Server>>,
    server_data: Arc<Mutex<ServerData>>,
}

impl Ticker {
    pub fn new(server: Arc<Mutex<Server>>) -> Ticker {
        let server_data = server.lock()
            .unwrap()
            .data()
            .clone();

        Ticker { server, server_data }
    }

    fn tick(&mut self) -> Option<u64> {
        let mut server_data = self.server_data.lock().unwrap();

        if server_data.is_running {
            let now = util::get_millis();
            let tick_delta = now - server_data.next_tick;

            if tick_delta > WARNING_THRESHOLD && now - server_data.last_warning >= WARNING_DELAY {
                let ticks = tick_delta / TICK;

                println!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", tick_delta, ticks);

                server_data.next_tick += TICK * ticks;
                server_data.last_warning = server_data.next_tick;
            }

            server_data.next_tick += TICK;

            // store delay
            let delay = server_data.next_tick - now;

            // prevent mutex lock
            drop(server_data);

            // trigger server tick
            self.server.lock()
                .unwrap()
                .tick();

            Some(delay)
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
                while self.server_data.lock().unwrap().is_running {
                    match self.tick() {
                        Some(duration) => thread::sleep(Duration::from_millis(duration)),
                        None => continue,
                    }
                }
            }))
        }
    }
}