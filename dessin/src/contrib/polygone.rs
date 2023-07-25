use crate::prelude::*;
use nalgebra::{Point2, Rotation2, Scale2, Transform2};
use std::f32::consts::{FRAC_PI_4, TAU};

pub mod polygones {
    use super::Polygone;
    pub type Triangle = Polygone<3>;
    pub type Square = Polygone<4>;
    pub type Pentagon = Polygone<5>;
    pub type Hexagon = Polygone<6>;
    pub type Heptagon = Polygone<7>;
    pub type Octogon = Polygone<8>;
    pub type Nonagon = Polygone<9>;
    pub type Decagon = Polygone<10>;
    pub type Dodecagon = Polygone<12>;
}

#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Polygone<const N: u32> {
    #[local_transform]
    pub local_transform: Transform2<f32>,
}
impl<const N: u32> Polygone<N> {
    const STEP: f32 = TAU / N as f32;
}

impl<const N: u32> From<Polygone<N>> for Shape {
    fn from(Polygone { local_transform }: Polygone<N>) -> Self {
        let step = Polygone::<N>::STEP;

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
fn bounding_box() {
    use crate::prelude::{polygones::*, *};
    use assert_float_eq::*;

    let polys: [(usize, Shape); 4] = [
        (3, Triangle::default().into()),
        (4, Square::default().into()),
        (8, Octogon::default().into()),
        (10, Decagon::default().into()),
    ];

    for (n, mut poly) in polys.into_iter() {
        let bb = poly.local_bounding_box().unwrap();
        assert!(bb.width() <= 2., "{} <= 2. for {n}-gon", bb.width());
        assert!(bb.height() <= 2., "{} <= 2. for {n}-gon", bb.height());

        poly.rotate(Rotation2::new(FRAC_PI_4));
        let bb = poly.local_bounding_box().unwrap();
        assert!(bb.width() <= 2., "{} <= 2. for {n}-gon", bb.width());
        assert!(bb.height() <= 2., "{} <= 2. for {n}-gon", bb.height());
        let bb = bb.straigthen();
        assert!(bb.width() <= 2., "{} <= 2. for {n}-gon", bb.width());
        assert!(bb.height() <= 2., "{} <= 2. for {n}-gon", bb.height());
    }
}
