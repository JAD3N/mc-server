use regex::Regex;
use std::fmt;
use crate::util::ToJsonValue;

lazy_static! {
    static ref IS_COLOR: Regex = Regex::new(&format!("(?i){}[0-9A-FK-OR]", Color::COLOR_CHAR)).unwrap();
}

#[derive(Clone, Copy)]
pub struct Color {
    code: char,
    name: &'static str,
}

impl Color {
    pub const COLOR_CHAR: char = '\u{00a7}';

    pub const BLACK: Color = Color { code: '0', name: "black" };
    pub const DARK_BLUE: Color = Color { code: '1', name: "dark_blue" };
    pub const DARK_GREEN: Color = Color { code: '2', name: "dark_green" };
    pub const DARK_AQUA: Color = Color { code: '3', name: "dark_aqua" };
    pub const DARK_RED: Color = Color { code: '4', name: "dark_red" };
    pub const DARK_PURPLE: Color = Color { code: '5', name: "dark_purple" };
    pub const GOLD: Color = Color { code: '6', name: "gold" };
    pub const GRAY: Color = Color { code: '7', name: "gray" };
    pub const DARK_GRAY: Color = Color { code: '8', name: "dark_gray" };
    pub const BLUE: Color = Color { code: '9', name: "blue" };
    pub const GREEN: Color = Color { code: 'a', name: "green" };
    pub const AQUA: Color = Color { code: 'b', name: "aqua" };
    pub const RED: Color = Color { code: 'c', name: "red" };
    pub const LIGHT_PURPLE: Color = Color { code: 'd', name: "light_purple" };
    pub const YELLOW: Color = Color { code: 'e', name: "yellow" };
    pub const WHITE: Color = Color { code: 'f', name: "white" };

    pub const OBFUSCATED: Color = Color { code: 'k', name: "obfuscated" };
    pub const BOLD: Color = Color { code: 'l', name: "bold" };
    pub const STRIKETHROUGH: Color = Color { code: 'm', name: "strikethrough" };
    pub const UNDERLINE: Color = Color { code: 'n', name: "underline" };
    pub const ITALIC: Color = Color { code: 'o', name: "italic" };
    pub const RESET: Color = Color { code: 'r', name: "reset" };

    pub fn code(&self) -> char {
        self.code
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn all() -> Vec<&'static Color> {
        vec![
            &Color::BLACK,
            &Color::DARK_BLUE,
            &Color::DARK_GREEN,
            &Color::DARK_AQUA,
            &Color::DARK_RED,
            &Color::DARK_PURPLE,
            &Color::GOLD,
            &Color::GRAY,
            &Color::DARK_GRAY,
            &Color::BLUE,
            &Color::GREEN,
            &Color::AQUA,
            &Color::RED,
            &Color::LIGHT_PURPLE,
            &Color::YELLOW,
            &Color::WHITE,

            &Color::OBFUSCATED,
            &Color::BOLD,
            &Color::STRIKETHROUGH,
            &Color::UNDERLINE,
            &Color::ITALIC,
            &Color::RESET,
        ]
    }

    pub fn from_code(code: char) -> Option<&'static Color> {
        for chat_color in Self::all() {
            if chat_color.code == code {
                return Some(chat_color);
            }
        }

        None
    }

    pub fn from_str(name: &str) -> Option<&'static Color> {
        let name = String::from(name).to_lowercase();

        for chat_color in Self::all() {
            if chat_color.name == name {
                return Some(chat_color);
            }
        }

        None
    }
}

impl ToJsonValue for Color {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(json!(self.name))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialEq<String> for Color {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Color::COLOR_CHAR, self.code)
    }
}