use crate::network::protocol::{ProtocolHandler, Var};
use crate::network::WorkerRequest;

protocol_handler!(HandshakePacketHandler);

impl HandshakePacketHandler {
    pub async fn handle_intention(&mut self, packet: &mut IntentionPacket) -> Result<(), anyhow::Error> {
        let intention = packet.intention.0;

        println!("Received handshake with intention: {}", intention);

        // valid intentions are either 0 or 1
        match intention {
            0 => self.send(WorkerRequest::SetProtocol(0))?,
            1 => self.send(WorkerRequest::SetProtocol(1))?,
            _ => anyhow::bail!("handshake invalid intention: {}", intention),
        }

        Ok(())
    }
}

protocol_struct!(IntentionPacket {
    protocol_version: Var<i32>,
    hostname: String,
    port: i16,
    intention: Var<i32>,
});

// Serverbound
packet!(HandshakePacketHandler, IntentionPacket, handle_intention);