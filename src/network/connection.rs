use super::protocol::{Packet, PacketListener};
use tokio::net::TcpStream;

pub struct Connection {
    // pub listener: Box<dyn PacketListener>,
    // pub stream: TcpStream,
}

impl Connection {
    // pub fn new() -> Self {
    //     Self {}
    // }

    pub fn handle_packet(&mut self, packet: Box<dyn Packet>) {
        // packet.handle(&mut self.listener);
    }
}

// impl Connection {
//     pub fn handle_packet(packet: Box<dyn Packet<dyn P>>) {

//     }
// }