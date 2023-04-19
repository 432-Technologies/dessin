use std::f32::consts::FRAC_PI_2;

use crate::{
    prelude::{Ellipse, Shape, ShapeOp},
    shapes::{Bezier, Curve, Keypoint},
};
use nalgebra::{self as na, Point2, Rotation2, Scale2, Transform2};

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

impl From<Circle> for Curve {
    #[inline]
    fn from(Circle { local_transform }: Circle) -> Self {
        let mut q1 = Bezier {
            start: None,
            start_control: Point2::new(0.5, 0.552284749831 / 2.),
            end_control: Point2::new(0.552284749831 / 2., 0.5),
            end: Point2::new(0., 0.5),
        };
        let q2 = q1.transform(&na::convert(Rotation2::new(FRAC_PI_2)));
        let q3 = q2.transform(&na::convert(Rotation2::new(FRAC_PI_2)));
        let q4 = q3.transform(&na::convert(Rotation2::new(FRAC_PI_2)));

        q1.start = Some(Point2::new(0.5, 0.));

        Curve {
            keypoints: vec![
                Keypoint::Bezier(q1),
                Keypoint::Bezier(q2),
                Keypoint::Bezier(q3),
                Keypoint::Bezier(q4),
            ],
            local_transform,
            closed: true,
        }
    }
}

impl From<Ellipse> for Circle {
    #[inline]
    fn from(Ellipse { local_transform }: Ellipse) -> Self {
        Circle { local_transform }
    }
}

impl From<Circle> for Ellipse {
    #[inline]
    fn from(Circle { local_transform }: Circle) -> Self {
        Ellipse { local_transform }
    }
}
