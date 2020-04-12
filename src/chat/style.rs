use super::{Color, ClickEvent, HoverEvent};
use crate::util::JsonValue;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq)]
pub enum StyleTrait {
    Bold,
    Italic,
    Strikethrough,
    Underline,
    Obfuscated,
    ClickEvent,
    HoverEvent,
}

traitable!(StyleTrait, Style {
    parent: Option<Rc<RefCell<Style>>>,
    color: Option<Color>,
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
    insertion: Option<String>,
});

impl Style {
    pub fn new() -> Style {
        Style {
            parent: None,
            traits: vec![],
            color: None,
            click_event: None,
            hover_event: None,
            insertion: None,
        }
    }

    pub fn set_parent(&mut self, parent: Option<Rc<RefCell<Style>>>) {
        self.parent = parent;
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn set_click_event(&mut self, click_event: Option<ClickEvent>) {
        self.click_event = click_event;
    }

    pub fn hover_event(&self) -> Option<&HoverEvent> {
        self.hover_event.as_ref()
    }

    pub fn set_hover_event(&mut self, hover_event: Option<HoverEvent>) {
        self.hover_event = hover_event;
    }

    pub fn insertion(&self) -> Option<&String> {
        self.insertion.as_ref()
    }

    pub fn set_insertion(&mut self, insertion: Option<String>) {
        self.insertion = insertion;
    }

    pub fn reset(&mut self) {
        self.traits = vec![];
        self.color = None;
        self.click_event = None;
        self.hover_event = None;
        self.insertion = None;
    }

    pub fn is_empty(&self) -> bool {
        use StyleTrait::*;

        if self.color.is_some()
            || self.click_event.is_some()
            || self.hover_event.is_some()
            || self.insertion.is_some() {
            false
        } else {
            self.has(Bold)
                || self.has(Italic)
                || self.has(Strikethrough)
                || self.has(Underline)
                || self.has(Obfuscated)
                || self.has(ClickEvent)
                || self.has(HoverEvent)
        }
    }
}

impl JsonValue for Style {
    fn to_json(&self) -> Option<serde_json::Value> {
        if self.is_empty() {
            None
        } else {
            

            None
        }
    }
}