pub mod arc;
pub mod circle;
pub mod line;
pub mod text;

use self::{arc::Arc, circle::Circle, line::Line, text::Text};
use crate::drawing::Drawing;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB { r: u8, g: u8, b: u8 },
    U32(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Fill {
    Color(Color),
}

#[derive(Debug, Clone, Copy)]
pub enum Stroke {
    Full {
        color: Color,
        width: f32,
    },
    Dashed {
        color: Color,
        width: f32,
        on: f32,
        off: f32,
    },
}

#[derive(Debug)]
pub enum Shape {
    Drawing(Drawing),
    Text(Text),
    Line(Line),
    Circle(Circle),
    Arc(Arc),
}
