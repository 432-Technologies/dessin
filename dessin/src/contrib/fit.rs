use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;

use crate::prelude::*;

/// Center and scale a shape to be contain into a given bounding_box
#[derive(Default, Shape)]
pub struct Fit<T> {
    /// The underlying shape
    #[shape(skip)]
    pub shape: T,

    /// Should the underlying shape be stretch or not ?
    #[shape(bool)]
    pub keep_ratio: bool,

    /// Container
    #[shape(skip)]
    pub bounding_box: Option<BoundingBox<Straight>>,
}

impl<T> Fit<T> {
    /// Container
    #[inline]
    pub fn bounding_box(&mut self, bb: BoundingBox<Straight>) -> &mut Self {
        self.bounding_box = Some(bb);
        self
    }

    /// Container
    #[inline]
    pub fn with_bounding_box(mut self, bb: BoundingBox<Straight>) -> Self {
        self.bounding_box(bb);
        self
    }
}

impl<T> Deref for Fit<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Fit<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T: Into<Shape>> From<Fit<T>> for Shape {
    fn from(
        Fit {
            shape,
            keep_ratio,
            bounding_box,
        }: Fit<T>,
    ) -> Self {
        let shape: Shape = shape.into();

        let shape_bb = shape.local_bounding_box().straigthen();

        let mut scale = if let Some(bb) = bounding_box {
            shape_bb.scale_difference(&bb)
        } else {
            Vector2::new(1., 1.)
        };

        if keep_ratio {
            let v = scale.x.min(scale.y);
            scale = Vector2::new(v, v);
        }

        let translation = shape_bb.center();

        shape.with_translate(-translation).with_resize(scale)
    }
}
