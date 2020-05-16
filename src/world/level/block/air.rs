use super::Block;

pub struct AirBlock;

impl Block for AirBlock {}

impl AirBlock {
    pub fn new() -> Self {
        Self
    }
}