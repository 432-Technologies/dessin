mod curve;
mod ellipse;
mod image;
mod text;

pub use self::image::*;
pub use curve::*;
pub use ellipse::*;
use std::marker::PhantomData;
pub use text::*;

use na::{Point2, Rotation2, Scale2};
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

pub struct UnParticular;
pub struct Straight;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox<Type> {
    _ty: PhantomData<Type>,
    top_left: Point2<f32>,
    top_right: Point2<f32>,
    bottom_right: Point2<f32>,
    bottom_left: Point2<f32>,
}
impl<T> BoundingBox<T> {
    pub fn top_left(&self) -> Point2<f32> {
        self.top_left
    }
    pub fn top_right(&self) -> Point2<f32> {
        self.top_right
    }
    pub fn bottom_right(&self) -> Point2<f32> {
        self.bottom_right
    }
    pub fn bottom_left(&self) -> Point2<f32> {
        self.bottom_left
    }

    pub fn straigthen(&self) -> BoundingBox<Straight> {
        let top = self
            .top_left
            .y
            .max(self.top_right.y)
            .max(self.bottom_left.y)
            .max(self.bottom_right.y);
        let bottom = self
            .top_left
            .y
            .min(self.top_right.y)
            .min(self.bottom_left.y)
            .min(self.bottom_right.y);

        let right = self
            .top_left
            .x
            .max(self.top_right.x)
            .max(self.bottom_left.x)
            .max(self.bottom_right.x);
        let left = self
            .top_left
            .x
            .min(self.top_right.x)
            .min(self.bottom_left.x)
            .min(self.bottom_right.x);

        BoundingBox {
            _ty: PhantomData,
            top_left: Point2::new(left, top),
            top_right: Point2::new(right, top),
            bottom_right: Point2::new(right, bottom),
            bottom_left: Point2::new(left, bottom),
        }
    }

    pub fn transform(self, transform: &Transform2<f32>) -> BoundingBox<UnParticular> {
        BoundingBox {
            _ty: PhantomData,
            top_left: transform * self.top_left,
            top_right: transform * self.top_right,
            bottom_right: transform * self.bottom_right,
            bottom_left: transform * self.bottom_left,
        }
    }

    pub fn width(&self) -> f32 {
        (self.top_right - self.top_left).magnitude()
    }

    pub fn height(&self) -> f32 {
        (self.top_right - self.bottom_right).magnitude()
    }
}

impl BoundingBox<UnParticular> {
    pub fn new(
        top_left: Point2<f32>,
        top_right: Point2<f32>,
        bottom_right: Point2<f32>,
        bottom_left: Point2<f32>,
    ) -> Self {
        BoundingBox {
            _ty: PhantomData,
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl BoundingBox<Straight> {
    pub fn as_unparticular(self) -> BoundingBox<UnParticular> {
        BoundingBox {
            _ty: PhantomData,
            top_left: self.top_left,
            top_right: self.top_right,
            bottom_right: self.bottom_right,
            bottom_left: self.bottom_left,
        }
    }

    pub fn join(mut self, other: BoundingBox<Straight>) -> BoundingBox<Straight> {
        self.top_left.x = self.top_left.x.min(other.top_left.x);
        self.top_left.y = self.top_left.y.max(other.top_left.y);
        self.top_right.x = self.top_right.x.max(other.top_right.x);
        self.top_right.y = self.top_right.y.max(other.top_right.y);
        self.bottom_right.x = self.bottom_right.x.max(other.bottom_right.x);
        self.bottom_right.y = self.bottom_right.y.min(other.bottom_right.y);
        self.bottom_left.x = self.bottom_left.x.min(other.bottom_left.x);
        self.bottom_left.y = self.bottom_left.y.min(other.bottom_left.y);

        self
    }
}

pub trait ShapeBoundingBox: ShapeOp {
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>>;
    fn global_bounding_box(
        &self,
        parent_transform: &Transform2<f32>,
    ) -> Option<BoundingBox<UnParticular>> {
        self.local_bounding_box()
            .map(|v| v.transform(&self.global_transform(parent_transform)))
    }
}

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
                *local_transform = transform_matrix * *local_transform;
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

impl ShapeBoundingBox for Shape {
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>> {
        match self {
            Shape::Group {
                local_transform,
                shapes,
            } => shapes
                .iter()
                .filter_map(|v| v.local_bounding_box())
                .map(|v| v.straigthen())
                .reduce(|acc, curr| BoundingBox::join(acc, curr))
                .map(|v| v.transform(local_transform)),
            Shape::Style { shape, .. } => shape.local_bounding_box(),
            Shape::Ellipse(e) => e.local_bounding_box(),
            Shape::Image(i) => i.local_bounding_box(),
            Shape::Text(t) => t.local_bounding_box(),
            Shape::Curve(c) => c.local_bounding_box(),
        }
    }
}
