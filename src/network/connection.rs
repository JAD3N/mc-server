use super::protocol::Packet;

pub struct Connection {}

impl Connection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_packet(&mut self, packet: Box<dyn Packet>) {
        // packet.handle(&mut self.listener);
    }
}