use crate::core::MappedRegistry;
use crate::network::{Connection, protocol::Protocol};
use super::{PacketCodec, Packet};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use flume::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use futures::{StreamExt, future::Either};

pub struct PacketListener {
    framed: Framed<TcpStream, PacketCodec>,
    tx: Sender<Box<dyn Packet>>,
    rx: Arc<Mutex<Receiver<Box<dyn Packet>>>>,
}

impl PacketListener {
    pub fn new(
        protocols: Arc<MappedRegistry<i32, Protocol>>,
        stream: TcpStream,
    ) -> Self {
        let codec = PacketCodec::new(protocols);
        let framed = Framed::new(stream, codec);

        let (tx, rx) = flume::unbounded();
        let rx = Arc::new(Mutex::new(rx));

        Self { framed, tx, rx }
    }

    pub async fn listen(&mut self) {
        loop {
            let packet = self.framed.next();

            packet.await;
        }
    }
}