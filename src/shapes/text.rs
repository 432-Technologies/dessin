use crate::{position::Rect, style::Style};

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

#[derive(Debug, Clone)]
pub struct Text {
    pub pos: Rect,
    pub text: String,
    pub style: Option<Style>,
    pub align: TextAlign,
    pub font_size: f32,
    pub font_weight: FontWeight,
}
crate::impl_pos!(Text);
crate::impl_style!(Text);
impl Text {
    pub const fn new(text: String) -> Self {
        Text {
            pos: Rect::new(),
            text,
            style: None,
            align: TextAlign::Left,
            font_size: 16.,
            font_weight: FontWeight::Regular,
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
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
}
