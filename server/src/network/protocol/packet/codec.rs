use crate::network::protocol::*;
use bytes::{Buf, BytesMut};
use bytes::buf::BufMutExt;
use tokio_util::codec::{Decoder, Encoder};
use std::sync::Arc;
use std::io::{Cursor, Read, Write};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

const HEADER_SIZE: usize = 5 * 2; // Max size of 2 VarInts is 10 bytes
const HEADER_EMPTY: [u8; HEADER_SIZE] = [0u8; HEADER_SIZE];

pub struct PacketsCodec {
    pub protocol: Option<Arc<Protocol>>,
    pub compression_threshold: Option<usize>,
    compression_buffer: Vec<u8>,
}

impl PacketsCodec {
    pub fn new() -> Self {
        Self {
            protocol: None,
            compression_threshold: None,
            compression_buffer: vec![],
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

        // create new cursor for packet contents
        cursor = Cursor::new(&src[..len]);

        if let Some(threshold) = self.compression_threshold {
            let data_len = <Var<i32>>::read(&mut cursor)?.0 as usize;

            // data length of zero means no compression
            if data_len != 0 {
                // clear compression buffer
                self.compression_buffer.clear();

                // decode into buffer
                let mut decoder = ZlibDecoder::new(cursor);
                decoder.read_to_end(&mut self.compression_buffer)?;

                if self.compression_buffer.len() < threshold {
                    return Err(anyhow::anyhow!("packet smaller than compression threshold"));
                }

                // change cursor to decompressed packet contents
                cursor = Cursor::new(&self.compression_buffer);
            }
        }

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

        // reserve header size
        dst.reserve(10);
        dst.extend_from_slice(&HEADER_EMPTY);

        // split header from dst
        let mut header = dst.split_to(HEADER_SIZE);

        let id_var = Var(id as i32);
        let packet_len = packet.len() + id_var.len();

        // reserve bytes to write packet
        dst.reserve(packet_len);

        // write packet to dst
        id_var.write(dst)?;
        protocol.client.write_packet(&payload, dst)?;

        let data_len_var = if let Some(threshold) = self.compression_threshold {
            if packet_len >= threshold {
                let packet_data = dst.split_to(packet_len);

                // create encoder using packet data
                let mut encoder = ZlibEncoder::new(
                    dst.writer(),
                    Compression::default(),
                );

                encoder.write_all(&packet_data)?;

                // packet is compressed
                Some(Var(packet_len as i32))
            } else {
                // no compression
                Some(Var(0))
            }
        } else {
            None
        };

        // total length of data len var
        let data_len_var_len = match &data_len_var {
            Some(data_len_var) => data_len_var.len(),
            None => 0,
        };

        let len_var = Var((dst.len() + data_len_var_len) as i32);
        let header_len = len_var.len() + data_len_var_len;

        // offset header
        header.advance(HEADER_SIZE - header_len);
        header.clear();

        // write length to header
        len_var.write(&mut header)?;

        // write data length to header if compression enabled
        if let Some(data_len_var) = &data_len_var {
            data_len_var.write(&mut header)?;
        }

        // swap so header is at start of unsplit
        std::mem::swap(dst, &mut header);
        dst.unsplit(header);

        Ok(())
    }
}