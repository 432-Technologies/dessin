use crate::prelude::*;
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use palette::{IntoColor, Srgba};
use std::{
	f32::consts::FRAC_1_SQRT_2,
	ops::{Deref, DerefMut, Mul},
};

/// Calculated result style
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StylePosition {
	pub stroke: Option<Stroke>,
	pub fill: Option<Fill>,
}

/// `Stroke`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stroke {
	/// Solid line
	Solid { color: Srgba, width: f32 },

	/// Dashed line
	Dashed {
		color: Srgba,
		width: f32,
		on: f32,
		off: f32,
	},
}
impl Stroke {
	/// Solid line
	pub fn new_full<F: IntoColor<Srgba>>(color: F, width: f32) -> Self {
		let color = color.into_color();
		Self::Solid { color, width }
	}

	/// Dashed line
	pub fn new_dashed<F: IntoColor<Srgba>>(color: F, width: f32, on: f32, off: f32) -> Self {
		let color = color.into_color();
		Self::Dashed {
			color,
			width,
			on,
			off,
		}
	}
}

impl<C: IntoColor<Srgba>> From<(C, f32)> for Stroke {
	fn from((color, width): (C, f32)) -> Self {
		Stroke::new_full(color, width)
	}
}

impl Mul<Stroke> for Transform2<f32> {
	type Output = Stroke;

	fn mul(self, rhs: Stroke) -> Self::Output {
		match rhs {
			Stroke::Solid { color, width } => Stroke::Solid {
				color,
				width: (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude() * width,
			},
			Stroke::Dashed {
				color,
				width,
				on,
				off,
			} => {
				let factor = (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude();

				Stroke::Dashed {
					color,
					width: width * factor,
					on: on * factor,
					off: off * factor,
				}
			}
		}
	}
}

/// `Stroke`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
	/// Solid fill
	Solid { color: Srgba },
}
impl Stroke {}

impl<C: IntoColor<Srgba>> From<C> for Fill {
	fn from(color: C) -> Self {
		Fill::Solid {
			color: color.into_color(),
		}
	}
}

impl Mul<Fill> for Transform2<f32> {
	type Output = Fill;

	fn mul(self, rhs: Fill) -> Self::Output {
		match rhs {
			Fill::Solid { color } => Fill::Solid { color },
		}
	}
}

#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Style<T> {
	/// Wrapped `Shape`
	pub shape: T,

	/// Add a fill color
	#[shape(into_some)]
	pub fill: Option<Fill>,

	/// Add a `Stroke`
	#[shape(into_some)]
	pub stroke: Option<Stroke>,
}
impl<T> Style<T> {
	/// Create a new `Style`
	#[inline]
	pub fn new(shape: T) -> Self {
		Style {
			shape,
			fill: None,
			stroke: None,
		}
	}
}

impl<T> Deref for Style<T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.shape
	}
}

impl<T> DerefMut for Style<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.shape
	}
}

impl<T: Into<Shape>> From<Style<T>> for Shape {
	#[inline]
	fn from(
		Style {
			shape,
			fill,
			stroke,
		}: Style<T>,
	) -> Self {
		if fill.is_none() && stroke.is_none() {
			shape.into()
		} else {
			Shape::Style {
				fill,
				stroke,
				shape: Box::new(shape.into()),
			}
		}
	}
}

impl<T: ShapeOp> ShapeOp for Style<T> {
	#[inline]
	fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
		self.shape.transform(transform_matrix);
		self
	}

	#[inline]
	fn translate<U: Into<Translation2<f32>>>(&mut self, translation: U) -> &mut Self {
		self.shape.translate(translation);
		self
	}

	#[inline]
	fn scale<S: Into<Scale2<f32>>>(&mut self, scale: S) -> &mut Self {
		self.shape.scale(scale);
		self
	}

	#[inline]
	fn rotate<R: Into<Rotation2<f32>>>(&mut self, rotation: R) -> &mut Self {
		self.shape.rotate(rotation);
		self
	}

	#[inline]
	fn local_transform(&self) -> &Transform2<f32> {
		self.shape.local_transform()
	}

	#[inline]
	fn global_transform(&self, parent_transform: &Transform2<f32>) -> Transform2<f32> {
		self.shape.global_transform(parent_transform)
	}
}

impl<T: ShapeBoundingBox> ShapeBoundingBox for Style<T> {
	#[inline]
	fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
		self.shape.local_bounding_box()
	}
}
