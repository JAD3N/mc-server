#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum PacketDirection {
    Server,
    Client,
}

impl PacketDirection {
    pub fn opposite(&self) -> Self {
        match self {
            PacketDirection::Server => PacketDirection::Client,
            PacketDirection::Client => PacketDirection::Server,
        }
    }
}