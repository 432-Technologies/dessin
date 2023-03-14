mod curve;
mod ellipse;
mod image;
mod text;

pub use self::image::*;
pub use ellipse::*;
pub use text::*;

use na::{Rotation2, Scale2};
use nalgebra::{self as na, Transform2, Translation2};

pub trait ShapeOp: Into<Shape> + Clone {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self;

    #[inline]
    fn translate(&mut self, translation: Translation2<f32>) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(translation));
        self
    }
    #[inline]
    fn resize(&mut self, scale: Scale2<f32>) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(scale));
        self
    }
    #[inline]
    fn rotate(&mut self, rotation: Rotation2<f32>) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(rotation));
        self
    }
}

pub trait ShapeOpWith: ShapeOp {
    #[inline]
    fn with_transform(mut self, transform_matrix: Transform2<f32>) -> Self {
        self.transform(transform_matrix);
        self
    }

    #[inline]
    fn with_translate(mut self, translation: Translation2<f32>) -> Self {
        self.translate(translation);
        self
    }
    #[inline]
    fn with_resize(mut self, scale: Scale2<f32>) -> Self {
        self.resize(scale);
        self
    }
    #[inline]
    fn with_rotate(mut self, rotation: Rotation2<f32>) -> Self {
        self.rotate(rotation);
        self
    }
}
impl<T: ShapeOp> ShapeOpWith for T {}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Shape {
    #[default]
    Empty,
    Group(Vec<Shape>),
    Style {
        fill: Option<crate::style::Fill>,
        stroke: Option<crate::style::Stroke>,
        shape: Box<Shape>,
    },
    Ellipse(Ellipse),
    Image(Image),
    Text(Text),
    Curve(),
}
impl ShapeOp for Shape {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        match self {
            Shape::Empty => {}
            Shape::Group(v) => v.iter_mut().for_each(|v| {
                v.transform(transform_matrix);
            }),
            Shape::Style { shape, .. } => {
                shape.transform(transform_matrix);
            }
            Shape::Ellipse(v) => {
                v.transform(transform_matrix);
            }
            Shape::Image(v) => {
                v.transform(transform_matrix);
            }
            Shape::Text(v) => {
                v.transform(transform_matrix);
            }
            _ => todo!(),
        };

        self
    }
}
