use crate::prelude::*;
use nalgebra::{Point2, Rotation2, Scale2, Transform2};
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Arc {
    pub local_transform: Transform2<f32>,
    /// start angle in radian
    pub start_angle: f32,
    /// end angle in radian
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
            while span > 1e-6 {
                let rot = Rotation2::new(start);
                let end_rot = Rotation2::new(start + (FRAC_PI_2 - span.min(FRAC_PI_2)));

                arcs.push(
                    Bezier {
                        start: if arcs.is_empty() {
                            Some(rot * Point2::new(0.5, 0.))
                        } else {
                            None
                        },
                        start_control: rot * Point2::new(0.5, 0.552284749831 / 2.),
                        end_control: end_rot * Point2::new(0.552284749831 / 2., 0.5),
                        end: end_rot * Point2::new(0., 0.5),
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
