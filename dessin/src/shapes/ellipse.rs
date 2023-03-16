use nalgebra::{Scale2, Transform2};

use crate::{Shape, ShapeOp};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ellipse {
    pub local_transform: Transform2<f32>,
}

impl Ellipse {
    #[inline]
    pub fn axis(&mut self, scale: Scale2<f32>) -> &mut Self {
        self.scale(scale);
        self
    }
    #[inline]
    pub fn with_axis(mut self, scale: Scale2<f32>) -> Self {
        self.axis(scale);
        self
    }

    #[inline]
    pub fn semi_major_axis(&mut self, value: f32) -> &mut Self {
        self.scale(Scale2::new(value, 1.));
        self
    }
    #[inline]
    pub fn with_semi_major_axis(mut self, value: f32) -> Self {
        self.semi_major_axis(value);
        self
    }

    #[inline]
    pub fn semi_minor_axis(&mut self, value: f32) -> &mut Self {
        self.scale(Scale2::new(1., value));
        self
    }
    #[inline]
    pub fn with_semi_minor_axis(mut self, value: f32) -> Self {
        self.semi_minor_axis(value);
        self
    }
}

impl From<Ellipse> for Shape {
    #[inline]
    fn from(v: Ellipse) -> Self {
        Shape::Ellipse(v)
    }
}

impl ShapeOp for Ellipse {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}
