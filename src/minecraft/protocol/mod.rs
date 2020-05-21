pub mod status;

use crate::core::RegisterEvent;
use crate::network::protocol::{Protocol, Packet, PacketRegistry, PacketDirection, PacketSet};



// new PacketSet<T>().addPacket(ServerboundStatusRequestPacket.class, ServerboundStatusRequestPacket::new)
//                     .addPacket(ServerboundPingRequestPacket.class, ServerboundPingRequestPacket::new))
//             .addFlow(PacketFlow.CLIENTBOUND,
//                     new PacketSet<T>()
//                             .addPacket(ClientboundStatusResponsePacket.class, ClientboundStatusResponsePacket::new)
//                             .addPacket(ClientboundPongResponsePacket.class, ClientboundPongResponsePacket::new))),

fn register_blocks(event: &mut RegisterEvent<PacketRegistry>) {
    let registry = &mut event.registry;

    registry.register(
        "minecraft:status",
        PacketRegistry::new(2)
            // .register(
            //     PacketDirection::Server,
            //     PacketSet::new()
            //         .add::<status::StatusRequestPacket>(Box::new(|src| PacketSet::wrap(status::StatusRequestPacket::read(src))))
            //         // .add::<status::PingRequestPacket, status::StatusListener>(|src| status::PingRequestPacket::read_as_box(src))
            // )
            // .register(
            //     PacketDirection::Client,
            //     PacketSet::new()
            //         .add::<status::StatusResponsePacket, status::StatusListener>(|src| status::StatusResponsePacket::read_as_box(src))
            //         .add::<status::PongResponsePacket, status::StatusListener>(|src| status::PongResponsePacket::read_as_box(src))
            // ),
    );
}

pub fn init() {
    subscribe_event!("main", register_blocks);
}