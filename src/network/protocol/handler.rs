use crate::server::Server;
use crate::chat::component::BoxComponent;
use crate::network::WorkerRequest;
use super::{Protocol, Packet};
use tokio::sync::Mutex;
use std::sync::Arc;
use flume::Sender;
use async_trait::async_trait;

#[async_trait]
pub trait ProtocolHandler: mopa::Any + Send + Sync {
    fn new(
        server: Arc<Mutex<Server>>,
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
    ) -> Self where Self: Sized;

    fn new_box(
        server: Arc<Mutex<Server>>,
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
    ) -> Box<dyn ProtocolHandler> where Self: Sized {
        Box::new(Self::new(server, protocol, worker_tx))
    }

    fn send_packet<T: Packet>(&self, packet: T) -> anyhow::Result<()> where Self: Sized;
    fn send(&self, request: WorkerRequest) -> anyhow::Result<()>;

    async fn tick(&mut self) -> anyhow::Result<()> {
        info!("connection tick");
        Ok(())
    }

    async fn handle_disconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub type ProtocolHandlerInit = fn(
    Arc<Mutex<Server>>,
    Arc<Protocol>,
    Sender<WorkerRequest>,
) -> Box<dyn ProtocolHandler>;

mopafy!(ProtocolHandler);

#[macro_export]
macro_rules! protocol_handler {
    ($name:ident) => {
        pub struct $name {
            pub server: std::sync::Arc<tokio::sync::Mutex<$crate::server::Server>>,
            pub protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
            pub worker_tx: flume::Sender<$crate::network::WorkerRequest>,
        }

        impl $crate::network::protocol::ProtocolHandler for $name {
            fn new(
                server: std::sync::Arc<tokio::sync::Mutex<$crate::server::Server>>,
                protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
                worker_tx: flume::Sender<$crate::network::WorkerRequest>,
            ) -> Self {
                Self { server, protocol, worker_tx }
            }

            fn send_packet<T: $crate::network::protocol::Packet>(&self, packet: T) -> anyhow::Result<()> {
                // create payload
                let payload = (self.protocol.client.id_of::<T>().ok_or_else(||
                    anyhow::anyhow!("tried to send unknown packet"),
                )?, packet.into_box());

                self.send($crate::network::WorkerRequest::SendPacket(payload))
            }

            fn send(&self, request: $crate::network::WorkerRequest) -> anyhow::Result<()> {
                self.worker_tx.send(request)
                    .map_err(|_| anyhow::anyhow!("failed to send request"))
            }
        }
    };
}