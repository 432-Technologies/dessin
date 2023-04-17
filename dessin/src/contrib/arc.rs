use crate::prelude::*;
use nalgebra::{Scale2, Transform2};
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Arc {
    pub local_transform: Transform2<f32>,
    pub start_rad: f32,
    pub end_rad: f32,
}
impl Arc {
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
    // #[inline]
    // pub fn anchor(&mut self, anchor: Vector2<f32>) -> &mut Self {
    //     self.anchor = anchor;
    //     self
    // }

    // #[inline]
    // pub fn with_anchor(mut self, anchor: Vector2<f32>) -> Self {
    //     self.anchor(anchor);
    //     self
    // }
}
impl From<Arc> for Shape {
    fn from(
        Arc {
            local_transform,
            start_rad,
            end_rad,
        }: Arc,
    ) -> Self {
        let span = (end_rad + 2. * PI - start_rad) % (2. * 360.);

        if (span - 2. * PI).abs() < 1e-6 {
            Circle { local_transform }.into()
        } else {
        }

        let circle_quarters = (span / FRAC_PI_2) as usize;
        if circle_quarters > 1 {
            for c in 0..circle_quarters - 1 {}
        }
        todo!()
    }
}

impl ShapeOp for Arc {
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

// #[derive(Debug, Clone, PartialEq)]
// pub struct Arc {
//     pub(crate) pos: Rect,
//     pub(crate) radius: f32,
//     pub(crate) start_angle: Angle,
//     pub(crate) end_angle: Angle,
//     pub(crate) style: Option<Style>,
// }
// crate::impl_pos_at!(Arc);
// crate::impl_pos_anchor!(Arc);
// crate::impl_style!(Arc);
// impl Arc {
//     pub const fn new() -> Arc {
//         Arc {
//             pos: Rect::new(),
//             radius: 0.0,
//             start_angle: Angle::radians(0.0),
//             end_angle: Angle::radians(0.0),
//             style: None,
//         }
//     }

//     pub const fn with_radius(mut self, radius: f32) -> Arc {
//         self.radius = radius;
//         self
//     }

//     pub const fn with_start_angle(mut self, start_angle: Angle) -> Arc {
//         self.start_angle = start_angle;
//         self
//     }

//     pub const fn with_end_angle(mut self, end_angle: Angle) -> Arc {
//         self.end_angle = end_angle;
//         self
//     }
// }

// impl Into<Keypoints> for Arc {
//     fn into(self) -> Keypoints {
//         fn normalize_rad(mut r: f32) -> f32 {
//             while r > PI {
//                 r -= 2. * PI
//             }
//             while r <= -PI {
//                 r += 2. * PI
//             }
//             r
//         }

//         let start = normalize_rad(self.start_angle.to_rad());
//         let end = normalize_rad(self.end_angle.to_rad());

//         let mut tmp_end = normalize_rad(end - start);
//         let mut tmp_angle_acc = 0.;

//         let start_quarter = Quarter::TopRight;
//         let end_quarter = match tmp_end {
//             x if x <= -FRAC_PI_2 => Quarter::BottomLeft,
//             x if x <= 0. => Quarter::BottomRight,
//             x if x <= FRAC_PI_2 => Quarter::TopRight,
//             _ => Quarter::TopLeft,
//         };
//         let mut move_quarter = start_quarter;

//         let mut ks = vec![Keypoint::Point(vec2(1., 0.) * self.radius + self.pos.pos)];

//         loop {
//             if move_quarter == end_quarter {
//                 let theta = tmp_end / 2.;

//                 let x0 = theta.cos();
//                 let y0 = theta.sin();

//                 let x1 = (4. - x0) / 3.;
//                 let y1 = (1. - x0) * (3. - x0) / (3. * y0);

//                 let x2 = x1;
//                 let y2 = -y1;

//                 let x3 = x0;
//                 let y3 = -y0;

//                 ks.push(
//                     Keypoint::BezierCubic {
//                         to: vec2(x3, y3).rot_rad(theta + tmp_angle_acc),
//                         control_to: vec2(x2, y2).rot_rad(theta + tmp_angle_acc),
//                         control_from: vec2(x1, y1).rot_rad(theta + tmp_angle_acc),
//                     } * self.radius
//                         + self.pos.pos,
//                 );

//                 break;
//             }

//             ks.extend(
//                 Into::<Keypoints>::into(
//                     QuarterCircle::new(move_quarter)
//                         .at(self.pos.pos)
//                         .with_anchor(self.pos.anchor)
//                         .with_radius(self.radius),
//                 )
//                 .0,
//             );

//             move_quarter = match move_quarter {
//                 Quarter::TopLeft => Quarter::BottomLeft,
//                 Quarter::BottomLeft => Quarter::BottomRight,
//                 Quarter::BottomRight => Quarter::TopRight,
//                 Quarter::TopRight => Quarter::TopLeft,
//             };

//             tmp_end = normalize_rad(tmp_end - FRAC_PI_2);
//             tmp_angle_acc = normalize_rad(tmp_angle_acc + FRAC_PI_2);
//         }

//         ks.iter_mut().for_each(|v| {
//             *v = v.rot_rad(-start);
//         });

//         Keypoints(ks)
//     }
// }

// // impl Into<Shape> for Arc {
// //     fn into(self) -> Shape {
// //         Path::new().then_do(Into::<Keypoints>::into(self)).into()
// //     }
// // }
