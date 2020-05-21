use super::Packet;
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};
use std::error::Error;

pub struct PacketCodec {
    old_size: usize,
}

impl PacketCodec {
    pub fn new() -> Self {
        Self { old_size: 0 }
    }
}

impl Decoder for PacketCodec {
    type Item = Box<dyn Packet>;
    type Error = Box<dyn Error>;

    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Self::Item>, Self::Error> {
        if self.old_size < src.len() {
            info!("{}", src.len());
            self.old_size = src.len();
        }
        Ok(None)
    }
}