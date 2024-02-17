use crate::prelude::*;
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use std::{
    f32::consts::FRAC_1_SQRT_2,
    fmt,
    ops::{Deref, DerefMut, Mul},
};

/// Create a color from red, green and blue
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::RGB { r, g, b }
}

/// Create a color from red, green, blue and alpha
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::RGBA { r, g, b, a }
}

/// Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    /// RGB with transparency
    RGBA {
        /// red
        r: u8,
        /// green
        g: u8,
        /// blue
        b: u8,
        /// alpha
        a: u8,
    },
    /// RGB
    RGB {
        /// red
        r: u8,
        /// green
        g: u8,
        /// blue
        b: u8,
    },
    /// Raw color code
    U32(u32),
}
impl Color {
    /// #FF0000
    pub const RED: Color = rgb(255, 0, 0);
    /// #00FF00
    pub const GREEN: Color = rgb(0, 255, 0);
    /// #0000FF
    pub const BLUE: Color = rgb(0, 0, 255);
    /// #FFFFFF
    pub const WHITE: Color = rgb(255, 255, 255);
    /// #000000
    pub const BLACK: Color = rgb(0, 0, 0);
    /// #FFFF00
    pub const YELLOW: Color = rgb(255, 255, 0);
    /// #FFA500
    pub const ORANGE: Color = rgb(255, 165, 0);
    /// #FF00FF
    pub const MAGENTA: Color = rgb(255, 0, 255);
    /// #00FFFF
    pub const CYAN: Color = rgb(0, 255, 255);
    /// #808080
    pub const GRAY: Color = rgb(128, 128, 128);
    /// #00000000
    pub const TRANSPARENT: Color = rgba(0, 0, 0, 0);
    /// #C0C0C0
    pub const LIGHT_GRAY: Color = rgb(192, 192, 192);
    /// #404040
    pub const DARK_GRAY: Color = rgb(64, 64, 64);

    /// Cast a color to (red, green, blue, alpha)
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        match self {
            Color::RGBA { r, g, b, a } => (r, g, b, a),
            Color::RGB { r, g, b } => (r, g, b, 255),
            Color::U32(c) => (
                ((c >> 16) & 0xFF) as u8,
                ((c >> 8) & 0xFF) as u8,
                (c & 0xFF) as u8,
                255,
            ),
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

    /// Cast a color to (red, green, blue)
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

    /// Cast a color to (red, green, blue), as f64
    pub fn as_rgb_f32(&self) -> (f32, f32, f32) {
        let (r, g, b) = self.as_rgb();
        (r as f32 / 255., g as f32 / 255., b as f32 / 255.)
    }

    /// Cast a color to (red, green, blue, alpha), as f32
    pub fn as_rgba_f32(&self) -> (f32, f32, f32, f32) {
        let (r, g, b, a) = self.rgba();
        (
            r as f32 / 255.,
            g as f32 / 255.,
            b as f32 / 255.,
            a as f32 / 255.,
        )
    }

    /// Cast a color to (red, green, blue), as f64
    pub fn as_rgb_f64(&self) -> (f64, f64, f64) {
        let (r, g, b) = self.as_rgb();
        (r as f64 / 255., g as f64 / 255., b as f64 / 255.)
    }

    /// Cast a color to (red, green, blue, alpha), as f64
    pub fn as_rgba_f64(&self) -> (f64, f64, f64, f64) {
        let (r, g, b, a) = self.rgba();
        (
            r as f64 / 255.,
            g as f64 / 255.,
            b as f64 / 255.,
            a as f64 / 255.,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b, a) = self.rgba();
        write!(f, "#{r:02X?}{g:02X?}{b:02X?}")?;
        if a < 255 {
            write!(f, "{a:02X?}")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct StylePosition {
    pub stroke: Option<Stroke>,
    pub fill: Option<Fill>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    Color(Color),
}

impl From<Color> for Fill {
    fn from(c: Color) -> Self {
        Fill::Color(c)
    }
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

impl From<(Color, f32)> for Stroke {
    fn from((color, width): (Color, f32)) -> Self {
        Stroke::Full { color, width }
    }
}

impl Mul<Stroke> for Transform2<f32> {
    type Output = Stroke;
    fn mul(self, rhs: Stroke) -> Self::Output {
        match rhs {
            Stroke::Full { color, width } => Stroke::Full {
                color,
                width: (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude() * width,
            },
            Stroke::Dashed {
                color,
                width,
                on,
                off,
            } => {
                let factor = (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude();

                Stroke::Dashed {
                    color,
                    width: width * factor,
                    on: on * factor,
                    off: off * factor,
                }
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Style<T> {
    pub shape: T,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}
impl<T> Style<T> {
    #[inline]
    pub fn new(shape: T) -> Self {
        Style {
            shape,
            fill: None,
            stroke: None,
        }
    }

    #[inline]
    pub fn stroke<S: Into<Stroke>>(&mut self, stroke: S) -> &mut Self {
        self.stroke = Some(stroke.into());
        self
    }
    #[inline]
    pub fn with_stroke<S: Into<Stroke>>(mut self, stroke: S) -> Self {
        self.stroke(stroke);
        self
    }

    #[inline]
    pub fn fill<F: Into<Fill>>(&mut self, fill: F) -> &mut Self {
        self.fill = Some(fill.into());
        self
    }
    #[inline]
    pub fn with_fill<F: Into<Fill>>(mut self, fill: F) -> Self {
        self.fill(fill);
        self
    }
}

impl<T> Deref for Style<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Style<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T: Into<Shape>> From<Style<T>> for Shape {
    #[inline]
    fn from(
        Style {
            shape,
            fill,
            stroke,
        }: Style<T>,
    ) -> Self {
        if fill.is_none() && stroke.is_none() {
            shape.into()
        } else {
            Shape::Style {
                fill,
                stroke,
                shape: Box::new(shape.into()),
            }
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
    fn translate<U: Into<Translation2<f32>>>(&mut self, translation: U) -> &mut Self {
        self.shape.translate(translation);
        self
    }
    #[inline]
    fn scale<S: Into<Scale2<f32>>>(&mut self, scale: S) -> &mut Self {
        self.shape.scale(scale);
        self
    }
    #[inline]
    fn rotate<R: Into<Rotation2<f32>>>(&mut self, rotation: R) -> &mut Self {
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

impl<T: ShapeBoundingBox> ShapeBoundingBox for Style<T> {
    #[inline]
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        self.shape.local_bounding_box()
    }
}
