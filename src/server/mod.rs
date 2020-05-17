pub mod network;

mod settings;
mod status;
mod ticker;

pub use settings::*;
pub use status::*;
pub use ticker::*;

use crate::core::Registries;
use crate::world::level::Level;
use self::network::Listener;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::future;

pub struct Server {
    pub registries: Arc<Registries>,
    pub settings: Arc<Settings>,
    pub levels: HashMap<String, Arc<RwLock<Level>>>,
}

impl Server {
    pub fn get_level(&self, dimension: &String) -> Option<&Arc<RwLock<Level>>> {
        self.levels.get(dimension)
    }

    pub async fn tick(&mut self) {
        self.tick_levels().await;
    }

    pub async fn tick_levels(&mut self) {
        let mut handles = vec![];

        for level in self.levels.values() {
            // clone level as reference would be dropped later
            let level_ref = level.clone();
            let handle = tokio::spawn(async move {
                // create lock inside async so the lock isn't dropped
                let mut level_lock = level_ref.write().await;

                // tick the level and wait for it to finish
                level_lock.tick().await;
            });

            handles.push(handle);
        }

        // wait for all level ticks to finish
        future::join_all(handles).await;
    }
}

pub struct ServerBuilder {
    pub server: Arc<RwLock<Server>>,
    pub listener: Option<Listener>,
}

impl ServerBuilder {
    pub fn new(registries: Registries, settings: Settings) -> Self {
        let server = Arc::new(RwLock::new(Server {
            registries: Arc::new(registries),
            settings: Arc::new(settings),
            levels: HashMap::new(),
        }));

        Self { server, listener: None }
    }

    pub async fn load_levels(&self) {
        let mut server = self.server.write().await;

        server.levels.insert(
            String::from("level_1"),
            Arc::new(RwLock::new(Level {
                server: self.server.clone(),
            }))
        );
    }

    pub async fn tick(&self) {
        let mut server = self.server.write().await;
        server.tick().await;
    }

    pub async fn listen(&mut self) {
        self.listener = Some(Listener::bind(""));
    }
}