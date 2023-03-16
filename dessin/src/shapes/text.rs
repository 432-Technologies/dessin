use na::{Point2, Scale2, Vector2};
use nalgebra::{self as na, Transform2};

use crate::{Shape, ShapeOp};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub local_transform: Transform2<f32>,
    pub align: TextAlign,
    pub font_weight: FontWeight,
}
impl Text {
    #[inline]
    pub fn text<T: ToString>(&mut self, text: T) -> &mut Self {
        self.text = text.to_string();
        self
    }
    #[inline]
    pub fn with_text<T: ToString>(mut self, text: T) -> Self {
        self.text(text);
        self
    }

    #[inline]
    pub fn align(&mut self, align: TextAlign) -> &mut Self {
        self.align = align;
        self
    }
    #[inline]
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align(align);
        self
    }

    #[inline]
    pub fn font_weight(&mut self, font_weight: FontWeight) -> &mut Self {
        self.font_weight = font_weight;
        self
    }
    #[inline]
    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight(font_weight);
        self
    }

    #[inline]
    pub fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.scale(Scale2::new(font_size, font_size))
    }
    #[inline]
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size(font_size);
        self
    }

    #[inline]
    pub fn get_font_size(&self) -> f32 {
        (self.local_transform * Vector2::new(0., 1.)).magnitude()
    }
}

impl From<Text> for Shape {
    fn from(v: Text) -> Self {
        Shape::Text(v)
    }
}

impl ShapeOp for Text {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}
