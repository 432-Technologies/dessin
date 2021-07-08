use super::{Shape, Style};
use algebra::Vec2;

#[derive(Debug)]
pub struct Line {
    pub from: Vec2,
    pub to: Vec2,
    pub style: Style,
}
impl Shape for Line {}