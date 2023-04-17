use crate::{
    prelude::{Shape, ShapeOp},
    shapes::{Curve, Keypoint},
};
use nalgebra::{Point2, Transform2};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Line {
    pub local_transform: Transform2<f32>,
    from: Point2<f32>,
    to: Point2<f32>,
}

impl Line {
    #[inline]
    pub fn from(&mut self, from: Point2<f32>) -> &mut Self {
        self.from = from;
        self
    }
    #[inline]
    pub fn with_from(mut self, from: Point2<f32>) -> Self {
        self.from(from);
        self
    }

    #[inline]
    pub fn to(&mut self, to: Point2<f32>) -> &mut Self {
        self.to = to;
        self
    }
    #[inline]
    pub fn with_to(mut self, to: Point2<f32>) -> Self {
        self.to(to);
        self
    }
}

impl ShapeOp for Line {
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

impl From<Line> for Shape {
    #[inline]
    fn from(l: Line) -> Self {
        Shape::Curve(Curve::from(l))
    }
}

impl From<Line> for Curve {
    #[inline]
    fn from(
        Line {
            local_transform,
            from,
            to,
        }: Line,
    ) -> Self {
        Curve {
            local_transform,
            closed: false,
            keypoints: vec![Keypoint::Point(from), Keypoint::Point(to)],
        }
    }
}
