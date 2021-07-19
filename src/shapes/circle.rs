use algebra::Vec2;

use crate::{position::Rect, style::Style};

#[derive(Debug, Clone)]
pub struct Circle {
    pub pos: Rect,
    pub radius: f32,
    pub style: Option<Style>,
}
macros::impl_pos!(Circle);
macros::impl_style!(Circle);
impl Circle {
    /// Create circle with radius.
    pub const fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
