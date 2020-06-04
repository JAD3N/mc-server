use crate::server::{ServerShared, ServerRequest};
use crate::core::MappedRegistry;
use super::protocol::Protocol;
use super::{Worker, Connection};
use std::net::SocketAddr;
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;
use flume::Sender;

pub struct Listener {
    server_tx: Sender<ServerRequest>,
    shared: Arc<ServerShared>,
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(
        server_tx: Sender<ServerRequest>,
        shared: Arc<ServerShared>,
        addr: SocketAddr,
    ) -> Result<Self, io::Error> {
        // read protocols from registries
        let protocols = shared.registries.protocols.clone();
        let listener = TcpListener::bind(addr).await?;

        info!("Binded to address: {}", addr);

        Ok(Self {
            server_tx,
            shared,
            protocols,
            listener,
        })
    }

    pub async fn listen(mut self) -> anyhow::Result<()> {
        loop {
            let (stream, _addr) = match self.listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    info!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            let mut connection = Connection::new();
            let mut worker = Worker::new(
                &mut connection,
                self.shared.clone(),
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