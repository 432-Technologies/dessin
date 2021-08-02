use algebr::{Angle, Vec2};

use crate::{
    shape::{self, Style},
    Rect, Shape,
};

pub type ThickArc = Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Arc {
    pub(crate) pos: Rect,
    pub(crate) inner_radius: f32,
    pub(crate) outer_radius: f32,
    pub(crate) start_angle: Angle,
    pub(crate) end_angle: Angle,
    pub(crate) style: Option<Style>,
}
crate::impl_pos_at!(Arc);
crate::impl_pos_anchor!(Arc);
crate::impl_style!(Arc);
impl Arc {
    pub const fn new() -> Self {
        Arc {
            pos: Rect::new(),
            inner_radius: 0.0,
            outer_radius: 0.0,
            start_angle: Angle::radians(0.0),
            end_angle: Angle::radians(0.0),
            style: None,
        }
    }

    pub const fn with_inner_radius(mut self, inner_radius: f32) -> Arc {
        self.inner_radius = inner_radius;
        self
    }

    pub const fn with_outer_radius(mut self, outer_radius: f32) -> Arc {
        self.outer_radius = outer_radius;
        self
    }

    pub const fn with_start_angle(mut self, start_angle: Angle) -> Arc {
        self.start_angle = start_angle;
        self
    }

    pub const fn with_end_angle(mut self, end_angle: Angle) -> Arc {
        self.end_angle = end_angle;
        self
    }
}

impl Into<Shape> for Arc {
    fn into(self) -> Shape {
        let size = Vec2::ones() * self.outer_radius * 2.;

        vec![shape::Arc::new()].into()
    }
}
