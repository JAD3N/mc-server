mod codec;
mod direction;
mod set;

pub use codec::*;
pub use direction::*;
pub use set::*;

use super::{ProtocolHandler, ProtocolRead, ProtocolWrite};
use async_trait::async_trait;
use std::fmt;

#[async_trait]
pub trait Packet: mopa::Any + ProtocolRead + ProtocolWrite + Send + Sync + fmt::Debug {
    async fn handle(&mut self, _handler: &mut Box<dyn ProtocolHandler>) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn into_box(self) -> Box<dyn Packet> where Self: Sized {
        Box::new(self)
    }
}

pub type PacketPayload = (usize, Box<dyn Packet>);

mopafy!(Packet);

#[macro_export]
macro_rules! packet {
    ($handler:tt, $name:tt, $lfname:tt) => {
        #[async_trait::async_trait]
        impl $crate::network::protocol::Packet for $name {
            async fn handle(&mut self, handler: &mut Box<dyn $crate::network::protocol::ProtocolHandler>) -> Result<(), anyhow::Error> {
                let handler: &mut $handler = handler.downcast_mut::<$handler>().unwrap();
                handler.$lfname(self).await
            }
        }
    };
    ($name:ident) => {
        impl $crate::network::protocol::Packet for $name {}
    };
}