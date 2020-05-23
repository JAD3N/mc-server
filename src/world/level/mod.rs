mod kind;
mod dimension;
mod block;
pub mod chunk;

pub use kind::*;
pub use dimension::*;
pub use block::*;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::server::Server;

pub struct Level {
    pub name: String,
    pub server: Arc<Mutex<Server>>,
    // pub dimension: String,
    // pub chunks: ChunkStore,
}

impl Level {
    pub async fn tick(&mut self) -> anyhow::Result<()> {
        // let _server = self.server.lock().await;
        info!("did level tick {}!", self.name);
        Ok(())
    }
}