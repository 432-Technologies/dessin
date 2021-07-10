pub mod line;
pub mod text;

use self::{line::Line, text::Text};

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
    Text(Text),
    Line(Line),
}
