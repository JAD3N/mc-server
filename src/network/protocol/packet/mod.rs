mod codec;
mod direction;
mod set;
mod listener;
mod handler;

pub use codec::*;
pub use direction::*;
pub use set::*;
pub use listener::*;
pub use handler::*;

use super::ProtocolData;
use std::any::Any;
use std::fmt;

pub trait Packet: ProtocolData + Send + Sync + fmt::Debug {
    fn handle(&mut self, _listener: &mut dyn Any) -> Option<()> {
        Some(())
    }
}

#[macro_export]
macro_rules! packet {
    ($listener:ty, $name:ident { $($fname:ident: $fty:ty),* $(,)? }, $lfname:ident) => {
        protocol_data_struct!($name {
            $(
                $fname:$fty,
            )*
        });

        impl $crate::network::protocol::Packet for $name {
            fn handle(&mut self, listener: &mut dyn std::any::Any) -> Option<()> {
                // listener.$lfname(self);
                let listener = listener.downcast_mut::<$listener>()?;
                listener.$lfname(self);
                Some(())
            }
        }
    };
    ($name:ident { $($fname:ident: $fty:ty),* $(,)? }) => {
        protocol_data_struct!($name {
            $(
                $fname:$fty,
            )*
        });

        impl $crate::network::protocol::Packet for $name {}
    };
}