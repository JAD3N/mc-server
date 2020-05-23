use crate::server::Server;
use crate::core::MappedRegistry;
use super::protocol::Protocol;
use super::Worker;
use std::net::SocketAddr;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;

pub struct Listener {
    server: Arc<Mutex<Server>>,
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(server: Arc<Mutex<Server>>, addr: SocketAddr) -> Result<Self, io::Error> {
        // read protocols from registries
        let protocols = server.lock().await
            .registries.protocols.clone();

        Ok(Self {
            server,
            protocols,
            listener: TcpListener::bind(addr).await?,
        })
    }

    pub async fn listen(&mut self) -> Option<()> {
        loop {
            let (stream, addr) = match self.listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    log::info!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            info!("Client connected! {}", addr);

            let mut worker = Worker::new(
                self.server.clone(),
                self.protocols.clone(),
                stream,
            );

            let _connection = worker.connection();

            // spawn worker for listening
            tokio::spawn(async move {
                if let Err(e) = worker.listen().await {
                    info!("Client disconnected: {}", e);
                }
            });

            tokio::task::yield_now().await;
        }
    }
}