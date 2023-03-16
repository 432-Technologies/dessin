use crate::prelude::{Ellipse, Shape, ShapeOp};
use nalgebra::{Scale2, Transform2};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Circle {
    pub local_transform: Transform2<f32>,
}

impl Circle {
    #[inline]
    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.scale(Scale2::new(2. * radius, 2. * radius));
        self
    }
    #[inline]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius(radius);
        self
    }
}

impl ShapeOp for Circle {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}

impl From<Circle> for Shape {
    #[inline]
    fn from(Circle { local_transform }: Circle) -> Self {
        Shape::Ellipse(Ellipse { local_transform })
    }
}
