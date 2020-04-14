use super::ResourceLocation;
use crate::world::level::Block;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Registry<T> {
    pub keys: Vec<ResourceLocation>,
    pub values: Vec<T>,
}

impl<T> Registry<T> {
    pub fn new() -> Registry<T> {
        Registry {
            keys: vec![],
            values: vec![],
        }
    }

    pub fn get_value_by_id(&self, id: usize) -> Option<&T> {
        self.values.get(id)
    }

    pub fn get_value<K: Into<ResourceLocation>>(&self, key: K) -> Option<&T> {
        let key = key.into();

        for (id, vkey) in self.keys.iter().enumerate() {
            if key.eq(vkey) {
                return self.values.get(id);
            }
        }

        None
    }

    pub fn map<K: Into<ResourceLocation>>(&mut self, key: K, value: T) {
        let key = key.into();

        if self.keys.contains(&key) {
            panic!("Cannot re-insert using same key!");
        } else {
            let i = self.keys.len();

            self.keys.insert(i, key);
            self.values.insert(i, value);
        }
    }
}

pub trait Registerable {
    fn register();
}

lazy_static! {
    pub static ref BLOCKS: RwLock<Registry<Box<dyn Block>>> = RwLock::new(Registry::new());
}

#[macro_export]
macro_rules! register {
    ($registry:ident, $key:expr, $value:expr) => {
        crate::core::registry::$registry
            .write()
            .unwrap()
            .map($key, $value)
    };
}

#[macro_export]
macro_rules! registry {
    ($registry:ident) => {
        crate::core::registry::$registry;
    };
}