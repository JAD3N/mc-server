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
mod macros;
mod logger;
pub mod events;
mod minecraft;

#[macro_use]
pub mod util;
pub mod context;
#[macro_use]
pub mod chat;
#[macro_use]
pub mod core;
pub mod server;
pub mod world;
pub mod auth;

use std::error::Error;
use context::Context;

#[tokio::main(core_threads = 3)]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up
    logger::init();
    events::init();
    minecraft::init();

    let ctx = Context::new();

    Ok(())
}