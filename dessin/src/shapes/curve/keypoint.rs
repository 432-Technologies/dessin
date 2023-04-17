use crate::{
    prelude::{BoundingBox, ShapeBoundingBox, UnParticular},
    shapes::ShapeOpWith,
};

use super::Curve;
use nalgebra::{Point, Point2, Transform2, Translation, Vector2};

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
            Keypoint::Bezier(b) => Keypoint::Bezier(b.transform(parent_transform)),
            Keypoint::Curve(c) => Keypoint::Curve(c.clone().with_transform(*parent_transform)),
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox<UnParticular>> {
        match self {
            Keypoint::Curve(c) => c.local_bounding_box(),
            Keypoint::Point(p) => Some(BoundingBox::at(*p).as_unparticular()),
            Keypoint::Bezier(b) => {
                let mut bb = BoundingBox::at(b.start_control)
                    .join(BoundingBox::at(b.end_control))
                    .join(BoundingBox::at(b.end));

                if let Some(start) = b.start {
                    bb = bb.join(BoundingBox::at(start))
                }

                Some(bb.as_unparticular())
            }
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
    pub start_control: Point2<f32>,

    pub end_control: Point2<f32>,
    pub end: Point2<f32>,
}
impl Bezier {
    pub fn new_with_start(
        start: Point2<f32>,
        start_control: Point2<f32>,
        end_control: Point2<f32>,
        end: Point2<f32>,
    ) -> Self {
        Bezier {
            start: Some(start),
            start_control,
            end_control,
            end,
        }
    }
    pub fn new(start_control: Point2<f32>, end_control: Point2<f32>, end: Point2<f32>) -> Self {
        Bezier {
            start: None,
            start_control,
            end_control,
            end,
        }
    }

    pub fn new_relative_with_start(
        start: Point2<f32>,
        start_control: Vector2<f32>,
        end_control: Vector2<f32>,
        end: Point2<f32>,
    ) -> Self {
        Bezier {
            start: Some(start),
            start_control: start + start_control,
            end_control: end + end_control,
            end,
        }
    }
    pub fn new_relative(
        start: &Point2<f32>,
        start_control: Vector2<f32>,
        end_control: Vector2<f32>,
        end: Point2<f32>,
    ) -> Self {
        Bezier {
            start: None,
            start_control: start + start_control,
            end_control: end + end_control,
            end,
        }
    }

    pub fn transform(&self, parent_transform: &Transform2<f32>) -> Self {
        Bezier {
            start: self.start.map(|v| parent_transform * v),
            start_control: parent_transform * self.start_control,
            end_control: parent_transform * self.end_control,
            end: parent_transform * self.end,
        }
    }
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

impl From<Curve> for Keypoint {
    #[inline]
    fn from(v: Curve) -> Self {
        Keypoint::Curve(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Translation2;

    const EPS: f32 = 0.000001;

    #[test]
    fn translate() {
        let b = Bezier {
            start: Some(Point2::new(0., 0.)),
            start_control: Point2::new(0., 1.),
            end_control: Point2::new(4., 5.),
            end: Point2::new(5., 5.),
        };

        let t: Transform2<f32> = nalgebra::convert(Translation2::new(-5., -5.));

        let new_b = b.transform(&t);

        assert!(
            (new_b.start.unwrap() - Point2::new(-5., -5.)).magnitude() < EPS,
            "left = {}, right = {}",
            new_b.start.unwrap(),
            Point2::new(-5., -5.),
        );
        assert!(
            (new_b.start_control - Point2::new(-5., -4.)).magnitude() < EPS,
            "left = {}, right = {}",
            new_b.start_control,
            Point2::new(-5., -4.),
        );
        assert!(
            (new_b.end_control - Point2::new(-1., 0.)).magnitude() < EPS,
            "left = {}, right = {}",
            new_b.end_control,
            Point2::new(-1., 0.),
        );
        assert!(
            (new_b.end - Point2::new(0., 0.)).magnitude() < EPS,
            "left = {}, right = {}",
            new_b.end,
            Point2::new(0., 0.),
        );
    }
}
