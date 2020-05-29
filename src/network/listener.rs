use crate::server::{Server, ServerRequest};
use crate::core::MappedRegistry;
use super::protocol::Protocol;
use super::{Worker, Connection};
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

    pub async fn listen(mut self) -> anyhow::Result<()> {
        loop {
            let (stream, _addr) = match self.listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    log::info!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            let mut connection = Connection::new();
            let mut worker = Worker::new(
                &mut connection,
                self.server.clone(),
                self.protocols.clone(),
                stream,
            );

            // add connection to array
            self.server_tx.send(ServerRequest::Connected(connection)).ok()
                .expect("failed to accept connection");

            // spawn worker for listening
            tokio::spawn(async move { worker.execute().await });
            tokio::task::yield_now().await;
        }
    }
}