use algebra::Vec2;

use super::{Fill, Stroke};

#[derive(Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct TextStyle {
    pub align: TextAlign,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl TextStyle {
    pub const fn new() -> Self {
        TextStyle {
            align: TextAlign::Left,
            fill: None,
            stroke: None,
        }
    }
    pub const fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

#[derive(Debug)]
pub struct Text {
    pub pos: Vec2,
    pub text: String,
    pub style: TextStyle,
}
impl Text {
    pub const fn new() -> Self {
        Text {
            pos: Vec2 { x: 0., y: 0. },
            text: String::new(),
            style: TextStyle::new(),
        }
    }
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}
