pub use crate::shapes::text::{FontWeight, TextAlign};
use algebr::Vec2;

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
    pub(crate) fn apply_transform(&mut self, _: Vec2, scale: f32) {
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

/// A style is a set of attributes that can be applied to a shape.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
