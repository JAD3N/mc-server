use super::{ResourceLocation, ResourceLocatable, Sound};
use crate::world::level::Block;
use event_bus::Event;
use std::sync::Arc;

#[derive(Debug)]
pub struct Registry<T> {
    pub keys: Vec<ResourceLocation>,
    pub values: Vec<Arc<T>>,
}

impl<T> Registry<T> {
    pub fn new() -> Registry<T> {
        Registry {
            keys: vec![],
            values: vec![],
        }
    }

    pub fn get_value_by_id(&self, id: usize) -> Option<&Arc<T>> {
        self.values.get(id)
    }

    pub fn get_value<K: Into<ResourceLocation>>(&self, key: K) -> Option<&Arc<T>> {
        let key = key.into();

        for (id, vkey) in self.keys.iter().enumerate() {
            if key.eq(vkey) {
                return self.values.get(id);
            }
        }

        None
    }

    pub fn register<K: Into<ResourceLocation>>(&mut self, key: K, value: T) -> Arc<T> {
        let key = key.into();

        if self.keys.contains(&key) {
            panic!("Cannot re-insert using same key!");
        } else {
            let i = self.keys.len();
            let value = Arc::new(value);

            self.keys.insert(i, key);
            self.values.insert(i, value.clone());

            value
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

impl<T: ResourceLocatable> Registry<T> {
    pub fn register_locatable(&mut self, value: T) -> Arc<T> {
        self.register(value.resource_location().clone(), value)
    }
}

pub struct RegisterEvent<T> {
    pub registry: Registry<T>,
}

impl<T: 'static> Event for RegisterEvent<T> {}

pub struct Registries {
    pub blocks: Registry<Box<dyn Block>>,
    pub sounds: Registry<Sound>,
}

fn init_registry<T: 'static>(name: &str) -> Registry<T> {
    let mut event = RegisterEvent {
        registry: Registry::new()
    };

    // send event to all subscribers to add to registry
    dispatch_event!("main", &mut event);
    info!("registries -> Loaded {}: {}", name, event.registry.len());

   event.registry
}

impl Registries {
    pub fn new() -> Self {
        // load registries
        info!("registries -> Loading registries...");

        let blocks = init_registry("blocks");
        let sounds = init_registry("sounds");

        info!("registries -> Finished loading registries.");

        Self { blocks, sounds }
    }
}