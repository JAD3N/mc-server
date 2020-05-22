use crate::chat::component::TextComponent;
use crate::server::*;
use crate::network::protocol::ProtocolHandler;

protocol_handler!(StatusPacketHandler);

impl StatusPacketHandler {
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

        self.send_packet(StatusResponsePacket { status })?;
        Ok(())
    }

    pub async fn handle_ping_request(&mut self, packet: &mut PingRequestPacket)  -> Result<(), anyhow::Error> {
        self.send_packet(PongResponsePacket { time: packet.time })?;
        Ok(())
    }
}

protocol_struct!(StatusRequestPacket {});
protocol_struct!(PingRequestPacket { time: u64 });
protocol_struct!(StatusResponsePacket { status: ServerStatus });
protocol_struct!(PongResponsePacket { time: u64 });

// Serverbound
packet!(StatusPacketHandler, StatusRequestPacket, handle_status_request);
packet!(StatusPacketHandler, PingRequestPacket, handle_ping_request);

// Clientbound
packet!(StatusResponsePacket);
packet!(PongResponsePacket);