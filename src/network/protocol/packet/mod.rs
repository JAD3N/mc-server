mod codec;
mod direction;
mod registry;
mod listener;

pub use codec::*;
pub use direction::*;
pub use registry::*;
pub use listener::*;

use super::Protocol;

pub struct StatusPacketListener;

impl PacketListener for StatusPacketListener {

}

use std::any::Any;

pub trait Packet: Protocol {
    fn handle(&mut self, listener: &mut dyn Any) -> Option<()> {
        Some(())
    }
}

#[macro_export]
macro_rules! packet {
    ($listener:ty, $name:ident { $($fname:ident: $fty:ty),* $(,)? }, $($lfname:ident)?) => {
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl $crate::network::protocol::Protocol for $name {
            fn len(&self) -> usize {
                0 $(+ (self.$fname).len())*
            }

            fn write<T: std::io::Write>(&self, _dst: &mut T) -> std::io::Result<()> {
                $(self.$fname.write(_dst)?;)*
                Ok(())
            }

            fn read<T: std::io::Read>(_src: &mut T) -> std::io::Result<$name> {
                Ok($name { $($fname: <$fty>::read(_src)?,)* })
            }
        }

        impl $crate::network::protocol::Packet<$listener> for $name {
            $(fn handle(&mut self, listener: &mut dyn std::any::Any) -> Option<()> {
                // listener.$lfname(self);
                let listener = listener.downcast_mut::<StatusPacketListener>()?;
                listener.$lfname(self);
                Some(())
            })?
        }
    };
}