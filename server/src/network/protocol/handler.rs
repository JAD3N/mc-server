use crate::server::Server;
use crate::chat::component::BoxComponent;
use crate::network::WorkerRequest;
use super::{Protocol, Packet};
use async_trait::async_trait;

pub struct ProtocolHandlerState {
    pub server: std::sync::Arc<tokio::sync::Mutex<Server>>,
    pub protocol: std::sync::Arc<Protocol>,
    pub worker_tx: flume::Sender<WorkerRequest>,
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
