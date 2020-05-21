use crate::core::{ResourceRegistry, RegisterEvent, Sound};

fn register_sounds(event: &mut RegisterEvent<ResourceRegistry<Sound>>) {
    let registry = &mut event.0;

    registry.register_locatable(Sound::new("minecraft:test"));
}

pub fn init() {
    subscribe_event!("main", register_sounds);
}