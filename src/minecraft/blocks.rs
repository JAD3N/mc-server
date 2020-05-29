use server::core::{ResourceRegistry, RegisterEvent};
use server::world::level::{Block, AirBlock};

fn register_blocks(event: &mut RegisterEvent<ResourceRegistry<Box<dyn Block>>>) {
    let registry = &mut event.0;

    registry.register("minecraft:air", Box::new(AirBlock::new()));
}

pub fn init() {
    subscribe_event!("main", register_blocks, 1000);
}