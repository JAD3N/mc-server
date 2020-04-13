mod text;
pub use text::*;

use crate::{chat::Style, util::ToJsonValue};
use std::sync::{Arc, RwLock};

pub trait Component: mopa::Any + ToJsonValue {
    fn type_(&self) -> ComponentType;

    fn add_extra_json(&self, json: &mut serde_json::Value) {
        let extra: Vec<serde_json::Value> = self.siblings()
            .iter()
            .map(|sibling| sibling.to_json().unwrap())
            .collect();

        if !extra.is_empty() {
            json["extra"] = json!(extra);
        }
    }

    fn add_style_json(&self, json: &mut serde_json::Value) {
        let style_json = self.style()
            .read()
            .unwrap()
            .to_json();

        if let Some(style_json) = style_json {
            json["style"] = style_json;
        }
    }

    fn style(&self) -> &Arc<RwLock<Style>>;
    fn style_mut(&mut self) -> &mut Arc<RwLock<Style>>;

    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>>;

    fn append(&mut self, sibling: Box<dyn Component>) {
        // adjust child component style
        let mut style = sibling.style()
            .write()
            .unwrap();

        style.parent = Some(self.style().clone());
        drop(style);

        // push to siblings
        self.siblings_mut().push(sibling.into());
    }

    fn contents(&self) -> &str {
        ""
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ComponentType {
    Text,
}

mopafy!(Component);