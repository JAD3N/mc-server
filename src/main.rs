extern crate java_props;

pub mod server;
pub mod world;
pub mod util;

use std::env;
use std::thread;
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use server::{Server, Settings};

fn get_server_settings() -> Settings {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    server::Settings::load(path)
}

fn await_server_lock(server_ref: Arc<Mutex<Server>>) -> Option<&mut Server> {
    server_ref
}

fn init_server(settings: Settings) {
    let server = Server::new(settings);
    let server = Arc::new(Mutex::new(server));

    let server_ref = server.clone();

    thread::spawn(move || {
        loop {
            {
                let mut test = server_ref.lock().unwrap();
                print!("t1: ");
                test.tick();
            }

            thread::sleep_ms(1000);
        }
    });

    let server_ref = server.clone();

    thread::spawn(move || {
        loop {
            {
                let mut test = server_ref.as_ref().lock().unwrap();
                print!("t2: ");
                test.tick();
            }

            thread::sleep_ms(1000);
        }
    }).join().unwrap();
}

fn main() {
    let settings = get_server_settings();
    init_server(settings);
}
