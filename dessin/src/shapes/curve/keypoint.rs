use crate::shapes::ShapeOpWith;

use super::Curve;
use nalgebra::{Point, Point2, Transform2, Vector2};

#[derive(Debug, Clone, PartialEq)]
pub enum Keypoint {
    Point(Point2<f32>),
    Bezier(Bezier),
    Curve(Curve),
}
impl Keypoint {
    pub fn transform(&self, parent_transform: &Transform2<f32>) -> Self {
        match self {
            Keypoint::Point(p) => Keypoint::Point(parent_transform * p),
            Keypoint::Bezier(Bezier {
                start,
                start_control,
                end_control,
                end,
            }) => Keypoint::Bezier(Bezier {
                start: start.map(|v| parent_transform * v),
                start_control: parent_transform * start_control,
                end_control: parent_transform * end_control,
                end: parent_transform * end,
            }),
            Keypoint::Curve(c) => Keypoint::Curve(c.clone().with_transform(*parent_transform)),
        }
    }
}

impl Default for Keypoint {
    fn default() -> Self {
        Keypoint::Point(Point2::origin())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bezier {
    pub start: Option<Point2<f32>>,
    pub start_control: Vector2<f32>,

    pub end_control: Vector2<f32>,
    pub end: Vector2<f32>,
}

impl From<Bezier> for Keypoint {
    #[inline]
    fn from(v: Bezier) -> Self {
        Keypoint::Bezier(v)
    }
}

impl From<Point2<f32>> for Keypoint {
    #[inline]
    fn from(v: Point2<f32>) -> Self {
        Keypoint::Point(v)
    }
}
