use crate::util;
use super::{Server, ServerData};
use std::thread;
use std::sync::{Arc, Mutex};

pub const TICK: u64 = 50;
pub const WARNING_THRESHOLD: u64 = 2000;
pub const WARNING_DELAY: u64 = 15000;

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

                warn!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", tick_delta, ticks);

                server_data.next_tick += TICK * ticks;
                server_data.last_warning = server_data.next_tick;
            }

            server_data.next_tick += TICK;

            let delay = if server_data.next_tick > now {
                Some(server_data.next_tick - now)
            } else {
                None
            };

            drop(server_data);

            self.server.lock()
                .unwrap()
                .tick();

            delay
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

            let thread_builder = thread::Builder::new()
                .name(String::from("Server thread"));

            Some(thread_builder.spawn(move || {
                while self.server_data.lock().unwrap().is_running {
                    if let Some(delay) = self.tick() {
                        util::sleep(delay);
                    }
                }
            }).unwrap())
        }
    }
}