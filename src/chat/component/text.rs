use super::ComponentType;

component!(TextComponent {
    text: String,
});

impl Into<ComponentType> for TextComponent {
    fn into(self) -> ComponentType {
        ComponentType::Text(self)
    }
}