use super::{Fill, Stroke};
use algebra::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct CircleStyle {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl CircleStyle {
    pub const fn new() -> Self {
        CircleStyle {
            fill: None,
            stroke: None,
        }
    }
    pub const fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
    pub const fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
    pub style: CircleStyle,
}
