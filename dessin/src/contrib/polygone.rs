use crate::prelude::*;
use nalgebra::{Point2, Transform2};
use std::f32::consts::TAU;

/// Regular polygons
pub mod polygons {
    use super::Polygon;

    /// Triangle
    pub type Triangle = Polygon<3>;
    /// Square
    pub type Square = Polygon<4>;
    /// Pentagon
    pub type Pentagon = Polygon<5>;
    /// Hexagon
    pub type Hexagon = Polygon<6>;
    /// Heptagon
    pub type Heptagon = Polygon<7>;
    /// Octogon
    pub type Octogon = Polygon<8>;
    /// Nonagon
    pub type Nonagon = Polygon<9>;
    /// Decagon
    pub type Decagon = Polygon<10>;
    /// Dodecagon
    pub type Dodecagon = Polygon<12>;
}

/// N sided regular polygone
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Polygon<const N: u32> {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,
}
impl<const N: u32> Polygon<N> {
    const STEP: f32 = TAU / N as f32;
}

impl<const N: u32> From<Polygon<N>> for Shape {
    fn from(Polygon { local_transform }: Polygon<N>) -> Self {
        let step = Polygon::<N>::STEP;

        dessin!(
            Curve: (
                extend={
                    (0..N).map(|p| Point2::from([(p as f32 * step).cos(), (p as f32 * step).sin()]).into())
                }
                closed
                transform={local_transform}
            ) -> ()
        )
    }
}

#[test]
fn triangle() {
    use crate::prelude::{polygons::*, *};
    use assert_float_eq::*;

    let sqrt3_over_2 = 3f32.sqrt() / 2.;

    let Shape::Curve(triangle) = Triangle::default().as_shape() else {
        panic!("Not a curve");
    };

    for (a, b) in triangle.keypoints.iter().zip(
        [
            Point2::new(1., 0.),
            Point2::new(-0.5, sqrt3_over_2),
            Point2::new(-0.5, -sqrt3_over_2),
        ]
        .iter(),
    ) {
        let Keypoint::Point(p) = a else {
            panic!("Not a point");
        };

        assert_float_absolute_eq!(p.x, b.x, 10e-5);
        assert_float_absolute_eq!(p.y, b.y, 10e-5);
    }
}

#[test]
fn square() {
    use crate::prelude::{polygons::*, *};
    use assert_float_eq::*;

    let Shape::Curve(square) = Square::default().as_shape() else {
        panic!("Not a curve");
    };

    for (a, b) in square.keypoints.iter().zip(
        [
            Point2::new(1., 0.),
            Point2::new(0., 1.),
            Point2::new(-1., 0.),
            Point2::new(0., -1.),
        ]
        .iter(),
    ) {
        let Keypoint::Point(p) = a else {
            panic!("Not a point");
        };

        assert_float_absolute_eq!(p.x, b.x, 10e-5);
        assert_float_absolute_eq!(p.y, b.y, 10e-5);
    }
}

#[test]
fn triangle_in_group() {
    use crate::prelude::{polygons::*, *};
    use assert_float_eq::*;
    use nalgebra::Transform2;

    let sqrt3_over_2 = 3f32.sqrt() / 2.;

    let Shape::Group (Group{ local_transform, shapes, .. }) = dessin!([Triangle: ()]) else {
        panic!("Not a group");
    };
    assert_eq!(shapes.len(), 1);
    assert_eq!(local_transform, Transform2::<f32>::default());

    let Shape::Curve(triangle) = shapes[0].clone() else {
        panic!("Not a curve");
    };

    for (a, b) in triangle.keypoints.iter().zip(
        [
            Point2::new(1., 0.),
            Point2::new(-0.5, sqrt3_over_2),
            Point2::new(-0.5, -sqrt3_over_2),
        ]
        .iter(),
    ) {
        let Keypoint::Point(p) = a else {
            panic!("Not a point");
        };

        assert_float_absolute_eq!(p.x, b.x, 10e-5);
        assert_float_absolute_eq!(p.y, b.y, 10e-5);
    }
}

#[test]
fn bounding_box() {
    use crate::prelude::{polygons::*, *};
    use nalgebra::Rotation2;
    use std::f32::consts::FRAC_PI_4;

    let polys: [(usize, Shape); 4] = [
        (3, Triangle::default().into()),
        (4, Square::default().into()),
        (8, Octogon::default().into()),
        (10, Decagon::default().into()),
    ];

    for (n, mut poly) in polys.into_iter() {
        let bb = poly.local_bounding_box();
        assert!(bb.width() <= 2., "{} <= 2. for {n}-gon", bb.width());
        assert!(bb.height() <= 2., "{} <= 2. for {n}-gon", bb.height());

        poly.rotate(Rotation2::new(FRAC_PI_4));
        let bb = poly.local_bounding_box();
        assert!(bb.width() <= 2., "{} <= 2. for {n}-gon", bb.width());
        assert!(bb.height() <= 2., "{} <= 2. for {n}-gon", bb.height());
        let bb = bb.straigthen();
        assert!(
            2. < bb.width() && bb.width() < 3.,
            "2. > {} < 3.  for {n}-gon",
            bb.width()
        );
        assert!(
            2. < bb.height() && bb.height() < 3.,
            "2. > {} < 3.  for {n}-gon",
            bb.height()
        );
    }
}
