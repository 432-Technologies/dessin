use super::{Fill, Stroke};
use algebra::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct ArcStyle {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl ArcStyle {
    pub const fn new() -> Self {
        ArcStyle {
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

#[derive(Debug)]
pub struct Arc {
    pub pos: Vec2,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub from_deg: f32,
    pub to_deg: f32,
    pub style: ArcStyle,
}
