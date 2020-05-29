use std::path::{Path, PathBuf};
use std::fs;
use java_props::Properties as JavaProps;

pub struct Properties {
    path: PathBuf,
    props: JavaProps,
}

impl Properties {
    pub fn load(path: PathBuf) -> Properties {
        let props = Self::load_props(&path);
        Properties {
            path,
            props,
        }
    }

    pub fn reload(&mut self) {
        self.props = Self::load_props(&self.path);
    }

    fn load_props(path: &Path) -> JavaProps {
        let contents = fs::read_to_string(path)
            .unwrap_or(String::new());

        JavaProps::from_str(&contents)
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.props.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn get_default(&mut self, key: &str, default: &str) -> String {
        let value = self.get(key);

        if value.is_some() {
            value.unwrap()
        } else {
            self.set(key, default);
            String::from(default)
        }
    }

    pub fn get_bool(&mut self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some(value) => {
                let value = value.to_lowercase();

                match value.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            },
            None => None,
        }
    }

    pub fn get_bool_default(&mut self, key: &str, default: bool) -> bool {
        let default_str = if default {
            "true"
        } else {
            "false"
        };

        match self.get_default(key, default_str).as_str() {
            "true" => true,
            "false" => false,
            _ => default,
        }
    }

    pub fn get_u32(&mut self, key: &str) -> Option<u32> {
        match self.get(key) {
            Some(value) => value.parse::<u32>().ok(),
            None => None,
        }
    }

    pub fn get_u32_default(&mut self, key: &str, default: u32) -> u32 {
        match self.get_u32(key) {
            Some(value) => value,
            None => {
                self.set_u32(key, default);
                default
            }
        }
    }

    pub fn get_i32(&mut self, key: &str) -> Option<i32> {
        match self.get(key) {
            Some(value) => value.parse::<i32>().ok(),
            None => None,
        }
    }

    pub fn get_i32_default(&mut self, key: &str, default: i32) -> i32 {
        match self.get_i32(key) {
            Some(value) => value,
            None => {
                self.set_i32(key, default);
                default
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.props.set(key, value);
        self.save();
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set(key, if value {
            "true"
        } else {
            "false"
        })
    }

    pub fn set_u32(&mut self, key: &str, value: u32) {
        self.set(key, &value.to_string());
    }

    pub fn set_i32(&mut self, key: &str, value: i32) {
        self.set(key, &value.to_string());
    }

    pub fn unset(&mut self, key: &str) {
        self.props.unset(key);
        self.save();
    }

    pub fn save(&self) {
        let contents = self.props.to_string();

        std::fs::write(&self.path, contents.as_bytes())
            .expect("Failed to write file.");
    }
}
