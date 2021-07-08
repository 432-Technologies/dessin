pub struct Text {
    pub text: String,
}

impl Text {
    pub const fn new() -> Text {
        Text {
            text: String::new(),
        }
    }
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}
