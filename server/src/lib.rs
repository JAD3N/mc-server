#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mopa;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate event_bus;

#[macro_use]
pub mod util;
#[macro_use]
pub mod chat;
#[macro_use]
pub mod core;
#[macro_use]
pub mod network;
pub mod auth;
pub mod server;
pub mod world;
pub mod events;
mod logger;

pub fn init() {
    // set up
    logger::init();
    events::init();
}