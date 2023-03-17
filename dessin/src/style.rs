use crate::shapes::{Shape, ShapeOp};
use nalgebra::{Rotation2, Scale2, Transform2, Translation2};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

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

    /// hue ∈ [0°, 360°], saturation ∈ [0, 1], lightness ∈ [0, 1] and alpha ∈ [0, 1]
    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Color {
        let chroma = 1. - (2. * lightness - 1.).abs() * saturation;
        let hue_prime = hue / 60.;
        let x = chroma * (1. - (hue_prime % 2. - 1.).abs());
        let a = (alpha * 255.) as u8;
        let (r, g, b) = if hue_prime < 1. {
            (chroma, x, 0.)
        } else if hue_prime < 2. {
            (x, chroma, 0.)
        } else if hue_prime < 3. {
            (0., chroma, x)
        } else if hue_prime < 4. {
            (0., x, chroma)
        } else if hue_prime < 5. {
            (x, 0., chroma)
        } else {
            (chroma, 0., x)
        };
        let m = lightness - chroma / 2.;
        Color::RGBA {
            r: ((r + m) * 255.) as u8,
            g: ((g + m) * 255.) as u8,
            b: ((b + m) * 255.) as u8,
            a,
        }
    }

    pub fn as_rgb(&self) -> (u8, u8, u8) {
        match *self {
            Color::RGBA { r, g, b, a: _ } => (r, g, b),
            Color::RGB { r, g, b } => (r, g, b),
            Color::U32(c) => (
                ((c >> 16) & 0xFF) as u8,
                ((c >> 8) & 0xFF) as u8,
                (c & 0xFF) as u8,
            ),
        }
    }

    pub fn as_rgb_f64(&self) -> (f64, f64, f64) {
        let (r, g, b) = self.as_rgb();
        (r as f64, g as f64, b as f64)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rgba() {
            Color::RGBA { r, g, b, a } => {
                write!(f, "rgba({},{},{},{})", r, g, b, a as f32 / 255.)
            }
            _ => unreachable!(),
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Style<T> {
    pub shape: T,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl<T> Style<T> {
    pub fn new(shape: T) -> Self {
        Style {
            shape,
            fill: None,
            stroke: None,
        }
    }

    #[inline]
    pub fn stroke(&mut self, stroke: Stroke) -> &mut Self {
        self.stroke = Some(stroke);
        self
    }
    #[inline]
    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke(stroke);
        self
    }

    #[inline]
    pub fn fill(&mut self, fill: Fill) -> &mut Self {
        self.fill = Some(fill);
        self
    }
    #[inline]
    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill(fill);
        self
    }
}

impl<T> Deref for Style<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Style<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T: ShapeOp> From<Style<T>> for Shape {
    #[inline]
    fn from(
        Style {
            shape,
            fill,
            stroke,
        }: Style<T>,
    ) -> Self {
        Shape::Style {
            fill,
            stroke,
            shape: Box::new(shape.into()),
        }
    }
}

impl<T: ShapeOp> ShapeOp for Style<T> {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.shape.transform(transform_matrix);
        self
    }

    #[inline]
    fn translate(&mut self, translation: Translation2<f32>) -> &mut Self {
        self.shape.translate(translation);
        self
    }
    #[inline]
    fn scale(&mut self, scale: Scale2<f32>) -> &mut Self {
        self.shape.scale(scale);
        self
    }
    #[inline]
    fn rotate(&mut self, rotation: Rotation2<f32>) -> &mut Self {
        self.shape.rotate(rotation);
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        self.shape.local_transform()
    }
    #[inline]
    fn global_transform(&self, parent_transform: &Transform2<f32>) -> Transform2<f32> {
        self.shape.global_transform(parent_transform)
    }
}
