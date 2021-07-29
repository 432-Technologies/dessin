pub use crate::shapes::text::{FontWeight, TextAlign};

pub fn rbg(r: u8, g: u8, b: u8) -> Color {
    Color::RGB { r, g, b }
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::RGBA { r, g, b, a }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    RGB { r: u8, g: u8, b: u8 },
    U32(u32),
}
impl Color {
    pub fn rgba(self) -> Color {
        match self {
            Color::RGBA { .. } => self,
            Color::RGB { r, g, b } => Color::RGBA { r, g, b, a: 255 },
            Color::U32(c) => Color::RGBA {
                r: ((c >> 16) & 0xFF) as u8,
                g: ((c >> 8) & 0xFF) as u8,
                b: (c & 0xFF) as u8,
                a: 255,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    Color(Color),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
// impl Stroke {
//     pub(crate) fn apply_transform(&mut self, _: Vec2, scale: f32) {
//         match self {
//             Stroke::Full { color: _, width } => *width *= scale,
//             Stroke::Dashed {
//                 color: _,
//                 width,
//                 on,
//                 off,
//             } => {
//                 *width *= scale;
//                 *on *= scale;
//                 *off *= scale;
//             }
//         }
//     }
// }

/// A style is a set of attributes that can be applied to a shape.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
