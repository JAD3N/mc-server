use super::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum StyleTrait {
    Color,
    Bold,
    Italic,
    Strikethrough,
    Underline,
    Obfuscated,
}

traitable!(StyleTrait, Style {
    color: Option<Color>,
});

impl Style {
    pub fn new() -> Style {
        Style {
            traits: vec![],
            color: None,
        }
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        if color.is_some() {
            self.add(StyleTrait::Color);
            self.color = color;
        } else {
            self.remove(StyleTrait::Color);
            self.color = None;
        }
    }

    pub fn get_color(&self) -> Option<Color> {
        self.color
    }

    pub fn reset(&mut self) {
        self.traits = vec![];
        self.color = None;
    }

    pub fn is_empty(&self) -> bool {
        use StyleTrait::*;

        self.has(Color)
            || self.has(Bold)
            || self.has(Italic)
            || self.has(Strikethrough)
            || self.has(Underline)
            || self.has(Obfuscated)
    }
}
