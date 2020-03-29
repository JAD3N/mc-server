#[macro_use]
extern crate lazy_static;
extern crate java_props;
extern crate regex;

pub mod server;
pub mod world;
pub mod util;
pub mod core;

use std::env;
use std::sync::{Arc, Mutex};
use server::{Server, Settings, Ticker, Watcher};

fn get_server_settings() -> Settings {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    server::Settings::load(path)
}

fn main() {
    let settings = get_server_settings();

    let server = Arc::new(Mutex::new(Server::new(settings)));
    let ticker = Ticker::new(server.clone());
    let ticker_handle = ticker.run();

    if let Some(ticker_handle) = ticker_handle {
        Watcher::new(
            5000,
            server.clone()
        ).watch();

        ticker_handle.join().unwrap();
    }
}
