use super::{Component, BaseComponent};

pub struct TextComponent {
    pub text: String,
}

impl Into<Component> for TextComponent {
    fn into(self) -> Component {
        Component::Text(self)
    }
}

impl BaseComponent for TextComponent {

}