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
        self.scale(Scale2::new(radius, radius));
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
            start_angle,
            end_angle,
        }: Arc,
    ) -> Self {
        let span = (end_angle + 2. * PI - start_angle) % (2. * PI);

        if (span - 2. * PI).abs() < 1e-6 {
            Curve::from(Circle { local_transform })
        } else {
            // From https://ecridge.com/bezier.pdf
            let curves = (span / FRAC_PI_2).ceil();
            let span_per_curve = span / curves;

            let mut arcs = vec![];
            for c in 0..(curves as u32) {
                let start = (start_angle + (c as f32) * span_per_curve) % 360.;

                let alpha = span_per_curve / 2.;

                let x3 = alpha.cos(); //D
                let y3 = alpha.sin(); //D

                let x2 = (4. - x3) / 3.; //C = λx + μy
                let y2 = y3 + 4. / 3. * (x3 - 1.) * x3 / y3;

                let x1 = x2; //B
                let y1 = -y2; //B

                let x0 = x3; //A
                let y0 = -y3; //A

                let rot = Rotation2::new(alpha + start);

                arcs.push(
                    Bezier {
                        start: if arcs.is_empty() {
                            Some(rot * Point2::new(x0, y0))
                        } else {
                            None
                        },
                        start_control: rot * Point2::new(x1, y1),
                        end_control: rot * Point2::new(x2, y2),
                        end: rot * Point2::new(x3, y3),
                    }
                    .into(),
                );
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
