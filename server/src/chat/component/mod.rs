mod text;
mod translatable;

pub use text::*;
pub use translatable::*;

use crate::chat::Style;
use crate::util::ToJsonValue;

#[derive(Clone)]
pub enum ComponentContainer {
    Text(TextComponent),
    Translatable(TranslatableComponent),
}

impl ToJsonValue for ComponentContainer {
    fn to_json(&self) -> Option<serde_json::Value> {
        match self {
            ComponentContainer::Text(c) => c.to_json(),
            ComponentContainer::Translatable(c) => c.to_json(),
        }
    }
}

pub trait Component: ToJsonValue {
    fn append_extra_json(&self, json: &mut serde_json::Value) {
        let extra: Vec<serde_json::Value> = self.siblings()
            .iter()
            .map(|sibling| sibling.to_json().unwrap())
            .collect();

        if !extra.is_empty() {
            json["extra"] = json!(extra);
        }
    }

    fn append_style_json(&self, json: &mut serde_json::Value) {
        let style_json = self.style().to_json();

        if let Some(style_json) = style_json {
            json["style"] = style_json;
        }
    }

    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;

    fn siblings(&self) -> &Vec<ComponentContainer>;
    fn siblings_mut(&mut self) -> &mut Vec<ComponentContainer>;

    fn append<T: Into<ComponentContainer>>(&mut self, sibling: T) {
        self.siblings_mut().push(sibling.into());
    }

    fn contents(&self) -> &str { "" }
}