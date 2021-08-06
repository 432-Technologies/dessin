use algebr::vec2;

use crate::{
    shape::Style,
    shapes::path::{Keypoint, Keypoints, Path},
    Rect, Shape,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quarter {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuarterCircle {
    pub(crate) pos: Rect,
    pub(crate) radius: f32,
    pub(crate) style: Option<Style>,
    pub(crate) quarter: Quarter,
}
crate::impl_pos_at!(QuarterCircle);
crate::impl_pos_anchor!(QuarterCircle);
crate::impl_style!(QuarterCircle);

impl QuarterCircle {
    pub fn new(quarter: Quarter) -> Self {
        QuarterCircle {
            pos: Rect::new(),
            radius: 0.0,
            style: None,
            quarter,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl Into<Keypoints> for QuarterCircle {
    fn into(self) -> Keypoints {
        const A: f32 = 0.551915024494;
        Keypoints(
            IntoIterator::into_iter(match self.quarter {
                Quarter::TopRight => [
                    Keypoint::Point(vec2(1., 0.)),
                    Keypoint::BezierCubic {
                        to: vec2(0., 1.),
                        control_from: vec2(1., A),
                        control_to: vec2(A, 1.),
                    },
                ],
                Quarter::TopLeft => [
                    Keypoint::Point(vec2(0., 1.)),
                    Keypoint::BezierCubic {
                        to: vec2(-1., 0.),
                        control_from: vec2(-A, 1.),
                        control_to: vec2(-1., A),
                    },
                ],
                Quarter::BottomLeft => [
                    Keypoint::Point(vec2(-1., 0.)),
                    Keypoint::BezierCubic {
                        to: vec2(0., -1.),
                        control_from: vec2(-1., -A),
                        control_to: vec2(-A, -1.),
                    },
                ],
                Quarter::BottomRight => [
                    Keypoint::Point(vec2(0., -1.)),
                    Keypoint::BezierCubic {
                        to: vec2(1., 0.),
                        control_from: vec2(A, -1.),
                        control_to: vec2(1., -A),
                    },
                ],
            })
            .map(|v| v * self.radius + self.pos.pos)
            .collect(),
        )
    }
}

impl Into<Shape> for QuarterCircle {
    fn into(self) -> Shape {
        Path::new().then_do(Into::<Keypoints>::into(self)).into()
    }
}
