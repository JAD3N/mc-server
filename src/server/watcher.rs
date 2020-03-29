use std::thread;
use std::process;
use std::time::Duration;
use crate::{util::get_millis, server::ServerRef};

pub struct Watcher {
    max_tick_time: i32,
    server: ServerRef,
}

impl Watcher {
    pub fn new(max_tick_time: i32, server: ServerRef) -> Watcher {
        Watcher {
            max_tick_time,
            server: server,
        }
    }

    pub fn watch(&self) {
        if self.max_tick_time <= 0 {
            return;
        }

        let max_tick_time = self.max_tick_time as u64;

        loop {
            let server = self.server.lock().unwrap();

            if !server.is_running() {
                break;
            }

            let next_tick = server.next_tick();
            let now = get_millis();

            if now > next_tick  {
                let tick_delta = now - next_tick;

                if tick_delta > max_tick_time {
                    eprintln!("A single server tick took {} seconds (should be max {})", tick_delta / 1000, max_tick_time / 1000);
                    eprintln!("Considering it to be crashed, server will forcibly shutdown.");

                    process::exit(1);
                }
            }

            // prevent server mutex being locked
            drop(server);

            // wait
            thread::sleep(Duration::from_millis(next_tick + max_tick_time - now));
        }
    }
}