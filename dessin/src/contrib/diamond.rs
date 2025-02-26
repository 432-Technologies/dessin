use crate::prelude::*;
use nalgebra::{Point2, Transform2};

#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Diamond {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	///diamond width following the x axis
	pub width: f32,

	///size between the origin and the diamond top apex following the y axis
	pub height_top: f32,

	///size between the origin and the diamond bottom apex following the y axis
	pub height_bottom: f32,
}

// create a struct which will be composed by 4 vectors (3 points of the vertex of the diamond)
impl From<Diamond> for Curve {
	//Create "from" where we note what we will ask when using Diamond
	fn from(
		Diamond {
			local_transform,
			width,
			height_top,
			height_bottom,
		}: Diamond,
	) -> Self {
		let right = Point2::new(width / 2.0, 0.);
		let top = Point2::new(0., height_top);
		let left = Point2::new(-width / 2.0, 0.);
		let bottom = Point2::new(0., -height_bottom);

		dessin!(Curve(
			transform = local_transform,
			then = right,
			then = top,
			then = left,
			then = bottom,
			closed,
		))
	}
}

impl From<Diamond> for Shape {
	fn from(v: Diamond) -> Self {
		Curve::from(v).into()
	}
}
