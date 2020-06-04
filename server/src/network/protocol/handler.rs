use crate::server::ServerShared;
use crate::chat::component::ComponentContainer;
use crate::network::WorkerRequest;
use super::{Protocol, Packet};
use std::sync::Arc;
use async_trait::async_trait;
use flume::Sender;

pub struct ProtocolHandlerState {
    pub shared: Arc<ServerShared>,
    pub protocol: Arc<Protocol>,
    pub worker_tx: Sender<WorkerRequest>,
}

impl ProtocolHandlerState {
    pub fn send_packet<T: Packet>(&self, packet: T) -> anyhow::Result<()> {
        // create payload
        let payload = (self.protocol.client.id_of::<T>().ok_or_else(||
            anyhow::anyhow!("tried to send unknown packet"),
        )?, packet.into_box());

        self.send(WorkerRequest::SendPacket(payload))
    }

    pub fn send(&self, request: WorkerRequest) -> anyhow::Result<()> {
        self.worker_tx.send(request)
            .map_err(|_| anyhow::anyhow!("failed to send request"))
    }
}

#[async_trait]
pub trait ProtocolHandler: mopa::Any + Send + Sync {
    fn new(state: ProtocolHandlerState) -> Self where Self: Sized;

    fn new_box(state: ProtocolHandlerState) -> Box<dyn ProtocolHandler> where Self: Sized {
        Box::new(Self::new(state))
    }

    async fn tick(&mut self) -> anyhow::Result<()> { Ok(()) }
    async fn handle_disconnect(&mut self) -> anyhow::Result<()> { Ok(()) }
}

mopafy!(ProtocolHandler);
