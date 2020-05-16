use crate::core::{Registry, RegisterEvent, Sound};
use crate::world::level::Block;

pub struct Context {
    pub blocks: Registry<Box<dyn Block>>,
    pub sounds: Registry<Sound>,
}

fn init_registry<T: 'static>(name: &str) -> Registry<T> {
    let mut event = RegisterEvent {
        registry: Registry::new()
    };

    // send event to all subscribers to add to registry
    dispatch_event!("main", &mut event);
    info!("context -> Loaded {}: {}", name, event.registry.len());

   event.registry
}

impl Context {
    pub fn new() -> Self {
        // load registries
        info!("context -> Loading registries...");

        let blocks = init_registry("blocks");
        let sounds = init_registry("sounds");

        info!("context -> Finished loading registries.");

        Self { blocks, sounds }
    }
}