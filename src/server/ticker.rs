use super::ServerRef;
use std::time::{SystemTime, Duration, Instant};
use std::thread;

const TICK: Duration = Duration::from_millis(50);
const WARNING_THRESHOLD: Duration = Duration::from_secs(2);
const WARNING_DELAY: Duration = Duration::from_secs(15);

pub struct Ticker {
    server: ServerRef,
    next_tick: Instant,
    last_warning: Instant,
}

impl Ticker {
    pub fn new(server: &ServerRef) -> Ticker {
        Ticker {
            server: server.clone(),
            next_tick: Instant::now(),
            last_warning: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        let server_ref = &self.server;

        // init server before ticking
        server_ref.lock().unwrap().init();

        // reset next time time
        self.next_tick = Instant::now();

        loop {
            let mut server = server_ref.lock().unwrap();
            let now = Instant::now();

            if server.is_running() {
                // delta times
                let tick_delta = now - self.next_tick;
                let warning_delta = now - self.last_warning;

                if tick_delta > WARNING_THRESHOLD && warning_delta >= WARNING_DELAY {
                    let ticks = (tick_delta.as_millis() / TICK.as_millis()) as u32;

                    println!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", tick_delta.as_millis(), ticks);

                    self.next_tick += TICK * ticks;
                    self.last_warning = self.next_tick;
                }

                self.next_tick += TICK;

                server.tick();
            } else {
                break;
            }

            drop(server_ref);

            thread::sleep(self.next_tick - now);
        }
    }
}