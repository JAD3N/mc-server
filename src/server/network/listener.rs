use std::net::SocketAddr;
use std::io;
use tokio::net::TcpListener;

pub struct Listener {
    addr: SocketAddr,
}

impl Listener {
    pub fn bind(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn listen(&mut self) -> Result<(), io::Error> {
        let mut listener = TcpListener::bind(self.addr).await?;

        loop {
            match listener.accept().await {
                Ok(res) => {
                    // panic!("ahh");
                    println!("Test!");
                },
                Err(_) => {
                    println!("Err!");
                }
            }
        }

        Ok(())
    }
}