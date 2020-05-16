use crate::core::{RegisterEvent, Sound};

fn register_sounds(event: &mut RegisterEvent<Sound>) {
    let registry = &mut event.registry;

    registry.register_locatable(Sound::new("minecraft:test"));
}

pub fn init() {
    subscribe_event!("main", register_sounds);
}