pub mod network;

mod settings;
mod status;
mod ticker;

pub use ticker::*;

use std::sync::{Arc, RwLock};

pub static mut SERVER: Option<Arc<RwLock<Server>>> = None;

pub struct Server {
    pub id: usize,
}

pub fn tick() {
    let server = get_server!();
}

pub fn init() {
    unsafe { SERVER = Some(Arc::new(RwLock::new(Server { id: 123 }))); }
}