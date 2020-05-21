use crate::server::ServerStatus;
use crate::network::protocol::PacketListener;

// pub struct StatusPacketListener;

// impl PacketListener for StatusPacketListener {

// }

// impl StatusPacketListener {
//     pub fn test(&mut self, packet: &mut StatusRequestPacket) {}
//     pub fn test2(&mut self, packet: &mut PingRequestPacket) {}
//     pub fn test3(&mut self, packet: &mut StatusResponsePacket) {}
//     pub fn test4(&mut self, packet: &mut PongResponsePacket) {}
// }

// // Serverbound
// packet!(StatusListener, StatusRequestPacket {}, test);

// packet!(StatusListener, PingRequestPacket { time: u64 }, test2);

// // Clientbound
// packet!(StatusListener, StatusResponsePacket { status: ServerStatus }, test3);
// packet!(StatusListener, PongResponsePacket { time: u64 }, test4);