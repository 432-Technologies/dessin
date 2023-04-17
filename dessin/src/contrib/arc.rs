use crate::prelude::*;
use nalgebra::{Point2, Rotation2, Scale2, Transform2};
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Arc {
    pub local_transform: Transform2<f32>,
    pub start_angle: f32,
    pub end_angle: f32,
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

    #[inline]
    pub fn start_angle(&mut self, start_angle: f32) -> &mut Self {
        self.start_angle = start_angle;
        self
    }
    #[inline]
    pub fn with_start_angle(mut self, start_angle: f32) -> Self {
        self.start_angle(start_angle);
        self
    }

    #[inline]
    pub fn end_angle(&mut self, end_angle: f32) -> &mut Self {
        self.end_angle = end_angle;
        self
    }
    #[inline]
    pub fn with_end_angle(mut self, end_angle: f32) -> Self {
        self.end_angle(end_angle);
        self
    }
}

impl From<Arc> for Curve {
    fn from(
        Arc {
            local_transform,
            start_angle: start_rad,
            end_angle: end_rad,
        }: Arc,
    ) -> Self {
        let mut span = (end_rad + 2. * PI - start_rad) % (2. * PI);

        if (span - 2. * PI).abs() < 1e-6 {
            Curve::from(Circle { local_transform })
        } else {
            let mut arcs = vec![];
            let mut start = start_rad;
            while span > 0. {
                let s = span.max(FRAC_PI_2);

                let start_rot = Rotation2::new(start);
                let end_rot = Rotation2::new(start + s);

                arcs.push(
                    Bezier {
                        start: Some(start_rot * Point2::new(1., 0.)),
                        start_control: start_rot * Point2::new(1., 0.552284749831),
                        end_control: end_rot * Point2::new(0.552284749831, 1.),
                        end: end_rot * Point2::new(0., 1.),
                    }
                    .into(),
                );

                span -= FRAC_PI_2;
                start += FRAC_PI_2;
            }

            Curve {
                closed: false,
                keypoints: arcs,
                local_transform,
            }
            .into()
        }
    }
}

impl From<Arc> for Shape {
    #[inline]
    fn from(arc: Arc) -> Self {
        Curve::from(arc).into()
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
