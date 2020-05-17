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
#[macro_use]
pub mod chat;
#[macro_use]
pub mod core;
pub mod server;
pub mod world;
pub mod auth;

use std::error::Error;
use self::core::Registries;
use self::server::{Settings, ServerBuilder};

use std::thread;
use tokio::task;
use futures::future;

async fn tick() {
    info!("tick start!");

    task::spawn(tick_chunks()).await.unwrap();

    info!("tick end!");
}

async fn tick_chunks() {
    let mut handles = vec![];

    handles.push(task::spawn(tick_c1()));
    handles.push(task::spawn(tick_c2()));

    future::join_all(handles).await;
}

async fn tick_c1() {
    info!("c1 start");
    thread::sleep_ms(2000);
    info!("c1 end");
}

async fn tick_c2() {
    info!("c2 start");
    thread::sleep_ms(1000);
    info!("c2 end");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // set up
    logger::init();
    events::init();
    minecraft::init();

    let mut server = ServerBuilder::new(
        // uses events to create registries
        Registries::new(),
        // uses normal file path to load settings
        Settings::load(),
    );

    server.load_levels().await;

    let time_start = crate::util::get_nanos();
    server.tick().await;
    let time_end = crate::util::get_nanos();
    info!("ticky took {}", time_end - time_start);

    Ok(())
}
/*

Region {
    level: Arc<RwLock<Level>>
    chunks = ChunkPos[]

    tick() {
        for all chunks {
            tick_chunk(chunk_pos);
        }
    }

    tick_chunk(chunk_pos) {
        // get tickable stuff
        // tick block
    }
}
*/