use server::network::protocol::{ProtocolHandler, ProtocolHandlerState};
use server::server::*;

pub struct LoginProtocolHandler {
    state: ProtocolHandlerState,
}

impl ProtocolHandler for LoginProtocolHandler {
    fn new(state: ProtocolHandlerState) -> Self {
        Self { state }
    }
}

impl LoginProtocolHandler {
    pub async fn handle_status_request(&mut self, _packet: &mut StatusRequestPacket) -> Result<(), anyhow::Error> {
        let status = self.state.shared.status.lock().await;

        self.state.send_packet(StatusResponsePacket {
            status: status.clone(),
        })?;

        Ok(())
    }

    pub async fn handle_ping_request(&mut self, packet: &mut PingRequestPacket)  -> Result<(), anyhow::Error> {
        self.state.send_packet(PongResponsePacket { time: packet.time })?;
        Ok(())
    }
}

protocol_struct!(DisconnectPacket { reason: ComponentContainer });
protocol_struct!(EncryptionRequestPacket { server_id: String, })