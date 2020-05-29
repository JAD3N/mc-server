use crate::chat::component::BoxComponent;
use super::{WorkerRequest};
use flume::Sender;

pub struct Connection {
    is_disconnected: bool,
    worker_tx: Option<Sender<WorkerRequest>>,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            is_disconnected: false,
            worker_tx: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        !self.is_disconnected && self.worker_tx.is_some()
    }

    pub fn is_connecting(&self) -> bool {
        !self.worker_tx.is_some()
    }

    pub fn attach_worker(&mut self, worker_tx: Sender<WorkerRequest>) {
        self.is_disconnected = false;
        self.worker_tx = Some(worker_tx);
    }

    pub fn send(&mut self, request: WorkerRequest) -> bool {
        if !self.is_connected() {
            return false;
        }

        // returns whether the request sent
        if let Err(_) = self.worker_tx.as_ref()
            .unwrap()
            .send(request) {
            self.is_disconnected = true;

            false
        } else {
            true
        }
    }

    pub fn tick(&mut self) {
        self.send(WorkerRequest::Tick);
    }

    pub fn disconnect(&mut self, reason: BoxComponent) {
        self.is_disconnected = true;
        self.send(WorkerRequest::Disconnect(reason));
    }
}