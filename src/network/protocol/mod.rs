#[macro_use]
mod io;
#[macro_use]
mod packet;
#[macro_use]
mod handler;

pub use io::*;
pub use packet::*;
pub use handler::*;

use super::protocol::ProtocolHandlerInit;
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
    pub id: i32,
    pub handler: ProtocolHandlerInit,
    pub server: PacketSet,
    pub client: PacketSet,
}

impl Protocol {
    pub const DEFAULT: i32 = -1;

    pub fn new<T: ProtocolHandler>(id: i32) -> Self {
        Self {
            id,
            handler: T::new_box,
            server: PacketSet::new(),
            client: PacketSet::new(),
        }
    }
}

#[macro_export]
macro_rules! protocol {
    {
        id: $id:expr,
        handler: $handler:ty
        $(, server: [$($sp:ty),* $(,)?] $(,)?)?
        $(, client: [$($cp:ty),* $(,)?] $(,)?)?
    } => {
        {
            use $crate::network::protocol::{ProtocolRead, ProtocolWrite, PacketSet};

            let mut protocol = Protocol::new::<$handler>($id);

            $({
                $(protocol.server.add::<$sp>(
                    |src| PacketSet::wrap(<$sp>::read(src)),
                    |packet, dst| {
                        packet.downcast_ref::<$sp>()
                            .unwrap()
                            .write(dst)
                    },
                );)*
            })?

            $({
                $(protocol.client.add::<$cp>(
                    |src| PacketSet::wrap(<$cp>::read(src)),
                    |packet, dst| {
                        packet.downcast_ref::<$cp>()
                            .unwrap()
                            .write(dst)
                    },
                );)*
            })?

            protocol
        }
    };
}