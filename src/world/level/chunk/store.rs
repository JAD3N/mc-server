use super::Chunk;
use std::sync::{Arc, RwLock};

pub struct ChunkStore {
    pub chunks: Vec<Arc<RwLock<Chunk>>>,
}