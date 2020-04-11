use std::process;
use std::sync::{Arc, RwLock};
use crate::util;
use super::{Server, ServerData, TICK};

pub struct Watcher {
    max_tick_time: i32,
    server_data: Arc<RwLock<ServerData>>,
}

impl Watcher {
    pub fn new(server: &Arc<RwLock<Server>>) -> Watcher {
        let server_lock = server.read().unwrap();
        let server_data = server_lock.data().clone();
        let max_tick_time = server_lock.settings().max_tick_time();

        Watcher {
            max_tick_time,
            server_data,
        }
    }

    pub fn watch(&self) {
        if self.max_tick_time <= 0 {
            return;
        }

        let max_tick_time = self.max_tick_time as u64;

        loop {
            let server_data = self.server_data.write().unwrap();

            // check if server is running
            if !server_data.is_running {
                break;
            }

            let now = util::get_millis();

            if now > server_data.next_tick  {
                let tick_delta = now - server_data.next_tick;

                if tick_delta > max_tick_time {
                    error!("A single server tick took {:.2} seconds (should be max {:.2})", tick_delta as f32 / 1000.0, TICK as f32 / 1000.0);
                    error!("Considering it to be crashed, server will forcibly shutdown.");

                    process::exit(1);
                }
            }

            let delay = server_data.next_tick + max_tick_time - now;

            drop(server_data);
            util::sleep(delay);
        }
    }
}