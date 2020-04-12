mod text;
pub use text::*;

use crate::{chat::Style, util::JsonValue};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Component: mopa::Any + JsonValue {
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
        // let extra: Vec<serde_json::Value> = self.siblings()
        //     .iter()
        //     .map(|sibling| sibling.to_json())
        //     .collect();

        // if !extra.is_empty() {
        //     json["extra"] = json!(extra);
        // }
    }

    fn style(&self) -> &Rc<RefCell<Style>>;
    fn style_mut(&mut self) -> &mut Rc<RefCell<Style>>;

    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>>;

    fn append(&mut self, sibling: Box<dyn Component>) {
        // adjust child component style
        let mut style = sibling.style().borrow_mut();
        style.set_parent(Some(self.style().clone()));
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