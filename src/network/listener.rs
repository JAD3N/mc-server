use crate::server::{Server, ServerRequest};
use crate::core::MappedRegistry;
use super::protocol::Protocol;
use super::Worker;
use std::net::SocketAddr;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use flume::Sender;

pub struct Listener {
    server_tx: Sender<ServerRequest>,
    server: Arc<Mutex<Server>>,
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(
        server_tx: Sender<ServerRequest>,
        server: Arc<Mutex<Server>>,
        addr: SocketAddr,
    ) -> Result<Self, io::Error> {
        // read protocols from registries
        let protocols = server.lock().await
            .registries.protocols.clone();

        Ok(Self {
            server_tx,
            server,
            protocols,
            listener: TcpListener::bind(addr).await?,
        })
    }

    pub async fn listen(&mut self) -> anyhow::Result<()> {
        loop {
            let (stream, addr) = match self.listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    log::info!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            info!("Client connected: {}", addr);

            let mut worker = Worker::new(
                self.server.clone(),
                self.protocols.clone(),
                stream,
            );

            let connection = worker.connection().clone();

            // inform server of new connection
            self.server_tx.send(
                ServerRequest::Connected(connection.clone()),
            ).ok();

            // spawn worker for listening
            tokio::spawn(async move {
                if let Err(e) = worker.listen().await {
                    // inform connection that it's disconnected
                    // connection
                    //     .write().await
                    //     .disconnect();

                    info!("Client disconnected: {}", e);
                }
            });

            tokio::task::yield_now().await;
        }
    }
}