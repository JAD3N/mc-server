use std::net::SocketAddr;
use std::io;
use tokio::net::TcpListener;

pub struct Listener {
    address: SocketAddr,
}

impl Listener {
    pub fn bind(address: SocketAddr) -> Self {
        Self { address }
    }

    pub async fn listen(&mut self) -> Result<(), io::Error> {
        let mut listener = TcpListener::bind(self.address).await?;

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