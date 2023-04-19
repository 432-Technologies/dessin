use crate::prelude::{Shape, ShapeBoundingBox, ShapeOp, ShapeOpWith};
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Anchor<T> {
    pub shape: T,
    pub anchor: Vector2<f32>,
}
impl<T> Anchor<T> {
    #[inline]
    pub fn new(shape: T) -> Self {
        Anchor {
            shape,
            anchor: Vector2::default(),
        }
    }

    #[inline]
    pub fn anchor(&mut self, anchor: Vector2<f32>) -> &mut Self {
        self.anchor = anchor;
        self
    }

    #[inline]
    pub fn with_anchor(mut self, anchor: Vector2<f32>) -> Self {
        self.anchor(anchor);
        self
    }
}

impl<T> Deref for Anchor<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Anchor<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T> From<Anchor<T>> for Shape
where
    T: Into<Shape>,
{
    fn from(Anchor { shape, anchor }: Anchor<T>) -> Self {
        let shape: Shape = shape.into();
        if let Some(bb) = shape.local_bounding_box() {
            let bb = bb.straigthen();
            let width = bb.width() / 2.;
            let height = bb.height() / 2.;
            shape.with_translate(Translation2::new(-anchor.x * width, -anchor.y * height))
        } else {
            shape
        }
    }
}

impl<T: ShapeOp> ShapeOp for Anchor<T> {
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
