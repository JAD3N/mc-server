use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::io::{self, Read};
use super::{Packet, PacketListener, PacketDirection};

pub struct PacketRegistry {
    id: usize,
    packets: HashMap<PacketDirection, PacketSet>,
}

impl PacketRegistry {
    pub fn new(id: usize) -> Self {
        PacketRegistry { id, packets: HashMap::new() }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn register(mut self, dir: PacketDirection, set: PacketSet) -> Self {
        self.packets.insert(dir, set);
        self
    }
}

type PacketInit = fn(Box<&mut dyn Read>) -> io::Result<Box<dyn Packet>>;

pub struct PacketSet {
    idMap: HashMap<usize, PacketInit>,
    typeMap: HashMap<TypeId, usize>,
}

impl PacketSet {
    pub fn new() -> Self {
        Self {
            idMap: HashMap::new(),
            typeMap: HashMap::new(),
        }
    }

    pub fn get_packet<T: Packet, V: Read>(&self, id: usize, src: &mut V) -> Option<Box<T>> {
        match self.idMap.get(&id) {
            Some(packetFn) => {
                // match packetFn(src) {
                //     Ok(packet) => packet.downcast::<T>().ok(),
                //     Err(err) => {
                //         error!("Error getting packet: {}", err);
                //         None
                //     }
                // }
                None
            },
            None => None,
        }
    }

    pub fn get_id<T: 'static + Packet>(&self) -> Option<usize> {
        let type_id = TypeId::of::<T>();

        match self.typeMap.get(&type_id) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.idMap.len()
    }

    pub fn add<T: Packet + 'static>(mut self, packet_init: PacketInit) -> Self {
        let id = self.len();
        let type_id = TypeId::of::<T>();

        self.idMap.insert(id, packet_init);
        self.typeMap.insert(type_id, id);

        self
    }

    pub fn wrap<T: Packet + 'static>(packet: io::Result<T>) -> io::Result<Box<dyn Any>> {
        Ok(Box::new(packet?))
    }
}