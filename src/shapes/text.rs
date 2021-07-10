use algebra::Vec2;

use super::{Fill, Stroke};

#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub align: TextAlign,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub font_size: f32,
    pub font_weight: FontWeight,
}
impl TextStyle {
    pub const fn new() -> Self {
        TextStyle {
            align: TextAlign::Left,
            fill: None,
            stroke: None,
            font_size: 16.,
            font_weight: FontWeight::Regular,
        }
    }
    pub const fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
    pub const fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
    pub const fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }
    pub const fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
    pub const fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
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
