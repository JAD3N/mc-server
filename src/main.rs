#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mopa;
#[macro_use]
extern crate serde_json;

#[macro_use]
pub mod util;
pub mod core;
pub mod server;
pub mod world;
#[macro_use]
pub mod chat;

use std::env;
use std::sync::{Arc, RwLock};
use server::{Server, Settings, Ticker, Watcher};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;

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

fn main() {
    init_logger();
    info!("Starting server...");

    use chat::component::{Component, TextComponent};
    use util::JsonValue;

    let mut c = TextComponent::from_str("This is a test!");
    c.style_mut().borrow_mut().bold = Some(true);
    c.style_mut().borrow_mut().italic = Some(true);

    c.append(Box::new(TextComponent::from_str("test 2")));
    c.append(Box::new(TextComponent::from_str("test 3")));

    info!("{}", c.to_json().unwrap());

    let settings = get_server_settings();

    let server = Arc::new(RwLock::new(Server::new(settings)));
    let ticker = Ticker::new(server.clone());
    let ticker_handle = ticker.run();

    info!("{}This is a red {}This is blue", chat::Color::RED, chat::Color::BLUE);

    if let Some(ticker_handle) = ticker_handle {
        Watcher::new(&server).watch();

        ticker_handle.join().unwrap();
    }
}