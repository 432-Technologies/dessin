use crate::prelude::*;
use nalgebra::Transform2;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub type Shaper = dyn Fn() -> Shape;

pub trait DynamicShape: std::fmt::Debug {
    fn as_shape(&self) -> Shape;
}

impl<T: std::fmt::Debug + Clone + Into<Shape>> DynamicShape for T {
    fn as_shape(&self) -> Shape {
        self.clone().into()
    }
}

#[derive(Clone, Debug, PartialEq, Shape)]
pub struct Dynamic<T> {
    #[local_transform]
    local_transform: Transform2<f32>,
    #[shape(skip)]
    shape: T,
}

impl<T: Default> Default for Dynamic<T> {
    fn default() -> Self {
        Dynamic {
            shape: T::default(),
            local_transform: Transform2::default(),
        }
    }
}

impl<T> Deref for Dynamic<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Dynamic<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T> From<Dynamic<T>> for Shape
where
    T: DynamicShape + 'static,
{
    fn from(
        Dynamic {
            local_transform,
            shape,
        }: Dynamic<T>,
    ) -> Self {
        Shape::Dynamic {
            local_transform,
            shaper: Arc::new(move || shape.as_shape()),
        }
    }
}
