use crate::prelude::*;
use nalgebra::{Point2, Transform2};

// create a struct which will be composed by 3 vectors (3 points of the vertex of the triangle)
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Triangle {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	///size of the side following the x axis
	pub width_x_axis: f32,

	///size of the side following the angle axis
	pub size_axis_angle: f32,

	///angle between the 2 side created before
	pub angle: f32,
}

impl From<Triangle> for Curve {
	//Create "from" where we note what we will ask when using Triangle
	fn from(
		Triangle {
			local_transform,
			width_x_axis,
			size_axis_angle,
			angle,
		}: Triangle,
	) -> Self {
		let origin = Point2::new(0., 0.); //create one point at the origin
		let base = Point2::new(width_x_axis, 0.); //create a second point on the x axis at a distance width_x_axis
		let top = Point2::new(size_axis_angle * angle.cos(), size_axis_angle * angle.sin());

		dessin! {
			Curve(
				transform = local_transform,
				then = origin,
				then = base,
				then = top,
				closed
			)
		}
	}
}

impl From<Triangle> for Shape {
	fn from(v: Triangle) -> Self {
		Curve::from(v).into()
	}
}
