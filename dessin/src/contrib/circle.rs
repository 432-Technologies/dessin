use crate::{
    prelude::{Ellipse, Shape, ShapeOp},
    shapes::{Bezier, Curve, Keypoint},
};
use nalgebra::{self as na, Point2, Rotation2, Scale2, Transform2};
use std::f32::consts::FRAC_PI_2;

/// Circle with a radius
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Circle {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,
}

impl Circle {
    /// Radius
    #[inline]
    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.scale(Scale2::new(2. * radius, 2. * radius));
        self
    }

    /// Radius
    #[inline]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius(radius);
        self
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

#[test]
pub fn bounding_box() {
    use crate::prelude::*;
    use assert_float_eq::*;
    use std::f32::consts::FRAC_PI_4;

    let mut circle: Shape = Circle::default().with_radius(10.).into();

    let bb = circle.local_bounding_box();
    assert_eq!(bb.width(), 20.);
    assert_eq!(bb.height(), 20.);

    let mut ellipse = circle.with_resize(Scale2::new(2., 0.5));
    let bb = ellipse.local_bounding_box();
    assert_eq!(bb.width(), 40.);
    assert_eq!(bb.height(), 10.);

    ellipse.rotate(Rotation2::new(FRAC_PI_2));
    let bb = ellipse.local_bounding_box();
    assert_eq!(bb.width(), 40.);
    assert_eq!(bb.height(), 10.);
    let bb = bb.straigthen();
    assert_float_absolute_eq!(bb.width(), 10., 10e-3);
    assert_float_absolute_eq!(bb.height(), 40., 10e-3);

    ellipse.rotate(Rotation2::new(-FRAC_PI_4));
    let bb = ellipse.local_bounding_box();
    assert_eq!(bb.width(), 40.);
    assert_eq!(bb.height(), 10.);
    let bb = bb.straigthen();
    assert_float_absolute_eq!(bb.width(), 35., 10.); // Good enought for now
    assert_float_absolute_eq!(bb.height(), 35., 10.); // Good enought for now
}

#[test]
pub fn bounding_box_7() {
    use crate::prelude::*;

    let mut circle: Shape = Circle::default().with_radius(7.).into();

    let bb = circle.local_bounding_box();
    assert_eq!(bb.width(), 14.);
    assert_eq!(bb.height(), 14.);

    let group = dessin!([
        Circle: (
            radius={7.}
        ),
    ]);
    let bb = group.local_bounding_box();
    assert_eq!(bb.width(), 14.);
    assert_eq!(bb.height(), 14.);
}
