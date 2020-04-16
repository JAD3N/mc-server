use crate::util;
use super::ServerRef;

const WARNING_THRESHOLD: u64 = 2000;
const WARNING_DELAY: u64 = 15000;

pub struct Ticker {
    server: ServerRef,
}

impl Ticker {
    pub fn new(server: &ServerRef) -> Self {
        Self { server: server.clone() }
    }

    pub fn start(&mut self, tick_period: u64) {
        loop {
            let mut delay = None;

            {
                let mut server = self.server.write().unwrap();

                let now = util::get_millis();
                let tick_delta = now - server.next_tick;

                if tick_delta > WARNING_THRESHOLD && now - server.last_warning >= WARNING_DELAY {
                    let ticks = tick_delta / tick_period;

                    warn!("Can't keep up! Is the server overloaded? Running {}ms or {} ticks behind", tick_delta, ticks);

                    // align next tick back to now
                    server.next_tick += tick_period * ticks;
                }

                // set next tick time
                server.next_tick += tick_period;

                if server.next_tick > now {
                    delay = Some(server.next_tick - now);
                }
            }

            if !self.tick() {
                break;
            }

            if let Some(delay) = delay {
                util::sleep(delay);
            }
        }
    }

    fn tick(&mut self) -> bool {
        let mut server = self.server.write().unwrap();
        let is_running = server.is_running();

        if is_running {
            server.tick();
        }

        is_running
    }
}