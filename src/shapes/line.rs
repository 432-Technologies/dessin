use super::Stroke;
use algebra::Vec2;

#[derive(Debug)]
pub struct LineStyle {
    stroke: Option<Stroke>,
}
impl LineStyle {
    pub const fn new() -> Self {
        LineStyle { stroke: None }
    }
}

#[derive(Debug)]
pub struct Line {
    pub from: Vec2,
    pub to: Vec2,
    pub style: LineStyle,
}
