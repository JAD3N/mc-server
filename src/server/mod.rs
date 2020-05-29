mod settings;
mod status;
mod executor;

pub use settings::*;
pub use status::*;
pub use executor::*;

use crate::core::Registries;
use crate::world::level::Level;
use crate::network::{Listener, Connection};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use flume::{Sender, Receiver};

pub enum ServerRequest {
    Connected(Connection),
}

pub struct Server {
    pub connections: Vec<Arc<Mutex<Connection>>>,
    pub registries: Arc<Registries>,
    pub settings: Arc<ServerSettings>,
    pub levels: HashMap<String, Arc<Mutex<Level>>>,
    pub tx: Sender<ServerRequest>,
    pub rx: Receiver<ServerRequest>,
}

impl Server {
    pub fn get_level(&self, dimension: &String) -> Option<&Arc<Mutex<Level>>> {
        self.levels.get(dimension)
    }

    pub async fn tick(&mut self) -> anyhow::Result<()> {
        for request in self.rx.try_iter() {
            match request {
                ServerRequest::Connected(connection) => {
                    // check if connection is closed before adding
                    if !connection.is_connected() {
                        continue;
                    }

                    // wrap connection in arc + mutex
                    let connection = Arc::new(Mutex::new(connection));
                    self.connections.push(connection);
                },
            }
        }

        let mut disconnected = vec![];

        // // update all connections
        for (i, connection) in self.connections.iter().enumerate() {
            let mut connection = connection.lock().await;

            // tick connection
            connection.tick();

            if !connection.is_connected() {
                disconnected.push(i);
            }
        }

        // remove disconnected clients from list
        for &i in disconnected.iter().rev() {
            self.connections.remove(i);
        }

        Ok(())
    }
}

pub struct ServerContainer {
    pub server: Arc<Mutex<Server>>,
}

impl ServerContainer {
    pub fn new(registries: Registries, settings: ServerSettings) -> Self {
        let (tx, rx) = flume::unbounded();
        let server = Arc::new(Mutex::new(Server {
            connections: vec![],
            registries: Arc::new(registries),
            settings: Arc::new(settings),
            levels: HashMap::new(),
            // listener channel for connections
            tx, rx,
        }));

        Self { server }
    }

    pub async fn load_levels(&self) -> anyhow::Result<()> {
        info!("loading levels");

        let mut server = self.server.lock().await;

        server.levels.insert(
            String::from("level_1"),
            Arc::new(Mutex::new(Level {
                name: String::from("Level 1"),
                server: self.server.clone(),
            }))
        );

        // server.levels.insert(
        //     String::from("level_2"),
        //     Arc::new(Mutex::new(Level {
        //         name: String::from("Level 2"),
        //         server: self.server.clone(),
        //     }))
        // );

        // server.levels.insert(
        //     String::from("level_3"),
        //     Arc::new(Mutex::new(Level {
        //         name: String::from("Level 3"),
        //         server: self.server.clone(),
        //     }))
        // );

        info!("loaded levels");

        Ok(())
    }

    pub async fn bind(&self, addr: &str) -> anyhow::Result<Listener> {
        let addr = addr.parse()?;
        let server = self.server.clone();
        let server_tx = server.lock().await
            .tx.clone();

        // don't move needed for error handling
        let listener = Listener::bind(
            server_tx,
            server,
            addr,
        ).await?;

        Ok(listener)
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        // create new executor
        let mut executor = ServerExecutor::new(self.server.clone());

        loop {
            executor.execute().await?;
            executor.wait().await;
        }
    }
}