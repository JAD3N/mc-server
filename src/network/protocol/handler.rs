use crate::network::{Connection, WorkerRequest};
use super::{Protocol, Packet, PacketPayload};
use tokio::sync::RwLock;
use std::sync::Arc;
use flume::Sender;

pub trait ProtocolHandler: mopa::Any + Send + Sync {
    fn new(
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
        connection: Arc<RwLock<Connection>>,
    ) -> Self where Self: Sized;

    fn new_box(
        protocol: Arc<Protocol>,
        worker_tx: Sender<WorkerRequest>,
        connection: Arc<RwLock<Connection>>,
    ) -> Box<dyn ProtocolHandler> where Self: Sized {
        Box::new(Self::new(protocol, worker_tx, connection))
    }

    fn send_packet<T: Packet>(&self) where Self: Sized;
}

pub type ProtocolHandlerInit = fn(
    Arc<Protocol>,
    Sender<WorkerRequest>,
    Arc<RwLock<Connection>>,
) -> Box<dyn ProtocolHandler>;

mopafy!(ProtocolHandler);

#[macro_export]
macro_rules! protocol_handler {
    ($name:ident) => {
        pub struct $name {
            protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
            worker_tx: flume::Sender<$crate::network::WorkerRequest>,
            connection: std::sync::Arc<tokio::sync::RwLock<$crate::network::Connection>>,
        }

        impl $crate::network::protocol::ProtocolHandler for $name {
            fn new(
                protocol: std::sync::Arc<$crate::network::protocol::Protocol>,
                worker_tx: flume::Sender<$crate::network::WorkerRequest>,
                connection: std::sync::Arc<tokio::sync::RwLock<$crate::network::Connection>>,
            ) -> Self {
                Self { protocol, worker_tx, connection }
            }

            fn send_packet<T: $crate::network::protocol::Packet>(&self) {
                let id = self.protocol.client.id_of::<T>();
            }
        }
    };
}