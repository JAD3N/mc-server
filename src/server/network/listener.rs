use std::net::SocketAddr;
use std::io;
use tokio::net::TcpListener;

pub struct Listener {
    listener: TcpListener,
    addr: SocketAddr,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener, addr })
    }

    pub async fn listen(&mut self) {
        loop {
            match self.listener.accept().await {
                Ok(res) => {
                    // panic!("ahh");
                    println!("Test!");
                },
                Err(_) => {
                    println!("Err!");
                }
            }
        }
    }
}