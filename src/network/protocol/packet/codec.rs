use crate::network::protocol::*;
use bytes::{Buf, BufMut, BytesMut,};
use tokio_util::codec::{Decoder, Encoder};
use std::sync::{Arc, Mutex};
use std::io::Cursor;

pub struct PacketsCodec {
    pub protocol: Option<Arc<Protocol>>,
}

impl PacketsCodec {
    pub fn new() -> Self {
        Self {
            protocol: None,
        }
    }
}

impl Decoder for PacketsCodec {
    type Item = Box<dyn Packet>;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(src);

        let len = match <Var<i32>>::read(&mut cursor) {
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
        let id = <Var<i32>>::read(&mut cursor)?.0 as usize;
        let protocol = match self.protocol.as_ref() {
            Some(protocol) => protocol,
            None => return Err(anyhow::anyhow!("protocol not set")),
        };

        // read server packet from protocol
        let packet = protocol.server.read_packet(id, &mut cursor);

        match packet {
            Some(packet) => {
                info!("Received: {:?}", packet);
                src.advance(len);

                Ok(Some(packet))
            },
            None => {
                warn!("Unknown or malformed packet: {}", id);
                src.advance(len);

                Ok(None)
            }
        }
    }
}

impl Encoder<PacketPayload> for PacketsCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, payload: PacketPayload, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let id = payload.0;
        let packet = &payload.1;

        let protocol = match self.protocol.as_ref() {
            Some(protocol) => protocol,
            None => return Err(anyhow::anyhow!("protocol not set")),
        };

        let id_var = Var(id as i32);
        let len_var = Var((packet.len() + id_var.len()) as i32);

        // total payload size
        let payload_len = len_var.len() + (len_var.0 as usize);

        // reserve payload
        dst.reserve(payload_len);

        len_var.write(dst)?;
        id_var.write(dst)?;

        protocol.client.write_packet(&payload, dst).ok_or_else(||
            anyhow::anyhow!("failed to send packet"),
        )
    }
}