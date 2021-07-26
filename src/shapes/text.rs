use algebr::Vec2;

use crate::{position::Rect, style::Style, Shape, ShapeType};

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
    pub(crate) pos: Rect,
    pub(crate) text: String,
    pub(crate) style: Option<Style>,
    pub(crate) align: TextAlign,
    pub(crate) font_size: f32,
    pub(crate) font_weight: FontWeight,
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

impl Into<Shape> for Text {
    fn into(self) -> Shape {
        Shape {
            pos: self.pos.with_size(Vec2::ones()),
            style: self.style,
            shape_type: ShapeType::Text {
                text: self.text,
                align: self.align,
                font_size: self.font_size,
                font_weight: self.font_weight,
            },
        }
    }
}
