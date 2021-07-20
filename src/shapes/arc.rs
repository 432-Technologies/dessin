use algebra::Angle;

use crate::{position::Rect, style::Style};

#[derive(Debug, Clone)]
pub struct Arc {
    pub pos: Rect,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub start_angle: Angle,
    pub end_angle: Angle,
    pub style: Option<Style>,
}
macros::impl_pos!(Arc);
macros::impl_style!(Arc);
impl Arc {
    pub const fn new() -> Arc {
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
