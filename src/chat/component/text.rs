use super::{Style, Component, ComponentType};
use crate::util::JsonValue;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TextComponent {
    style: Rc<RefCell<Style>>,
    siblings: Vec<Box<dyn Component>>,
    text: String,
}

impl TextComponent {
    pub fn new() -> TextComponent {
        Self::from_str("")
    }

    pub fn from_str(s: &str) -> TextComponent {
        let style = Rc::new(RefCell::new(Style::default()));
        let siblings = vec![];
        let text = String::from(s);

        TextComponent { style, siblings, text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }
}

impl JsonValue for TextComponent {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({
            "text": &self.text,
        });

        self.add_extra_json(&mut json);
        self.add_style_json(&mut json);

        Some(json)
    }
}

impl Component for TextComponent {
    fn type_(&self) -> ComponentType {
        ComponentType::Text
    }

    fn style(&self) -> &Rc<RefCell<Style>> {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Rc<RefCell<Style>> {
        &mut self.style
    }

    fn siblings(&self) -> &Vec<Box<dyn Component>> {
        &self.siblings
    }

    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.siblings
    }

    fn contents(&self) -> &str {
        &self.text
    }
}