use crate::prelude::*;
use core::f32::consts::PI;
use nalgebra::Transform2;

/// An arc with a thickness
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct ThickArc {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	/// start angle in radian
	pub start_angle: f32,

	/// end angle in radian
	pub end_angle: f32,

	/// Width of the inner radius
	pub inner_radius: f32,

	/// Width of the outer radius
	pub outer_radius: f32,
}
impl ThickArc {
	/// End angle from a span
	pub fn span_angle(&mut self, span_angle: f32) -> &mut Self {
		self.end_angle = (self.start_angle + span_angle) % (2. * PI);
		self
	}
	/// End angle from a span
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
		dessin!(Curve(
			transform = local_transform,
			then = Curve::from(
				Arc {
					start_angle,
					end_angle,
					..Default::default()
				}
				.with_radius(outer_radius),
			),
			then = Curve::from(
				Arc {
					start_angle,
					end_angle,
					..Default::default()
				}
				.with_radius(inner_radius),
			)
			.reversed(),
			closed,
		))
		.into()
	}
}
