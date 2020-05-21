use crate::server::ServerStatus;
use crate::network::protocol::PacketHandler;

pub struct StatusPacketHandler;

impl PacketHandler for StatusPacketHandler {}

impl StatusPacketHandler {
    pub fn test(&mut self, _packet: &mut StatusRequestPacket) {}
    pub fn test2(&mut self, _packet: &mut PingRequestPacket) {}
}

// // Serverbound
packet!(StatusPacketHandler, StatusRequestPacket {}, test);
packet!(StatusPacketHandler, PingRequestPacket { time: u64 }, test2);

// // Clientbound
packet!(StatusResponsePacket { status: ServerStatus });
packet!(PongResponsePacket { time: u64 });