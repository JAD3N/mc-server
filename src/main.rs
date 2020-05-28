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

use self::core::Registries;
use self::server::{ServerSettings, ServerContainer};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_ref = is_running.clone();

    // add ctrl-c handler for smooth close
    ctrlc::set_handler(move ||
        is_running_ref.clone()
            .store(false, Ordering::SeqCst)
    ).ok();

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

    // try bind port
    if let Err(e) = server.listen("127.0.0.1:25565").await {
        error!("Error creating socket: {}", e);
        return Ok(());
    }

    // try load world
    server.load_levels().await?;

    // create separate runtime for server ticking
    Runtime::new()?.spawn(async move {
        if let Err(e) = server.execute(is_running.clone()).await {
            error!("Fatal error running server executor: {}", e);
        }
    }).await?;

    Ok(())
}