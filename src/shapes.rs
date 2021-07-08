use self::{line::Line, text::Text};

pub mod line;
pub mod text;

pub enum Shape {
    Text(Text),
    Line(Line),
}

#[derive(Debug)]
pub struct Style {}
