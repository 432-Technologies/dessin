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
        match self.quarter {
            Quarter::TopRight => Keypoints(vec![
                Keypoint::Bezier(vec2(1., 0.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(1., A) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(A, 0.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(0., 1.) * self.radius + self.pos.pos),
            ]),
            Quarter::TopLeft => Keypoints(vec![
                Keypoint::Bezier(vec2(0., 1.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-A, 0.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-1., A) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-1., 0.) * self.radius + self.pos.pos),
            ]),
            Quarter::BottomLeft => Keypoints(vec![
                Keypoint::Bezier(vec2(0., -1.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-A, 0.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-1., -A) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(-1., 0.) * self.radius + self.pos.pos),
            ]),
            Quarter::BottomRight => Keypoints(vec![
                Keypoint::Bezier(vec2(0., -1.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(A, 0.) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(1., -A) * self.radius + self.pos.pos),
                Keypoint::Bezier(vec2(1., 0.) * self.radius + self.pos.pos),
            ]),
        }
    }
}

impl Into<Shape> for QuarterCircle {
    fn into(self) -> Shape {
        Path::new().then(Into::<Keypoints>::into(self)).into()
    }
}
