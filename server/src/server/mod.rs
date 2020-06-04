mod settings;
mod status;
mod executor;

pub use settings::*;
pub use status::*;
pub use executor::*;

use crate::core::Registries;
use crate::chat::component::TextComponent;
use crate::world::level::Level;
use crate::network::{Listener, Connection};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::runtime;
use futures::future;
use flume::{Sender, Receiver};

pub static VERSION_NAME: &str = "1.15.2";
pub static VERSION_STABLE: bool = true;
pub static WORLD_VERSION: u32 = 2230;
pub static PROTOCOL_VERSION: u32 = 578;
pub static PACK_VERSION: u32 = 5;
pub static RELEASE_TARGET: &str = "1.15.2";

pub enum ServerRequest {
    Connected(Connection),
}

pub struct ServerShared {
    pub registries: Arc<Registries>,
    pub settings: Arc<ServerSettings>,
    pub status: Arc<Mutex<ServerStatus>>,
}

pub struct Server {
    pub shared: Arc<ServerShared>,
    pub connections: Vec<Arc<Mutex<Connection>>>,
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
    pub shared: Arc<ServerShared>,
}

impl ServerContainer {
    pub fn new(registries: Registries, settings: ServerSettings) -> Self {
        let status = ServerStatus {
            description: TextComponent::new(settings.motd()).into(),
            version: ServerStatusVersion {
                name: String::from(VERSION_NAME),
                protocol: PROTOCOL_VERSION,
            },
            players: ServerStatusPlayers {
                max_players: settings.max_players(),
                num_players: 0,
                sample: vec![],
            },
            // TODO: Load icon from server-icon.png or level icon.png and convert to base64
            favicon: None,
        };

        let shared = Arc::new(ServerShared {
            registries: Arc::new(registries),
            settings: Arc::new(settings),
            status: Arc::new(Mutex::new(status)),
        });

        let (tx, rx) = flume::unbounded();
        let server = Arc::new(Mutex::new(Server {
            shared: shared.clone(),
            connections: vec![],
            levels: HashMap::new(),
            tx, rx,
        }));

        Self { server, shared }
    }

    async fn load_levels(&self) -> anyhow::Result<()> {
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

    async fn bind(&self, addr: SocketAddr) -> anyhow::Result<Listener> {
        let shared = self.shared.clone();
        let server = self.server.clone();
        let server_tx = server.lock().await
            .tx.clone();

        // don't move needed for error handling
        let listener = Listener::bind(
            server_tx,
            shared,
            addr,
        ).await?;

        Ok(listener)
    }

    async fn execute(&self) -> anyhow::Result<()> {
        // create new executor
        let mut executor = ServerExecutor::new(self.server.clone());

        loop {
            executor.execute().await?;
            executor.wait().await;
        }
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let addr = self.shared.settings.addr().parse::<SocketAddr>()?;

        let mut network_rt = runtime::Builder::new()
            .thread_name("network")
            .core_threads(2)
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        let mut server_rt = runtime::Builder::new()
            .thread_name("server")
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        // Bind:
        match network_rt.block_on(self.bind(addr)) {
            Err(e) => error!("Network error: {}", e),
            Ok(listener) => {
                let (load_levels, stop_handle_1) = future::abortable(self.load_levels());
                let (execute, stop_handle_2) = future::abortable(self.execute());

                ctrlc::set_handler(move || {
                    stop_handle_1.abort();
                    stop_handle_2.abort();
                }).ok();

                // Start Up:
                if let Err(e) = server_rt.block_on(load_levels) {
                    error!("Fatal error loading levels: {}", e);
                } else {
                    // Listen:
                    network_rt.spawn(listener.listen());

                    // Tick:
                    server_rt.block_on(execute).ok();

                    // drop network runtime
                    drop(network_rt);

                    // TODO: Level saves?
                }
            }
        };

        info!("Server shutdown.");

        Ok(())
    }
}