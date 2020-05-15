use super::{Color, ClickEvent, HoverEvent};
use crate::util::ToJsonValue;
use serde_json::Value;
use std::sync::{Arc, RwLock};

pub struct Style {
    pub parent: Option<Arc<RwLock<Style>>>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub color: Option<Color>,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            parent: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }
}

impl Style {
    pub fn is_empty(&self) -> bool {
        self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.color.is_none()
            && self.insertion.is_none()
            && self.click_event.is_none()
            && self.hover_event.is_none()
    }
}

impl ToJsonValue for Style {
    fn to_json(&self) -> Option<serde_json::Value> {
        if self.is_empty() {
            None
        } else {
            let mut json = json!({});

            if let Some(bold) = self.bold {
                json["bold"] = Value::Bool(bold);
            }

            if let Some(italic) = self.italic {
                json["italic"] = Value::Bool(italic);
            }

            if let Some(underlined) = self.underlined {
                json["underlined"] = Value::Bool(underlined);
            }

            if let Some(strikethrough) = self.strikethrough {
                json["strikethrough"] = Value::Bool(strikethrough);
            }

            if let Some(obfuscated) = self.obfuscated {
                json["obfuscated"] = Value::Bool(obfuscated);
            }

            if let Some(color) = self.color {
                json["color"] = color.to_json().unwrap();
            }

            if let Some(insertion) = &self.insertion {
                json["insertion"] = Value::String(insertion.clone());
            }

            if let Some(click_event) = &self.click_event {
                json["clickEvent"] = click_event.to_json().unwrap();
            }

            if let Some(hover_event) = &self.hover_event {
                json["hoverEvent"] = hover_event.to_json().unwrap();
            }

            Some(json)
        }
    }
}