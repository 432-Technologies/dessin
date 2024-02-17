use crate::prelude::*;
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Shape)]
pub struct Padding<T> {
    pub shape: T,

    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
}
impl<T> Padding<T> {
    /// Wrap a [`Shape`] with Padding
    #[inline]
    pub fn new(shape: T) -> Self {
        Padding {
            shape,
            padding_left: 0.,
            padding_right: 0.,
            padding_top: 0.,
            padding_bottom: 0.,
        }
    }

    #[inline]
    pub fn padding_x(&mut self, padding: f32) -> &mut Self {
        self.padding_left = padding;
        self.padding_right = padding;
        self
    }
    #[inline]
    pub fn with_padding_x(mut self, padding: f32) -> Self {
        self.padding_x(padding);
        self
    }

    #[inline]
    pub fn padding_y(&mut self, padding: f32) -> &mut Self {
        self.padding_top = padding;
        self.padding_bottom = padding;
        self
    }
    #[inline]
    pub fn with_padding_y(mut self, padding: f32) -> Self {
        self.padding_y(padding);
        self
    }

    #[inline]
    pub fn padding(&mut self, padding: f32) -> &mut Self {
        self.padding_x(padding).padding_y(padding)
    }
    #[inline]
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding(padding);
        self
    }
}

impl<T> Deref for Padding<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Padding<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T> From<Padding<T>> for Shape
where
    T: Into<Shape>,
{
    fn from(
        Padding {
            shape,
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
        }: Padding<T>,
    ) -> Self {
        let shape: Shape = shape.into();

        let bb = shape.local_bounding_box().straigthen();

        dessin2!([
            Rectangle(
                scale = [
                    bb.width() + padding_left + padding_right,
                    bb.height() + padding_top + padding_bottom,
                ],
                translate = bb.center()
                    + Vector2::new(
                        (padding_right - padding_left) / 2.,
                        (padding_top - padding_bottom) / 2.,
                    ),
            ),
            var[shape](),
        ])
    }
}

impl<T: ShapeOp> ShapeOp for Padding<T> {
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
