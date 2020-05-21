use crate::network::protocol::ProtocolError;
use super::{Packet, PacketListener, PacketDirection};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::io::Cursor;
use bytes::Buf;

type PacketInit = fn(&mut Cursor<&[u8]>) -> Result<Box<dyn Packet>, ProtocolError>;

pub struct PacketSet {
    id_map: HashMap<usize, PacketInit>,
    type_map: HashMap<TypeId, usize>,
}

impl PacketSet {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
            type_map: HashMap::new(),
        }
    }

    pub fn create_packet(&self, id: usize, src: &mut Cursor<&[u8]>) -> Option<Box<dyn Packet>> {
        match self.id_map.get(&id) {
            Some(packet_init) => {
                match packet_init(src) {
                    Ok(packet) => Some(packet),
                    Err(err) => {
                        error!("Error creating packet: {}", err);
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
        self.id_map.len()
    }

    pub fn add<T: Packet + 'static>(&mut self, packet_init: PacketInit) {
        let id = self.len();
        let type_id = TypeId::of::<T>();

        self.id_map.insert(id, packet_init);
        self.type_map.insert(type_id, id);
    }

    pub fn wrap<T: Packet + 'static>(packet: Result<T, ProtocolError>) -> Result<Box<dyn Packet>, ProtocolError> {
        Ok(Box::new(packet?))
    }
}