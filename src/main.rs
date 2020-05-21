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
pub mod server;
pub mod world;
pub mod auth;

#[macro_use]
mod macros;
mod logger;
pub mod events;
mod minecraft;

use std::error::Error;
use self::core::Registries;
use self::server::{ServerSettings, ServerContainer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up
    logger::init();
    events::init();
    minecraft::init();

    let mut server = ServerContainer::new(
        // uses events to create registries
        Registries::new(),
        // uses normal file path to load settings
        ServerSettings::load(),
    );

    server.load_levels().await;
    server.listen("127.0.0.1:25565").await?;

    let time_start = crate::util::get_nanos();
    server.tick().await;
    let time_end = crate::util::get_nanos();
    info!("ticky took {}", time_end - time_start);

    loop {}

    Ok(())

}
