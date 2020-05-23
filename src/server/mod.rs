mod settings;
mod status;
mod executor;

pub use settings::*;
pub use status::*;
pub use executor::*;

use crate::core::Registries;
use crate::world::level::Level;
use crate::network::Listener;
use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::Mutex;

pub struct Server {
    pub registries: Arc<Registries>,
    pub settings: Arc<ServerSettings>,
    pub levels: HashMap<String, Arc<Mutex<Level>>>,
}

impl Server {
    pub fn get_level(&self, dimension: &String) -> Option<&Arc<Mutex<Level>>> {
        self.levels.get(dimension)
    }

    pub async fn tick(&mut self) -> anyhow::Result<()> {
        info!("did server tick!");
        Ok(())
    }
}

pub struct ServerContainer {
    pub server: Arc<Mutex<Server>>,
}

impl ServerContainer {
    pub fn new(registries: Registries, settings: ServerSettings) -> Self {
        let server = Arc::new(Mutex::new(Server {
            registries: Arc::new(registries),
            settings: Arc::new(settings),
            levels: HashMap::new(),
        }));

        Self { server }
    }

    pub async fn load_levels(&self) -> anyhow::Result<()> {
        let mut server = self.server.lock().await;

        server.levels.insert(
            String::from("level_1"),
            Arc::new(Mutex::new(Level {
                name: String::from("Level 1"),
                server: self.server.clone(),
            }))
        );

        server.levels.insert(
            String::from("level_2"),
            Arc::new(Mutex::new(Level {
                name: String::from("Level 2"),
                server: self.server.clone(),
            }))
        );

        server.levels.insert(
            String::from("level_3"),
            Arc::new(Mutex::new(Level {
                name: String::from("Level 3"),
                server: self.server.clone(),
            }))
        );

        info!("loaded levels");

        Ok(())
    }

    pub async fn listen(&mut self, addr: &str) -> anyhow::Result<()> {
        let addr = addr.parse()?;
        let server = self.server.clone();

        tokio::spawn(async move {
            // create listener
            let mut listener = Listener::bind(
                server,
                addr,
            ).await.unwrap();

            // wait till done
            listener.listen().await;
        });

        Ok(())
    }

    pub async fn execute(&mut self, is_running: Arc<AtomicBool>) -> anyhow::Result<()> {
        // create new executor
        let mut executor = ServerExecutor::new(is_running, self.server.clone());

        loop {
            executor.execute().await?;
            if !executor.wait().await {
                break;
            }
        }

        Ok(())
    }
}