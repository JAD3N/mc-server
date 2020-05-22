mod kind;
mod dimension;
mod block;
pub mod chunk;

pub use kind::*;
pub use dimension::*;
pub use block::*;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::server::Server;

pub struct Level {
    pub server: Arc<RwLock<Server>>,
    // pub dimension: String,
    // pub chunks: ChunkStore,
}

impl Level {
    pub async fn tick(&mut self) {
        let _server = self.server.write();
        info!("got server!");
    }
}