use algebr::Angle;
use shape::{Keypoint, Keypoints, Path};

use crate::{
    shape::{self, Style},
    Rect, Shape,
};

use super::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct ThickArc {
    pub(crate) pos: Rect,
    pub(crate) inner_radius: f32,
    pub(crate) outer_radius: f32,
    pub(crate) start_angle: Angle,
    pub(crate) end_angle: Angle,
    pub(crate) style: Option<Style>,
}
crate::impl_pos_at!(ThickArc);
crate::impl_pos_anchor!(ThickArc);
crate::impl_style!(ThickArc);
impl ThickArc {
    pub const fn new() -> Self {
        ThickArc {
            pos: Rect::new(),
            inner_radius: 0.0,
            outer_radius: 0.0,
            start_angle: Angle::radians(0.0),
            end_angle: Angle::radians(0.0),
            style: None,
        }
    }

    pub const fn with_inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius;
        self
    }

    pub const fn with_outer_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius = outer_radius;
        self
    }

    pub const fn with_start_angle(mut self, start_angle: Angle) -> Self {
        self.start_angle = start_angle;
        self
    }

    pub const fn with_end_angle(mut self, end_angle: Angle) -> Self {
        self.end_angle = end_angle;
        self
    }
}

impl Into<Shape> for ThickArc {
    fn into(self) -> Shape {
        let outer: Keypoints = Arc::new()
            .at(self.pos.pos)
            .with_anchor(self.pos.anchor)
            .with_radius(self.outer_radius)
            .with_start_angle(self.start_angle)
            .with_end_angle(self.end_angle)
            .into();

        let inner: Keypoints = Arc::new()
            .at(self.pos.pos)
            .with_anchor(self.pos.anchor)
            .with_radius(self.inner_radius)
            .with_start_angle(self.start_angle)
            .with_end_angle(self.end_angle)
            .into();

        let inner = inner.reversed();

        let p = if let Keypoint::Point(p) = inner.0.first().unwrap() {
            *p
        } else {
            unreachable!()
        };

        Path::new()
            .then_do(outer)
            .then(p)
            .then_do(inner)
            .close()
            .into()
    }
}
