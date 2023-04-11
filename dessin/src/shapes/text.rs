pub mod font;

use super::{BoundingBox, Curve, CurvePosition, ShapeBoundingBox, UnParticular};
use crate::shapes::{Shape, ShapeOp};
use na::{Point2, Vector2};
use nalgebra::{self as na, Transform2};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

pub struct TextPosition<'a> {
    pub text: &'a str,
    pub align: TextAlign,
    pub font_weight: FontWeight,
    pub on_curve: Option<CurvePosition>,
    pub font_size: f32,
    pub reference_start: Point2<f32>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub local_transform: Transform2<f32>,
    pub align: TextAlign,
    pub font_weight: FontWeight,
    pub on_curve: Option<Curve>,
    pub font_size: f32,
    pub font: Option<usize>,
}
impl Text {
    pub fn font(&mut self, font: font::FontGroup<font::Font>) -> &mut Self {
        self.font = Some(font::add_font(font));
        self
    }

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
        self.font_size = font_size;
        self
    }
    #[inline]
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size(font_size);
        self
    }

    pub fn on_curve(&mut self, curve: Curve) -> &mut Self {
        self.on_curve = Some(curve);
        self
    }

    pub fn position(&self, parent_transform: &Transform2<f32>) -> TextPosition {
        let transform = self.global_transform(parent_transform);

        let font_size = self.font_size * (transform * Vector2::new(0., 1.)).magnitude();
        let bottom_left = transform * Point2::new(0., font_size / 2.);

        TextPosition {
            text: &self.text,
            align: self.align,
            font_weight: self.font_weight,
            on_curve: self.on_curve.as_ref().map(|v| v.position(&transform)),
            font_size,
            reference_start: bottom_left,
        }
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

impl ShapeBoundingBox for Text {
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>> {
        todo!()
    }
}
