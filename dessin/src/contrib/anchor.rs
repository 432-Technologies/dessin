use crate::prelude::{Shape, ShapeBoundingBox, ShapeOp, ShapeOpWith};
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use std::ops::{Deref, DerefMut};

/// Place a shape relative to its anchor points.
///
/// Relative to its center, an anchor of:
/// - [0, 0] places its content in the center
/// - [1, 1] places its content in the bottom right
/// and so on
#[derive(Debug, Clone, PartialEq, Shape)]
pub struct Anchor<T> {
	/// Holded shape
	pub shape: T,
	/// Anchor point
	#[shape(into)]
	pub anchor: Vector2<f32>,
}
impl<T> Default for Anchor<T>
where
	T: Default,
{
	fn default() -> Self {
		Anchor {
			shape: T::default(),
			anchor: Vector2::zeros(),
		}
	}
}
impl<T> Anchor<T> {
	/// Wrap a [`Shape`] with Anchor
	#[inline]
	pub fn new(shape: T) -> Self {
		Anchor {
			shape,
			anchor: Vector2::default(),
		}
	}
}

impl<T> Deref for Anchor<T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.shape
	}
}

impl<T> DerefMut for Anchor<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.shape
	}
}

impl<T> From<Anchor<T>> for Shape
where
	T: Into<Shape>,
{
	fn from(Anchor { shape, anchor }: Anchor<T>) -> Self {
		let shape: Shape = shape.into();

		let bb = shape.local_bounding_box().straigthen();

		let width = bb.width() / 2.;
		let height = bb.height() / 2.;

		let translate_x = -anchor.x * width;
		let translate_y = -anchor.y * height;

		shape.with_translate(Translation2::new(translate_x, translate_y))
	}
}

impl<T: ShapeOp> ShapeOp for Anchor<T> {
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

#[cfg(test)]
mod tests {

	use crate::prelude::*;
	use ::image::DynamicImage;
	use nalgebra::{Point2, Transform2, Translation2};

	#[test]
	fn base() {
		let Shape::Image(img) = Shape::from(dessin!(Anchor::<Image>())) else {
			unreachable!()
		};

		let empty_image = DynamicImage::default();

		assert_eq!(
			img.position(&Transform2::default()),
			ImagePosition {
				center: Point2::origin(),
				top_left: Point2::new(-0.5, 0.5),
				top_right: Point2::new(0.5, 0.5),
				bottom_right: Point2::new(0.5, -0.5),
				bottom_left: Point2::new(-0.5, -0.5),
				width: 1.,
				height: 1.,
				rotation: 0.,
				image: &empty_image,
			}
		);
	}

	#[test]
	fn anchor() {
		let Shape::Image(img) = Shape::from(dessin!(Anchor::<Image>(anchor = [1., 1.]))) else {
			unreachable!()
		};

		let empty_image = DynamicImage::default();

		assert_eq!(
			img.position(&Transform2::default()),
			ImagePosition {
				center: Point2::new(-0.5, -0.5),
				top_left: Point2::new(-1., 0.),
				top_right: Point2::new(0., 0.),
				bottom_right: Point2::new(0., -1.),
				bottom_left: Point2::new(-1., -1.),
				width: 1.,
				height: 1.,
				rotation: 0.,
				image: &empty_image,
			}
		);
	}

	#[test]
	fn translate() {
		let Shape::Image(img) = Shape::from(dessin!(Anchor::<Image>(anchor = [1., 1.]))) else {
			unreachable!()
		};

		let empty_image = DynamicImage::default();
		let translation: Transform2<f32> = nalgebra::convert(Translation2::new(15., 13.));

		assert_eq!(
			img.position(&translation),
			ImagePosition {
				center: Point2::new(14.5, 12.5),
				top_left: Point2::new(14., 13.),
				top_right: Point2::new(15., 13.),
				bottom_right: Point2::new(15., 12.),
				bottom_left: Point2::new(14., 12.),
				width: 1.,
				height: 1.,
				rotation: 0.,
				image: &empty_image,
			}
		);
	}
}
