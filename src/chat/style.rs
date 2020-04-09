use super::{Color, ClickEvent, HoverEvent};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq)]
pub enum StyleTrait {
    Color,
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
});

impl Style {
    pub fn new() -> Style {
        Style {
            parent: None,
            traits: vec![],
            color: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub fn set_parent(&mut self, parent: Option<Rc<RefCell<Style>>>) {
        self.parent = parent;
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;

        if color.is_some() {
            self.add(StyleTrait::Color);
        } else {
            self.remove(StyleTrait::Color);
        }
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn set_click_event(&mut self, click_event: Option<ClickEvent>) {
        self.click_event = click_event;

        if self.click_event.is_some() {
            self.add(StyleTrait::ClickEvent);
        } else {
            self.remove(StyleTrait::ClickEvent);
        }
    }

    pub fn click_event(&self) -> Option<&ClickEvent> {
        self.click_event.as_ref()
    }

    pub fn set_hover_event(&mut self, hover_event: Option<HoverEvent>) {
        self.hover_event = hover_event;

        if self.hover_event.is_some() {
            self.add(StyleTrait::HoverEvent);
        } else {
            self.remove(StyleTrait::HoverEvent);
        }
    }

    pub fn hover_event(&self) -> Option<&HoverEvent> {
        self.hover_event.as_ref()
    }

    pub fn reset(&mut self) {
        self.traits = vec![];
        self.color = None;
        self.click_event = None;
        self.hover_event = None;
    }

    pub fn is_empty(&self) -> bool {
        use StyleTrait::*;

        self.has(Color)
            || self.has(Bold)
            || self.has(Italic)
            || self.has(Strikethrough)
            || self.has(Underline)
            || self.has(Obfuscated)
            || self.has(ClickEvent)
            || self.has(HoverEvent)
    }
}
