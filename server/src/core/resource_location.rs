use std::sync::Arc;
use std::{fmt, error};
use regex::Regex;

lazy_static! {
    static ref IS_NAMESPACE: Regex = Regex::new("^[a-z0-9_.-]+$").unwrap();
    static ref IS_PATH: Regex = Regex::new("^[a-z0-9/._-]+$").unwrap();
}

#[derive(Debug, Clone)]
pub struct ResourceLocationError(String);

impl error::Error for ResourceLocationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ResourceLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub trait ResourceLocatable {
    fn resource_location(&self) -> &ResourceLocation;
}

#[derive(Clone, Debug)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

impl PartialEq for ResourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.path == other.path
    }
}

impl ResourceLocation {
    pub fn new(namespace: &str, path: &str) -> ResourceLocation {
        ResourceLocation {
            namespace: String::from(namespace),
            path: String::from(path),
        }
    }

    pub fn parse(s: &str) -> Result<ResourceLocation, ResourceLocationError> {
        let (namespace, path) = if s.contains(":") {
            let namespace;
            let path;

            if s.starts_with(":") {
                namespace = String::from("minecraft");
                path = String::from(&s[1..]);
            } else {
                let i = s.find(":").unwrap();

                namespace = String::from(&s[..i]);
                path = String::from(&s[(i + 1)..]);
            }

            (namespace, path)
        } else {
            let namespace = String::from("minecraft");
            let path = String::from(s);

            (namespace, path)
        };

        if !Self::is_valid_namespace(&namespace) {
            Err(ResourceLocationError(String::from(r"Namespace must only contain: [a-z0-9_.-]")))
        } else if !Self::is_valid_path(&path) {
            Err(ResourceLocationError(String::from(r"Path must only contain: [a-z0-9/._-]")))
        } else {
            Ok(ResourceLocation { namespace, path })
        }
    }

    pub fn is_valid_namespace(s: &str) -> bool {
        IS_NAMESPACE.is_match(s)
    }

    pub fn is_valid_path(s: &str) -> bool {
        IS_PATH.is_match(s)
    }
}

impl Into<ResourceLocation> for &str {
    fn into(self) -> ResourceLocation {
        ResourceLocation::parse(self).unwrap()
    }
}

impl Into<ResourceLocation> for String {
    fn into(self) -> ResourceLocation {
        ResourceLocation::parse(&self).unwrap()
    }
}

#[derive(Debug)]
pub struct ResourceRegistry<T> {
    pub keys: Vec<ResourceLocation>,
    pub values: Vec<Arc<T>>,
}

impl<T> ResourceRegistry<T> {
    pub fn new() -> ResourceRegistry<T> {
        Self {
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

impl<T: ResourceLocatable> ResourceRegistry<T> {
    pub fn register_locatable(&mut self, value: T) -> Arc<T> {
        self.register(value.resource_location().clone(), value)
    }
}