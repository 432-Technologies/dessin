use crate::{
    prelude::*,
    shapes::{Curve, Keypoint},
};
use nalgebra::{Point2, Transform2};

/// Straight line between 2 points, from and to
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Line {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,

    /// Starting point
    #[shape(into)]
    from: Point2<f32>,

    /// Ending point
    #[shape(into)]
    to: Point2<f32>,
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
