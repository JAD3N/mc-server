use crate::core::MappedRegistry;
use crate::network::protocol::*;
use crate::network::Connection;
use bytes::{Buf, BufMut, BytesMut,};
use tokio_util::codec::{Decoder, Encoder};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::io::Cursor;
use std::any::Any;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketsCodecError {

}

pub struct PacketsCodec {
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    protocol: i32,
}

impl PacketsCodec {
    pub fn new(protocols: Arc<MappedRegistry<i32, Protocol>>) -> Self {
        Self { protocols, protocol: Protocol::DEFAULT }
    }

    fn read_packet(&self, protocol: i32, id: usize, src: &mut Cursor<&[u8]>) -> Option<Box<dyn Packet>> {
        let protocol = match self.protocols.get(&protocol) {
            Some(protocol) => protocol,
            None => {
                warn!("no active protocol set for connection");
                return None;
            }
        };

        let packet_set = match protocol.get(PacketDirection::Server) {
            Some(packet_set) => packet_set,
            None => {
                warn!("no server set for active protocol");
                return None;
            }
        };

        packet_set.read_packet(id, src)
    }

    fn write_packet(&self, protocol: i32, id: usize, packet: Box<dyn Packet>, dst: &mut BytesMut) -> Option<()> {
        let protocol = match self.protocols.get(&protocol) {
            Some(protocol) => protocol,
            None => {
                warn!("no active protocol set for connection");
                return None;
            }
        };

        let packet_set = match protocol.get(PacketDirection::Server) {
            Some(packet_set) => packet_set,
            None => {
                warn!("no server set for active protocol");
                return None;
            }
        };

        packet_set.write_packet(id, &packet, dst)
    }
}

impl Decoder for PacketsCodec {
    type Item = Box<dyn Packet>;
    type Error = Box<dyn Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(src);

        let len = match ProtocolData::<Var<i32>>::read(&mut cursor) {
            Ok(len) => len.0 as usize,
            // not enough bytes to read packet yet
            Err(ProtocolError::NotEnoughBytes) => return Ok(None),
            // error decoding packet
            Err(err) => return Err(err.into()),
        };

        // check if full packet has been received
        if len > cursor.remaining() {
            // not enough bytes to read full packet
            return Ok(None);
        }

        // advance to start of packet
        let pos = cursor.position() as usize;
        src.advance(pos);

        // create new cursor for packet length
        cursor = Cursor::new(&src[..len]);

        // read packet id
        let id = ProtocolData::<Var<i32>>::read(&mut cursor)?.0 as usize;
        let packet = self.read_packet(self.protocol, id, &mut cursor);

        match packet {
            Some(packet) => {
                info!("Received: {:?}", packet);
                src.advance(len);

                Ok(Some(packet))
            },
            None => {
                warn!("Unknown packet: {}", id);
                src.advance(len);

                Ok(None)
            }
        }
    }
}

impl Encoder<Box<dyn Packet>> for PacketsCodec {
    type Error = Box<dyn Error>;

    fn encode(&mut self, packet: Box<dyn Packet>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.write_packet(self.protocol, 0, packet, dst);
        // Ok(packet.write(dst)?)

        // let test = ProtocolData::<Box<dyn Packet>>::write();

        Ok(())
    }
}