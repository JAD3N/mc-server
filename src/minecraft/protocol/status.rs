use server::network::protocol::{ProtocolHandler, ProtocolHandlerState};
use server::chat::component::TextComponent;
use server::server::*;

pub struct StatusProtocolHandler {
    state: ProtocolHandlerState,
}

impl ProtocolHandler for StatusProtocolHandler {
    fn new(state: ProtocolHandlerState) -> Self {
        Self { state }
    }
}

impl StatusProtocolHandler {
    pub async fn handle_status_request(&mut self, _packet: &mut StatusRequestPacket) -> Result<(), anyhow::Error> {
        // self.send_packet(PongResponsePacket { time: 123 });

        let mut status = ServerStatus::new();

        status.version = Some(ServerStatusVersion {
            name: String::from("§2ur gay"),
            protocol: 1000,
        });

        status.description = Some(TextComponent::from_str("§cTa dah!").into());

        status.players = Some(ServerStatusPlayers {
            max_players: 1337,
            num_players: 420,
            sample: vec![],
        });

        self.state.send_packet(StatusResponsePacket { status })?;
        Ok(())
    }

    pub async fn handle_ping_request(&mut self, packet: &mut PingRequestPacket)  -> Result<(), anyhow::Error> {
        self.state.send_packet(PongResponsePacket { time: packet.time })?;
        Ok(())
    }
}

protocol_struct!(StatusRequestPacket {});
protocol_struct!(PingRequestPacket { time: u64 });
protocol_struct!(StatusResponsePacket { status: ServerStatus });
protocol_struct!(PongResponsePacket { time: u64 });

// Serverbound
packet!(StatusProtocolHandler, StatusRequestPacket, handle_status_request);
packet!(StatusProtocolHandler, PingRequestPacket, handle_ping_request);

// Clientbound
packet!(StatusResponsePacket);
packet!(PongResponsePacket);