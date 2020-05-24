use super::{WorkerRequest};
use flume::Sender;

pub struct Connection {
    connected: bool,
    worker_tx: Sender<WorkerRequest>,
}

impl Connection {
    pub fn new(worker_tx: Sender<WorkerRequest>) -> Self {
        Self { connected: true, worker_tx }
    }

    pub fn set_protocol(&self, protocol: i32) {
        self.send(WorkerRequest::SetProtocol(protocol));
    }

    pub fn send(&self, request: WorkerRequest) {
        if self.connected {
            self.worker_tx.send(request).ok().expect("Error!");
        }
    }

    pub fn tick(&mut self) {
        self.send(WorkerRequest::Tick);
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        info!("i have been disconnceted!");
    }

    pub fn disconnected(&self) -> bool {
        !self.connected
    }
}