use super::protocol::PacketCodec;
use std::net::SocketAddr;
use std::io;
use tokio::stream::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self, io::Error> {
        Ok(Self { listener: TcpListener::bind(addr).await? })
    }

    pub async fn listen(&mut self) -> Option<()> {
        loop {
            let (socket, _) = self.listener.accept().await.ok()?;
            socket.nodelay();

            tokio::spawn(async move {
                let mut packets = Framed::new(socket, PacketCodec::new());

                loop {
                    let packet = packets.next().await;
                }
            });
        }

        Some(())
    }
}