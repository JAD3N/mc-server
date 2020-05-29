pub mod status;
pub mod handshake;

use server::core::{RegisterEvent, MappedRegistry};
use server::network::protocol::Protocol;

fn register_protocols(event: &mut RegisterEvent<MappedRegistry<i32, Protocol>>) {
    let protocols = &mut event.0;

    protocols.register(-1, protocol! {
        id: -1,
        handler: handshake::HandshakeProtocolHandler,
        server: [handshake::IntentionPacket],
    });

    protocols.register(1, protocol! {
        id: 1,
        handler: status::StatusProtocolHandler,
        server: [
            status::StatusRequestPacket,
            status::PingRequestPacket,
        ],
        client: [
            status::StatusResponsePacket,
            status::PongResponsePacket,
        ],
    });
}

pub fn init() {
    subscribe_event!("main", register_protocols);
}