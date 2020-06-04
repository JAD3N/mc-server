use crate::chat::Style;
use crate::util::ToJsonValue;
use super::{Component, ComponentContainer};

#[derive(Clone)]
pub struct TranslatableComponent {
    style: Style,
    siblings: Vec<ComponentContainer>,
    key: String,
    args: Vec<serde_json::Value>,
}

impl ToJsonValue for TranslatableComponent {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({
            "key": &self.key,
            "args": &self.args,
        });

        self.append_extra_json(&mut json);
        self.append_style_json(&mut json);

        Some(json)
    }
}

impl Component for TranslatableComponent {
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

impl Into<ComponentContainer> for TranslatableComponent {
    fn into(self) -> ComponentContainer {
        ComponentContainer::Translatable(self)
    }
}

impl TranslatableComponent {
    pub fn new<T: Into<String>>(key: T, args: Vec<serde_json::Value>) -> Self {
        Self::new_with_style(key, args, Style::default())
    }

    pub fn new_with_style<T: Into<String>>(key: T, args: Vec<serde_json::Value>, style: Style) -> Self {
        Self {
            siblings: vec![],
            style,
            key: key.into(),
            args,
        }
    }

    pub fn append_arg<T: Into<serde_json::Value>>(&mut self, arg: T) {
        self.args.push(arg.into());
    }
}