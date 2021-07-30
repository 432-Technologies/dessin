use algebr::{Angle, Vec2};

use crate::{position::Rect, style::Style, Shape, ShapeType};

#[derive(Debug, Clone)]
pub struct Arc {
    pub(crate) pos: Rect,
    pub(crate) radius: f32,
    pub(crate) start_angle: Angle,
    pub(crate) end_angle: Angle,
    pub(crate) style: Option<Style>,
}
crate::impl_pos!(Arc);
crate::impl_style!(Arc);
impl Arc {
    pub const fn new() -> Arc {
        Arc {
            pos: Rect::new(),
            radius: 0.0,
            start_angle: Angle::radians(0.0),
            end_angle: Angle::radians(0.0),
            style: None,
        }
    }

    pub const fn with_radius(mut self, radius: f32) -> Arc {
        self.radius = radius;
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
        let size = Vec2::ones() * self.radius * 2.;

        Shape {
            pos: self.pos.with_size(size),
            style: self.style,
            shape_type: ShapeType::Arc {
                radius: self.radius,
                start_angle: self.start_angle,
                end_angle: self.end_angle,
            },
        }
    }
}
