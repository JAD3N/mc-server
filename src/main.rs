#[macro_use]
extern crate async_trait;
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
use self::server::{ServerSettings, ServerBuilder};

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

use std::sync::{Arc, Mutex};

trait Listener {
    fn do_stuff(&mut self, connection: &mut Connection);
}

struct Connection {
    listener: Arc<Mutex<Box<dyn Listener>>>,
}

struct ConnectionContainer {
    connection: Connection,
}

impl ConnectionContainer {
    fn do_stuff(&mut self) {
        let listener = self.connection.listener.clone();
        listener.lock().unwrap().do_stuff(&mut self.connection);
    }
}

// impl Listener for ListenerA {
//     fn do_Stuff(&mut self, connection: &mut ConnectionInner)
// }

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