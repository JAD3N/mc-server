use crate::core::RegisterEvent;
use crate::world::level::{Block, AirBlock};

fn register_blocks(event: &mut RegisterEvent<Box<dyn Block>>) {
    let registry = &mut event.registry;

    registry.register("minecraft:air", Box::new(AirBlock::new()));
}

pub fn init() {
    subscribe_event!("main", register_blocks);
}