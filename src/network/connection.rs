use super::protocol::{Protocol, ProtocolHandler, Packet};
use super::{WorkerRequest};
use flume::Sender;

pub struct Connection {
    worker_tx: Sender<WorkerRequest>,
}

impl Connection {
    pub fn new(worker_tx: Sender<WorkerRequest>) -> Self {
        Self { worker_tx }
    }

    pub fn set_protocol(&self, protocol: i32) {
        self.send(WorkerRequest::SetProtocol(protocol));
    }

    pub fn send(&self, request: WorkerRequest) {
        self.worker_tx.send(request).ok().expect("Error!");
    }
}