use crate::core::MappedRegistry;
use crate::network::protocol::{Protocol, ProtocolData, ProtocolError, Packet, PacketDirection, Var};
use crate::network::Connection;
use bytes::{Buf, BufMut, BytesMut,};
use tokio_util::codec::{Decoder, Encoder};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::io::Cursor;

pub struct PacketCodec {
    protocols: Arc<MappedRegistry<i32, Protocol>>,
    protocol: i32,
}

impl PacketCodec {
    pub fn new(protocols: Arc<MappedRegistry<i32, Protocol>>) -> Self {
        Self { protocols, protocol: Protocol::DEFAULT }
    }

    fn create_packet(&self, protocol: i32, id: usize, cursor: &mut Cursor<&[u8]>) -> Option<Box<dyn Packet>> {
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

        packet_set.create_packet(id, cursor)
    }
}

impl Decoder for PacketCodec {
    type Item = Box<dyn Packet>;
    type Error = Box<dyn Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(src);

        let len = match Var::<i32>::read(&mut cursor) {
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
        let id = Var::<i32>::read(&mut cursor)?.0 as usize;
        let packet = self.create_packet(self.protocol, id, &mut cursor);

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