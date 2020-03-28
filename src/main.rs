extern crate java_props;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod server;
pub mod world;
pub mod util;
// pub mod registry;
pub mod core;

use std::env;
use std::thread;
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use server::{Server, Settings, Ticker};

fn get_server_settings() -> Settings {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    server::Settings::load(path)
}

// fn await_server_lock(server_ref: Arc<Mutex<Server>>) -> Option<&mut Server> {
//     server_ref
// }

fn init_server(settings: Settings) {
    let server = Server::new(settings);
    let server = Arc::new(Mutex::new(server));

    let server_ref = server.clone();
    let server_thread = thread::spawn(
        move || Ticker::new(&server_ref).run()
    );

    server_thread.join().unwrap();
}

fn main() {
    let settings = get_server_settings();
    init_server(settings);
}
