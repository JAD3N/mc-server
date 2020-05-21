use crate::network::protocol::{Var, PacketHandler};

pub struct HandshakePacketHandler;

impl PacketHandler for HandshakePacketHandler {}

impl HandshakePacketHandler {
    pub fn handle_intention(&mut self, _packet: &mut IntentionPacket) {}
}

packet!(HandshakePacketHandler, IntentionPacket {
    protocol_version: Var<i32>,
    hostname: String,
    port: i16,
    intention: Var<i32>,
}, handle_intention);
