use crate::prelude::*;
use core::f32::consts::PI;
use nalgebra::Transform2;

#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct ThickArc {
    #[local_transform]
    pub local_transform: Transform2<f32>,
    /// start angle in radian
    pub start_angle: f32,
    /// end angle in radian
    pub end_angle: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
}
impl ThickArc {
    pub fn span_angle(&mut self, span_angle: f32) -> &mut Self {
        self.end_angle = (self.start_angle + span_angle) % (2. * PI);
        self
    }
    pub fn with_span_angle(mut self, span_angle: f32) -> Self {
        self.end_angle((self.start_angle + span_angle) % (2. * PI));
        self
    }
}

impl From<ThickArc> for Shape {
    fn from(
        ThickArc {
            local_transform,
            start_angle,
            end_angle,
            inner_radius,
            outer_radius,
        }: ThickArc,
    ) -> Self {
        dessin!(Curve: (
            transform={local_transform}
            then={Curve::from(Arc {start_angle, end_angle, ..Default::default()}.with_radius(outer_radius))}
            then={Curve::from(Arc {start_angle, end_angle, ..Default::default()}.with_radius(inner_radius)).reversed()}
			closed
        ))
        .into()
    }
}
