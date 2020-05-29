use server::network::protocol::{ProtocolHandler, ProtocolHandlerState, Var};
use server::network::WorkerRequest;

pub struct HandshakeProtocolHandler {
    state: ProtocolHandlerState,
}

impl ProtocolHandler for HandshakeProtocolHandler {
    fn new(state: ProtocolHandlerState) -> Self {
        Self { state }
    }
}

impl HandshakeProtocolHandler {
    pub async fn handle_intention(&mut self, packet: &mut IntentionPacket) -> Result<(), anyhow::Error> {
        let intention = packet.intention.0;

        println!("Received handshake with intention: {}", intention);

        // valid intentions are either 0 or 1
        match intention {
            0 => self.state.send(WorkerRequest::SetProtocol(0))?,
            1 => self.state.send(WorkerRequest::SetProtocol(1))?,
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
packet!(HandshakeProtocolHandler, IntentionPacket, handle_intention);