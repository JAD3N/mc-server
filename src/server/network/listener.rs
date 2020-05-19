use std::net::SocketAddr;
use std::io;
use tokio::net::TcpListener;

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self, io::Error> {
        Ok(Self { listener: TcpListener::bind(addr).await? })
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