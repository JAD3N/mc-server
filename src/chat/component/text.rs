base_component!(TextComponent { text: String }, self => {
    self.text()
});

impl TextComponent {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }
}
