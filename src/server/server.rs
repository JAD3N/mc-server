use std::sync::{Arc, RwLock};
use futures::executor::{ThreadPool, ThreadPoolBuilder};
use crate::util;
use super::Settings;

const TICK_SAMPLE: f32 = 10.0;

// used to store data independently from the server struct
pub struct ServerData {
    pub is_running: bool,
    pub tick_count: u64,
    pub next_tick: u64,
    pub last_warning: u64,
}

pub struct Server {
    pool: ThreadPool,
    settings: Settings,
    data: Arc<RwLock<ServerData>>,
    tick_times: [u64; 100],
    average_tick_time: f32,
}

impl Server {
    pub fn new(settings: Settings) -> Server {
        let pool = ThreadPoolBuilder::new()
            .name_prefix("Server ")
            .create()
            .unwrap();

        let data = Arc::new(RwLock::new(ServerData {
            is_running: false,
            tick_count: 0,
            next_tick: 0,
            last_warning: 0,
        }));

        Server {
            pool,
            settings,
            data,
            tick_times: [0; 100],
            average_tick_time: 0.0,
        }
    }

    pub fn data(&self) -> &Arc<RwLock<ServerData>> {
        &self.data
    }

    pub fn is_running(&self) -> bool {
        self.data.read().unwrap().is_running
    }

    pub fn tick_count(&self) -> u64 {
        self.data.read().unwrap().tick_count
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn init(&mut self) {
        let mut data = self.data.write().unwrap();

        data.is_running = true;
        data.tick_count = 0;
        data.next_tick = util::get_millis();
        data.last_warning = 0;
    }

    pub fn tick(&mut self) {
        let start_time = util::get_nanos();
        let tick_count = {
            let mut data = self.data.write().unwrap();
            if !data.is_running {
                return;
            }

            data.tick_count += 1;
            data.tick_count
        };

        // TODO: Tick command functions
        // TODO: Tick connections (new connections?)
        // TODO: Tick player list

        if tick_count % 6000 == 0 {
            // TODO: Save player list
            // TODO: Save all chunks
        }

        let total_time = util::get_nanos() - start_time;

        self.tick_times[(tick_count % 100) as usize] = total_time;

        self.average_tick_time *= (TICK_SAMPLE - 1.0) / TICK_SAMPLE;
        self.average_tick_time += (total_time as f32) / TICK_SAMPLE;

        if tick_count % 20 == 0 {
            info!("Tick time: {}", self.average_tick_time);
        }
    }
}