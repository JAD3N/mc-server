#[macro_use]
mod data;

#[macro_use]
mod packet;

pub use data::*;
pub use packet::*;

use crate::core::MappedRegistry;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("not enough bytes remaining")]
    NotEnoughBytes,

    #[error("invalid value")]
    Invalid,

    #[error("value is too large")]
    TooLarge,

    #[error("unknown protocol error")]
    Unknown,
}

pub struct Protocol {
    id: i32,
    packets: MappedRegistry<PacketDirection, PacketSet>,
}

impl Protocol {
    pub const DEFAULT: i32 = -1;

    pub fn new(id: i32) -> Self {
        Self { id, packets: MappedRegistry::new() }
    }

    pub fn get(&self, key: PacketDirection) -> Option<&Arc<PacketSet>> {
        self.packets.get(&key)
    }

    pub fn register(&mut self, dir: PacketDirection, set: PacketSet) {
        self.packets.register(dir, set);
    }
}

#[macro_export]
macro_rules! protocol {
    {
        id: $id:expr
        $(, server: [$($sp:ty),* $(,)?] $(,)?)?
        $(, client: [$($cp:ty),* $(,)?] $(,)?)?
    } => {
        {
            use $crate::network::protocol::{ProtocolData, PacketSet, PacketDirection};

            let mut protocol = Protocol::new($id);

            $({
                let mut packets = PacketSet::new();
                $(packets.add::<$sp>(|src| PacketSet::wrap(<$sp>::read(src)));)*
                protocol.register(PacketDirection::Server, packets);
            })?

            $({
                let mut packets = PacketSet::new();
                $(packets.add::<$cp>(|src| PacketSet::wrap(<$cp>::read(src)));)*
                protocol.register(PacketDirection::Client, packets);
            })?

            protocol
        }
    };
}