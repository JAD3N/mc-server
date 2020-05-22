use crate::network::protocol::ProtocolError;
use super::{Packet, PacketListener, PacketDirection};
use std::collections::HashMap;
use std::any::{TypeId};
use std::io::Cursor;
use mopa::Any;
use bytes::BytesMut;

type PacketReadInit = fn(&mut Cursor<&[u8]>) -> Result<Box<dyn Packet>, ProtocolError>;
type PacketWriteInit = fn(&Box<dyn Packet>, &mut BytesMut) -> Result<(), ProtocolError>;

pub struct PacketSet {
    id_read_map: HashMap<usize, PacketReadInit>,
    id_write_map: HashMap<usize, PacketWriteInit>,
    type_map: HashMap<TypeId, usize>,
}

impl PacketSet {
    pub fn new() -> Self {
        Self {
            id_read_map: HashMap::new(),
            id_write_map: HashMap::new(),
            type_map: HashMap::new(),
        }
    }

    pub fn read_packet(&self, id: usize, src: &mut Cursor<&[u8]>) -> Option<Box<dyn Packet>> {
        match self.id_read_map.get(&id) {
            Some(packet_init) => {
                match packet_init(src) {
                    Ok(packet) => Some(packet),
                    Err(err) => {
                        error!("Error reading packet: {}", err);
                        None
                    }
                }
            },
            None => None,
        }
    }

    pub fn write_packet(&self, id: usize, packet: &Box<dyn Packet>, dst: &mut BytesMut) -> Option<()> {
        match self.id_write_map.get(&id) {
            Some(packet_init) => {
                match packet_init(packet, dst) {
                    Ok(packet) => Some(packet),
                    Err(err) => {
                        error!("Error reading packet: {}", err);
                        None
                    }
                }
            },
            None => None,
        }
    }

    pub fn id_of<T: 'static + Packet>(&self) -> Option<usize> {
        let type_id = TypeId::of::<T>();

        match self.type_map.get(&type_id) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.type_map.len()
    }

    pub fn add<T: Packet + 'static>(&mut self, packet_read_init: PacketReadInit, packet_write_init: PacketWriteInit) {
        let id = self.len();
        let type_id = TypeId::of::<T>();

        self.id_read_map.insert(id, packet_read_init);
        self.id_write_map.insert(id, packet_write_init);
        self.type_map.insert(type_id, id);
    }

    pub fn wrap<T: Packet + 'static>(packet: Result<T, ProtocolError>) -> Result<Box<dyn Packet>, ProtocolError> {
        Ok(Box::new(packet?))
    }
}