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

#[macro_use]
mod macros;
pub mod events;
mod logger;
mod minecraft;

use self::core::Registries;
use self::server::{ServerContainer, ServerSettings};
use std::sync::Arc;
use tokio::runtime;
use futures::future;

fn main() -> anyhow::Result<()> {
    // set up
    logger::init();
    events::init();
    minecraft::init();

    let server = Arc::new(ServerContainer::new(
        // uses events to create registries
        Registries::new(),
        // uses normal file path to load settings
        ServerSettings::load(),
    ));

    let mut network_rt = runtime::Builder::new()
        .thread_name("network")
        .core_threads(2)
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let mut server_rt = runtime::Builder::new()
        .thread_name("server")
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    // Bind:
    match network_rt.block_on(server.bind("127.0.0.1:25565")) {
        Err(e) => error!("Network error: {}", e),
        Ok(listener) => {
            let (load_levels, stop_handle_1) = future::abortable(server.load_levels());
            let (execute, stop_handle_2) = future::abortable(server.execute());

            ctrlc::set_handler(move || {
                stop_handle_1.abort();
                stop_handle_2.abort();
            }).ok();

            // Start Up:
            if let Err(e) = server_rt.block_on(load_levels) {
                error!("Fatal error loading levels: {}", e);
            } else {
                // Listen:
                network_rt.spawn(listener.listen());

                // Tick:
                server_rt.block_on(execute).ok();

                // drop network runtime
                drop(network_rt);

                // TODO: Level saves?
            }
        }
    };

    info!("Server shutdown.");

    Ok(())
}
