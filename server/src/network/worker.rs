use crate::chat::component::ComponentContainer;
use crate::server::ServerShared;
use crate::core::MappedRegistry;
use super::Connection;
use super::protocol::{Protocol, ProtocolHandler, ProtocolHandlerState, PacketsCodec, Packet, PacketPayload};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tokio_io_timeout::TimeoutStream;
use std::sync::Arc;
use std::time::Duration;
use flume::{Sender, Receiver};
use futures::future::{self, Either};
use futures::{SinkExt, StreamExt};

pub enum WorkerRequest {
    Tick,
    SendPacket(PacketPayload),
    SetProtocol(i32),
    Disconnect(ComponentContainer),
}

pub struct Worker {
    handler: Option<Box<dyn ProtocolHandler>>,

    shared: Arc<ServerShared>,
    protocols: Arc<MappedRegistry<i32, Protocol>>,

    framed: Framed<TimeoutStream<TcpStream>, PacketsCodec>,
    tx: Sender<WorkerRequest>,
    rx: Receiver<WorkerRequest>,
}

impl Worker {
    pub fn new(
        connection: &mut Connection,
        shared: Arc<ServerShared>,
        protocols: Arc<MappedRegistry<i32, Protocol>>,
        stream: TcpStream,
    ) -> Self {
        let mut stream = TimeoutStream::new({
            stream.set_nodelay(true).ok();
            stream
        });

        // add 30 second read timeout
        stream.set_read_timeout(Some(Duration::from_secs(30)));

        let handler = None;

        let codec = PacketsCodec::new();
        let framed = Framed::new(stream, codec);
        let (tx, rx) = flume::unbounded();

        connection.attach_worker(tx.clone());

        let mut worker = Self { handler, shared, protocols, framed, tx, rx };
        worker.set_protocol(Protocol::DEFAULT);
        worker
    }

    pub async fn execute(&mut self) {
        if let Err(_) = self.listen().await {
            if let Some(handler) = self.handler.as_mut() {
                // ignore handle disconnect error
                handler.handle_disconnect().await.ok();
            }
        }
    }

    async fn listen(&mut self) -> anyhow::Result<()> {
        loop {
            let request = self.rx.next();
            let packet = self.framed.next();

            match future::select(request, packet).await {
                Either::Left((request, _)) => {
                    if let Some(request) = request {
                        self.handle_request(request).await?;
                    }
                },
                Either::Right((packet_res, _)) => {
                    // if future fails, the client has disconnected
                    let packet_res = packet_res.ok_or_else(||
                        anyhow::anyhow!("lost connection"),
                    )?;

                    // bubble errors to stop worker
                    self.handle_packet(packet_res?).await?;
                },
            }

            tokio::task::yield_now().await;
        }
    }

    pub fn set_protocol(&mut self, protocol: i32) {
        let protocol = match self.protocols.get(&protocol) {
            Some(protocol) => {
                let state = ProtocolHandlerState {
                    shared: self.shared.clone(),
                    protocol: protocol.clone(),
                    worker_tx: self.tx.clone(),
                };

                let handler_init = protocol.handler;
                let handler = handler_init(state);

                self.handler = Some(handler);
                Some(protocol.clone())
            },
            None => {
                self.handler = None;
                None
            },
        };

        // change protocol on codec
        self.framed.codec_mut().protocol = protocol;
    }

    async fn handle_request(&mut self, request: WorkerRequest) -> anyhow::Result<()> {
        match request {
            WorkerRequest::Tick => {
                if let Some(handler) = self.handler.as_mut() {
                    handler.tick().await?;
                }
            },
            WorkerRequest::SendPacket(packet) => self.framed.send(packet).await?,
            WorkerRequest::SetProtocol(protocol) => self.set_protocol(protocol),
            WorkerRequest::Disconnect(reason) => anyhow::bail!("i'm dead!"),
        }

        Ok(())
    }

    async fn handle_packet(&mut self, mut packet: Box<dyn Packet>) -> anyhow::Result<()> {
        if let Some(handler) = self.handler.as_mut() {
            packet.handle(handler).await?;
        }

        Ok(())
    }
}