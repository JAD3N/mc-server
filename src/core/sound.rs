use super::ResourceLocation;
use super::registry::{self, Registrable};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Sound {
    location: ResourceLocation,
}

impl Sound {
    pub fn new<T: Into<ResourceLocation>>(location: T) -> Self {
        Self { location: location.into() }
    }

    pub fn location(&self) -> &ResourceLocation {
        &self.location
    }
}

lazy_static! {
    pub static ref MAP: RwLock<HashMap<&'static str, Arc<Sound>>> = RwLock::new(HashMap::new());
}

macro_rules! sound {
    ($key:expr) => {
        {
            use crate::core::sound::MAP;

            let sounds = MAP.read().unwrap();

            match sounds.get($key) {
                Some(sound) => Some(sound.clone()),
                None => None,
            }
        }
    };
}

impl Registrable for Sound {
    fn register() {
        let mut sounds_registry = registry::SOUNDS.write().unwrap();
        let mut sounds_map = MAP.write().unwrap();

        include!(concat!(env!("OUT_DIR"), "/sounds.rs"));

        debug!("Registered {} sounds.", sounds_map.len());
    }
}