mod curve;
mod ellipse;
mod image;
mod text;

pub use self::image::*;
pub use curve::*;
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
    fn scale(&mut self, scale: Scale2<f32>) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(scale));
        self
    }
    #[inline]
    fn rotate(&mut self, rotation: Rotation2<f32>) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(rotation));
        self
    }

    fn local_transform(&self) -> &Transform2<f32>;
    #[inline]
    fn global_transform(&self, parent_transform: &Transform2<f32>) -> Transform2<f32> {
        parent_transform * self.local_transform()
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
        self.scale(scale);
        self
    }
    #[inline]
    fn with_rotate(mut self, rotation: Rotation2<f32>) -> Self {
        self.rotate(rotation);
        self
    }
}
impl<T: ShapeOp> ShapeOpWith for T {}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Group {
        local_transform: Transform2<f32>,
        shapes: Vec<Shape>,
    },
    Style {
        fill: Option<crate::style::Fill>,
        stroke: Option<crate::style::Stroke>,
        shape: Box<Shape>,
    },
    Ellipse(Ellipse),
    Image(Image),
    Text(Text),
    Curve(Curve),
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Group {
            local_transform: Transform2::default(),
            shapes: vec![],
        }
    }
}

impl ShapeOp for Shape {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        match self {
            Shape::Group {
                local_transform, ..
            } => {
                *local_transform *= transform_matrix;
            }
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

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        match self {
            Shape::Group {
                local_transform, ..
            } => local_transform,
            Shape::Style { shape, .. } => shape.local_transform(),
            Shape::Ellipse(v) => v.local_transform(),
            Shape::Image(v) => v.local_transform(),
            Shape::Text(v) => v.local_transform(),
            _ => todo!(),
        }
    }
}
