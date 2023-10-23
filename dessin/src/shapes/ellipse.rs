use crate::{
    prelude::Circle,
    shapes::{Shape, ShapeOp},
};
use nalgebra::{Point2, Scale2, Transform2, Unit, Vector2};

use super::{BoundingBox, Curve, ShapeBoundingBox, UnParticular};

#[derive(Debug, Clone, PartialEq)]
pub struct EllipsePosition {
    pub center: Point2<f32>,

    pub semi_major_axis: f32,
    pub semi_minor_axis: f32,

    pub rotation: f32,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ellipse {
    pub local_transform: Transform2<f32>,
}

impl Ellipse {
    #[inline]
    pub fn axis<S: Into<Scale2<f32>>>(&mut self, scale: S) -> &mut Self {
        self.scale(scale);
        self
    }
    #[inline]
    pub fn with_axis<S: Into<Scale2<f32>>>(mut self, scale: S) -> Self {
        self.axis(scale);
        self
    }

    #[inline]
    pub fn semi_major_axis(&mut self, value: f32) -> &mut Self {
        self.scale(Scale2::new(2. * value, 1.));
        self
    }
    #[inline]
    pub fn with_semi_major_axis(mut self, value: f32) -> Self {
        self.semi_major_axis(value);
        self
    }

    #[inline]
    pub fn semi_minor_axis(&mut self, value: f32) -> &mut Self {
        self.scale(Scale2::new(1., 2. * value));
        self
    }
    #[inline]
    pub fn with_semi_minor_axis(mut self, value: f32) -> Self {
        self.semi_minor_axis(value);
        self
    }

    pub fn position(&self, parent_transform: &Transform2<f32>) -> EllipsePosition {
        let transform = self.global_transform(parent_transform);

        let center = transform * Point2::origin();

        let semi_major_axis = transform * Vector2::new(0.5, 0.);
        let semi_minor_axis = transform * Vector2::new(0., 0.5);

        let rotation = semi_major_axis.y.atan2(semi_major_axis.x);

        EllipsePosition {
            center,
            semi_major_axis: semi_major_axis.magnitude(),
            semi_minor_axis: semi_minor_axis.magnitude(),
            rotation,
        }
    }
}

impl From<Ellipse> for Curve {
    #[inline]
    fn from(e: Ellipse) -> Self {
        Circle::from(e).into()
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

impl ShapeBoundingBox for Ellipse {
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        BoundingBox::new(
            self.local_transform() * Point2::new(-0.5, 0.5),
            self.local_transform() * Point2::new(0.5, 0.5),
            self.local_transform() * Point2::new(0.5, -0.5),
            self.local_transform() * Point2::new(-0.5, -0.5),
        )
    }
}
