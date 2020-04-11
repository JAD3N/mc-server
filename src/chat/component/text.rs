use serde::ser::{Serializer, Serialize, SerializeStruct};

use super::{Style, Component};
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
        let style = Rc::new(RefCell::new(Style::new()));
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

impl Component for TextComponent {
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

impl Serialize for TextComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("TextComponent", 1)?;
        state.serialize_field("text", &self.text)?;
        state.end()
    }
}