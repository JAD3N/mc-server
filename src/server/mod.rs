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
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::{Mutex, RwLock};
use flume::{Sender, Receiver};

pub enum ServerRequest {
    Connected(Arc<RwLock<Connection>>),
}

pub struct Server {
    pub tx: Sender<ServerRequest>,
    pub rx: Receiver<ServerRequest>,
    pub connections: Vec<Arc<RwLock<Connection>>>,
    pub registries: Arc<Registries>,
    pub settings: Arc<ServerSettings>,
    pub levels: HashMap<String, Arc<Mutex<Level>>>,
}

impl Server {
    pub fn get_level(&self, dimension: &String) -> Option<&Arc<Mutex<Level>>> {
        self.levels.get(dimension)
    }

    pub async fn tick(&mut self) -> anyhow::Result<()> {
        // check if any new requests
        for request in self.rx.try_iter() {
            match request {
                ServerRequest::Connected(connection) => self.connections.push(connection),
            }
        }

        let mut disconnected = vec![];

        // update all connections
        for (i, connection) in self.connections.iter().enumerate() {
            let mut connection = connection.write().await;

            // tick connection
            connection.tick();

            if connection.disconnected() {
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
            tx, rx,
            connections: vec![],
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

    pub async fn listen(&mut self, addr: &str) -> anyhow::Result<()> {
        let addr = addr.parse()?;
        let server = self.server.clone();
        let server_tx = {
            // wait for server from mutex
            let server = self.server.lock().await;

            // clone sender
            server.tx.clone()
        };

        tokio::spawn(async move {
            // create listener
            let mut listener = Listener::bind(
                server_tx,
                server,
                addr,
            ).await.unwrap();

            // wait till done
            listener.listen().await.ok();
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