use super::ResourceLocation;
use super::sound::Sound;
use crate::world::level::Block;
use std::sync::{Arc, RwLock};

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
}

pub trait Registrable {
    fn register();
}

lazy_static! {
    pub static ref BLOCKS: RwLock<Registry<Box<dyn Block>>> = RwLock::new(Registry::new());
    pub static ref SOUNDS: RwLock<Registry<Sound>> = RwLock::new(Registry::new());
}

#[macro_export]
macro_rules! register {
    ($registry:ident, $key:expr, $value:expr) => {
        $registry
            .write()
            .unwrap()
            .register($key, $value)
    };
}

#[macro_export]
macro_rules! registry {
    ($registry:ident) => {
        crate::core::registry::$registry;
    };
}