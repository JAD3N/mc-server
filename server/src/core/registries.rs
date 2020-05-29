use super::{ResourceRegistry, Sound};
use crate::network::protocol::Protocol;
use crate::world::level::Block;
use event_bus::Event;
use std::sync::Arc;
use std::collections::HashMap;
use std::hash::Hash;

pub struct RegisterEvent<T>(pub T);

impl<T: 'static> Event for RegisterEvent<T> {}

pub struct Registries {
    pub protocols: Arc<MappedRegistry<i32, Protocol>>,
    pub blocks: Arc<ResourceRegistry<Box<dyn Block>>>,
    pub sounds: Arc<ResourceRegistry<Sound>>,
}

fn init_resource_registry<T: 'static>(name: &str) -> Arc<ResourceRegistry<T>> {
    let mut event = RegisterEvent(ResourceRegistry::new());

    // send event to all subscribers to add to registry
    dispatch_event!("main", &mut event);

    // log completion
    info!("registries -> Loaded {}: {}", name, event.0.len());

    Arc::new(event.0)
}

fn init_protocols() -> Arc<MappedRegistry<i32, Protocol>> {
    let mut event = RegisterEvent(MappedRegistry::new());

    // send event to protocols
    dispatch_event!("main", &mut event);

    // log completion
    info!("registries -> Loaded protocols: {}", event.0.len());

    Arc::new(event.0)
}

impl Registries {
    pub fn new() -> Self {
        // load registries
        info!("registries -> Loading registries...");

        let protocols = init_protocols();
        let blocks = init_resource_registry("blocks");
        let sounds = init_resource_registry("sounds");

        info!("registries -> Finished loading registries.");

        Self { protocols, blocks, sounds }
    }
}

pub struct MappedRegistry<K, V> {
    map: HashMap<K, Arc<V>>,
}

impl<K: Eq + Hash, V> MappedRegistry<K, V> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get(&self, key: &K) -> Option<&Arc<V>> {
        self.map.get(key)
    }

    pub fn register(&mut self, key: K, value: V) {
        self.map.insert(key, Arc::new(value));
    }

    pub fn values(&self) -> Vec<&Arc<V>> {
        self.map.values().collect()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}