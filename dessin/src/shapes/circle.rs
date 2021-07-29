use algebr::Vec2;

use crate::{position::Rect, style::Style, Shape, ShapeType};

#[derive(Debug, Clone)]
pub struct Circle {
    pub(crate) pos: Rect,
    pub(crate) radius: f32,
    pub(crate) style: Option<Style>,
}
crate::impl_pos!(Circle);
crate::impl_style!(Circle);
impl Circle {
    /// Default circle.
    pub const fn new() -> Circle {
        Circle {
            pos: Rect::new(),
            radius: 0.0,
            style: None,
        }
    }

    /// Create circle with radius.
    pub const fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl Into<Shape> for Circle {
    fn into(self) -> Shape {
        let size = Vec2::ones() * self.radius * 2.;

        Shape {
            pos: self.pos.with_size(size),
            style: self.style,
            shape_type: ShapeType::Circle {
                radius: self.radius,
            },
        }
    }
}
