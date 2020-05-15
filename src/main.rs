extern crate tokio;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mopa;
#[macro_use]
extern crate serde_json;
extern crate proc_macro;

#[macro_use]
pub mod util;
#[macro_use]
pub mod chat;
#[macro_use]
pub mod core;
pub mod server;
pub mod world;
pub mod auth;

use std::env;
use std::sync::{Arc, RwLock};
use server::{Server, Settings};
use self::core::registry::Registrable;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use futures::join;

const LOG_PATTERN: &str = "[{d(%H:%M:%S)}] [{thread}/{h({level})}]: {m}{n}";

fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build();

    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build("logs/server.log", Box::new(
            CompoundPolicy::new(
                Box::new(SizeTrigger::new(10_000_000)),
                Box::new(FixedWindowRoller::builder().build("logs/server-{}.log.gz", 10).unwrap()),
            ),
        ))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info)
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn get_server_settings() -> Settings {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    server::Settings::load(path)
}

fn register() {
    debug!("Loading assets...");

    core::sound::Sound::register();

    debug!("Finished loading assets.");
}

use self::core::events::*;

struct SimpleEvent;
impl Event for SimpleEvent {}

struct CancellableEvent {
    cancelled: bool,
}

impl Event for CancellableEvent {
    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn set_cancelled(&mut self, cancel: bool) {
        self.cancelled = cancel;
    }
}

fn on_simple_event_1(event: &mut SimpleEvent) {
    println!("SimpleEvent triggered 1!");
}

fn on_simple_event_2(event: &mut SimpleEvent) {
    println!("SimpleEvent triggered 2!");
}

fn on_cancellable_event_1(event: &mut CancellableEvent) {
    println!("CancellableEvent triggered 1!");
    event.cancel();
}

fn on_cancellable_event_2(event: &mut CancellableEvent) {
    println!("CancellableEvent triggered 2!");
}

fn test_events() {
    let bus = EventBus::new("main");

    subscribe_event!("main", on_simple_event_1);
    subscribe_event!("main", on_simple_event_2);

    subscribe_event!("main", on_cancellable_event_1);
    subscribe_event!("main", on_cancellable_event_2);

    dispatch_event!("main", &mut CancellableEvent { cancelled: false });
}

#[tokio::main(core_threads = 2, max_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    register();

    info!("Starting server...");

    test_events();

    // let ticker_handle = tokio::spawn(async {
    //     let mut ticker = time::interval(Duration::from_millis(500));

    //     loop {
    //         info!("Tick!");

    //         ticker.tick().await;
    //     }
    // });

    // let server = Server::new(get_server_settings());
    // let server = Arc::new(RwLock::new(server));

    // let mut listener = server::network::Listener::bind("127.0.0.1:25565".parse().unwrap());

    // let listener_handle = tokio::spawn(async move {
    //     listener.listen().await.unwrap();
    // });

    Ok(())
}
