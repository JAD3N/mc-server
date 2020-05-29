#[macro_use]
extern crate log;
#[macro_use]
extern crate server;
#[macro_use]
extern crate event_bus;

mod minecraft;

use server::core::Registries;
use server::server::{ServerSettings, ServerContainer};

fn main() {
    server::init();
    minecraft::init();

    let registries = Registries::new();
    let settings = ServerSettings::load();
    let server = ServerContainer::new(registries, settings);

    if let Err(e) = server.start() {
        error!("Server error: {}", e);
    }
}