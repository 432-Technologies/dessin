pub mod arc;
pub mod circle;
pub mod drawing;
pub mod line;
pub mod text;

use algebra::Vec2;

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
impl Stroke {
    pub(crate) fn apply_transform(&mut self, pos: Vec2, scale: f32) {
        match self {
            Stroke::Full { color: _, width } => *width *= scale,
            Stroke::Dashed {
                color: _,
                width,
                on,
                off,
            } => {
                *width *= scale;
                *on *= scale;
                *off *= scale;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Drawing(Vec<Shape>),
    Text(Text),
    Line(Line),
    Circle(Circle),
    Arc(Arc),
}
impl Shape {
    pub(crate) fn apply_transform(&mut self, pos: Vec2, scale: f32) {
        match self {
            Shape::Drawing(_) => {
                todo!()
            }
            Shape::Text(v) => {
                v.pos = (v.pos * scale) + pos;
                v.style.font_size *= scale;
                if let Some(ref mut s) = v.style.stroke {
                    s.apply_transform(pos, scale);
                }
            }
            Shape::Line(v) => {
                v.from = (v.from * scale) + pos;
                v.to = (v.to * scale) + pos;
                if let Some(ref mut s) = v.style.stroke {
                    s.apply_transform(pos, scale);
                }
            }
            Shape::Circle(v) => {
                v.pos = (v.pos * scale) + pos;
                v.radius *= scale;
                if let Some(ref mut s) = v.style.stroke {
                    s.apply_transform(pos, scale);
                }
            }
            Shape::Arc(v) => {
                v.pos = (v.pos * scale) + pos;
                v.inner_radius *= scale;
                v.outer_radius *= scale;
                if let Some(ref mut s) = v.style.stroke {
                    s.apply_transform(pos, scale);
                }
            }
        }
    }
}
