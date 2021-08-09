pub use crate::shapes::text::{FontWeight, TextAlign};

pub const fn rbg(r: u8, g: u8, b: u8) -> Color {
    Color::RGB { r, g, b }
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::RGBA { r, g, b, a }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    RGB { r: u8, g: u8, b: u8 },
    U32(u32),
}
impl Color {
    pub const RED: Color = rbg(255, 0, 0);
    pub const GREEN: Color = rbg(0, 255, 0);
    pub const BLUE: Color = rbg(0, 0, 255);
    pub const WHITE: Color = rbg(255, 255, 255);
    pub const BLACK: Color = rbg(0, 0, 0);
    pub const YELLOW: Color = rbg(255, 255, 0);
    pub const ORANGE: Color = rbg(255, 165, 0);
    pub const MAGENTA: Color = rbg(255, 0, 255);
    pub const CYAN: Color = rbg(0, 255, 255);
    pub const GRAY: Color = rbg(128, 128, 128);
    pub const TRANSPARENT: Color = rgba(0, 0, 0, 0);
    pub const LIGHT_GRAY: Color = rbg(192, 192, 192);
    pub const DARK_GRAY: Color = rbg(64, 64, 64);

    pub const fn rgba(self) -> Color {
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
