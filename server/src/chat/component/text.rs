use crate::chat::Style;
use crate::util::ToJsonValue;
use super::{Component, ComponentContainer};

#[derive(Clone)]
pub struct TextComponent {
    style: Style,
    siblings: Vec<ComponentContainer>,
    text: String,
}

impl ToJsonValue for TextComponent {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({
            "text": &self.text,
        });

        self.append_extra_json(&mut json);
        self.append_style_json(&mut json);

        Some(json)
    }
}

impl Component for TextComponent {
    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn siblings(&self) -> &Vec<ComponentContainer> {
        &self.siblings
    }

    fn siblings_mut(&mut self) -> &mut Vec<ComponentContainer> {
        &mut self.siblings
    }
}

impl Into<ComponentContainer> for TextComponent {
    fn into(self) -> ComponentContainer {
        ComponentContainer::Text(self)
    }
}

impl TextComponent {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self::new_with_style(text, Style::default())
    }

    pub fn new_with_style<T: Into<String>>(text: T, style: Style) -> Self {
        Self {
            siblings: vec![],
            style,
            text: text.into(),
        }
    }
}