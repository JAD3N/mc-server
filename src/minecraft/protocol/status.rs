use crate::server::ServerStatus;
use crate::network::protocol::ProtocolHandler;
use crate::network::Connection;
use tokio::sync::RwLock;
use std::sync::Arc;

protocol_handler!(StatusPacketHandler);

impl StatusPacketHandler {
    pub async fn handle_status_request(&mut self, _packet: &mut StatusRequestPacket) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub async fn handle_ping_request(&mut self, _packet: &mut PingRequestPacket)  -> Result<(), anyhow::Error> {
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