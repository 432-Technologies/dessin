pub mod line;
pub mod text;

use self::{line::Line, text::Text};

#[derive(Debug)]
pub enum Fill {
    Color(u32),
}

#[derive(Debug)]
pub enum Stroke {
    Full(u32, u32),
    Dashed(u32, u32, u32),
}

#[derive(Debug)]
pub enum Shape {
    Text(Text),
    Line(Line),
}
