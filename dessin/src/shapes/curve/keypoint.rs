use super::Curve;
use nalgebra::{Point2, Vector2};

#[derive(Debug, Clone, PartialEq)]
pub enum Keypoint {
    Point(Point2<f32>),
    Bezier(Bezier),
    Curve(Curve),
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
