use crate::server::Server;
use crate::core::MappedRegistry;
use super::protocol::{Protocol, PacketListener};
use super::Connection;
use std::net::SocketAddr;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use tokio::net::TcpListener;

pub struct Listener {
    server: Arc<RwLock<Server>>,
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(server: Arc<RwLock<Server>>, addr: SocketAddr) -> Result<Self, io::Error> {
        // read protocols from registries
        let protocols = server.read().await
            .registries.protocols.clone();

        Ok(Self {
            server,
            protocols,
            listener: TcpListener::bind(addr).await?,
        })
    }

    pub async fn listen(&mut self) -> Option<()> {
        loop {
            let (stream, _) = match self.listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    log::info!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            // let connection = Arc::new(Mutex::new(Connection::new()));
            let mut packet_listener = PacketListener::new(
                self.protocols.clone(),
                stream,
            );

            // TODO: Other listener stuff?

            // spawn listener task for connection
            tokio::spawn(async move { packet_listener.listen().await });
            tokio::task::yield_now().await;
        }

        Some(())
    }
}