use super::{Fill, Stroke};
use algebra::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct LineStyle {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl LineStyle {
    pub const fn new() -> Self {
        LineStyle {
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
pub struct Line {
    pub from: Vec2,
    pub to: Vec2,
    pub style: LineStyle,
}
