use crate::server::Server;
use crate::network::{Connection, WorkerRequest};
use super::{Protocol, Packet};
use tokio::sync::{RwLock, Mutex};
use std::sync::Arc;
use flume::Sender;
use async_trait::async_trait;

#[async_trait]
pub trait ProtocolHandler: mopa::Any + Send + Sync {
    fn new(
        server: Arc<Mutex<Server>>,
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
        connection: Arc<RwLock<Connection>>,
    ) -> Self where Self: Sized;

    fn new_box(
        server: Arc<Mutex<Server>>,
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
        connection: Arc<RwLock<Connection>>,
    ) -> Box<dyn ProtocolHandler> where Self: Sized {
        Box::new(Self::new(server, protocol, worker_tx, connection))
    }

    fn send_packet<T: Packet>(&self, packet: T) -> anyhow::Result<()> where Self: Sized;

    async fn tick(&mut self) -> anyhow::Result<()> {
        info!("connection tick");
        Ok(())
    }
}

pub type ProtocolHandlerInit = fn(
    Arc<Mutex<Server>>,
    Arc<Protocol>,
    Sender<WorkerRequest>,
    Arc<RwLock<Connection>>,
) -> Box<dyn ProtocolHandler>;

mopafy!(ProtocolHandler);

#[macro_export]
macro_rules! protocol_handler {
    ($name:ident) => {
        pub struct $name {
            pub server: std::sync::Arc<tokio::sync::Mutex<$crate::server::Server>>,
            pub protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
            pub worker_tx: flume::Sender<$crate::network::WorkerRequest>,
            pub connection: std::sync::Arc<tokio::sync::RwLock<$crate::network::Connection>>,
        }

        impl $crate::network::protocol::ProtocolHandler for $name {
            fn new(
                server: std::sync::Arc<tokio::sync::Mutex<$crate::server::Server>>,
                protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
                worker_tx: flume::Sender<$crate::network::WorkerRequest>,
                connection: std::sync::Arc<tokio::sync::RwLock<$crate::network::Connection>>,
            ) -> Self {
                Self { server, protocol, worker_tx, connection }
            }

            fn send_packet<T: $crate::network::protocol::Packet>(&self, packet: T) -> anyhow::Result<()> {
                // create payload
                let payload = (self.protocol.client.id_of::<T>().ok_or_else(||
                    anyhow::anyhow!("tried to send unknown packet"),
                )?, packet.into_box());

                // send payload to worker
                self.worker_tx.send($crate::network::WorkerRequest::SendPacket(payload))
                    .map_err(|_| anyhow::anyhow!("failed to send payload"))?;

                Ok(())
            }
        }
    };
}