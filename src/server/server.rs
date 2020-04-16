use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
// use futures::executor::{ThreadPool, ThreadPoolBuilder};
use crate::util;
use super::{Settings, Ticker};

const TICK_SAMPLE: f32 = 10.0;

pub struct Server {
    pub settings: Settings,
    pub is_running: AtomicBool,

    pub tick_count: u64,
    pub next_tick: u64,
    pub last_warning: u64,

    pub tick_times: [u64; 100],
    pub average_tick_time: f32,
}

pub type ServerRef = Arc<RwLock<Server>>;

impl Server {
    pub fn new(settings: Settings) -> Self {
        // let pool = ThreadPoolBuilder::new()
        //     .name_prefix("Server ")
        //     .create()
        //     .unwrap();

        // let data = Arc::new(RwLock::new(ServerData {
        //     is_running: false,
        //     tick_count: 0,
        //     next_tick: 0,
        //     last_warning: 0,
        // }));

        // let status = ServerStatus::new();

        // Server {
        //     pool,
        //     settings,
        //     status,
        //     data,
        //     tick_times: [0; 100],
        //     average_tick_time: 0.0,
        // }
        Self {
            settings,
            is_running: AtomicBool::from(false),

            tick_count: 0,
            next_tick: 0,
            last_warning: 0,

            tick_times: [0; 100],
            average_tick_time: 0.0,
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    fn init(server: &ServerRef) {
        let mut server = server.write().unwrap();

        // mark server as running
        server.is_running.store(true, Ordering::Relaxed);
        server.next_tick = util::get_millis();

        // do any init stuff here (e.g. watcher, and anything that uses server ref)
    }

    pub fn start(server: &ServerRef) -> thread::JoinHandle<()> {
        let server = server.clone();

        Server::init(&server);

        let thread_builder = thread::Builder::new()
            .name(String::from("Server thread"));

        thread_builder.spawn(move ||
            Ticker::new(&server).start(50)
        ).unwrap()
    }

    pub fn tick(&mut self) {
        if !self.is_running() {
            return;
        }

        let start_time = util::get_nanos();

        self.tick_count += 1;

        // TODO: Tick command functions
        // TODO: Tick connections (new connections?)
        // TODO: Tick player list

        if self.tick_count % 6000 == 0 {
            // TODO: Save player list
            // TODO: Save all chunks
        }

        let total_time = util::get_nanos() - start_time;

        self.tick_times[(self.tick_count % 100) as usize] = total_time;

        self.average_tick_time *= (TICK_SAMPLE - 1.0) / TICK_SAMPLE;
        self.average_tick_time += (total_time as f32) / TICK_SAMPLE;

        if self.tick_count % 20 == 0 {
            info!("Tick time: {}", self.average_tick_time);
        }
    }
}