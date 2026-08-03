use super::{BoundingBox, ShapeBoundingBox, UnParticular};
use crate::shapes::{Shape, ShapeOp};
use image::DynamicImage;
use nalgebra::{Point2, Scale2, Transform2};

#[derive(Debug, Clone, PartialEq)]
///
pub struct ImagePosition<'a> {
	///
	pub bounding_box: BoundingBox<UnParticular>,

	///
	pub rotation: f32,

	///
	pub image: &'a DynamicImage,
}

#[derive(Default, Debug, Clone, PartialEq)]
///
pub struct Image {
	///
	pub image: DynamicImage,
	///
	pub local_transform: Transform2<f32>,
}
impl Image {
	#[inline]
	///
	pub fn image_size_pixel(&self) -> (u32, u32) {
		(self.image.width(), self.image.height())
	}

	#[inline]
	///
	pub fn aspect_ratio(&self) -> f32 {
		let (w, h) = self.image_size_pixel();
		w as f32 / h as f32
	}

	///
	pub fn image(&mut self, image: DynamicImage) -> &mut Self {
		self.image = image;
		self
	}

	#[inline]
	///
	pub fn with_image(mut self, image: DynamicImage) -> Self {
		self.image(image);
		self
	}

	///
	pub fn keep_aspect_ratio(&mut self) -> &mut Self {
		self.scale(Scale2::new(self.aspect_ratio(), 1.));
		self
	}

	#[inline]
	///
	pub fn with_keep_aspect_ratio(mut self) -> Self {
		self.keep_aspect_ratio();
		self
	}

	///
	pub fn position<'a>(&'a self, parent_transform: &Transform2<f32>) -> ImagePosition<'a> {
		let bounding_box = self.global_bounding_box(parent_transform);

		let rot_dir = bounding_box.top_right() - bounding_box.top_left();
		let rotation = rot_dir.y.atan2(rot_dir.x);

		ImagePosition {
			bounding_box,
			rotation,
			image: &self.image,
		}
	}
}

impl From<Image> for Shape {
	#[inline]
	fn from(v: Image) -> Self {
		Shape::Image(v)
	}
}

impl ShapeOp for Image {
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

impl ShapeBoundingBox for Image {
	fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
		let top_left = self.local_transform * Point2::new(-0.5, 0.5);
		let top_right = self.local_transform * Point2::new(0.5, 0.5);
		let bottom_right = self.local_transform * Point2::new(0.5, -0.5);
		let bottom_left = self.local_transform * Point2::new(-0.5, -0.5);

		BoundingBox::new(top_left, top_right, bottom_right, bottom_left)
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use ::image::DynamicImage;
	use assert_float_eq::*;
	use nalgebra::{Point2, Rotation2, Scale2, Transform2, Translation2};
	use std::f32::consts::SQRT_2;

	#[test]
	fn base() {
		let img = dessin!(Image());

		assert_eq!(
			img.position(&Transform2::default()).bounding_box,
			BoundingBox::mins_maxs(-0.5, -0.5, 0.5, 0.5).as_unparticular(),
		);
	}

	#[test]
	fn bounding_box() {
		let img = dessin!(Image());
		let bb = img.local_bounding_box();

		assert_eq!(bb.width(), 1.);
		assert_eq!(bb.height(), 1.);

		assert_eq!(
			bb,
			BoundingBox::new(
				Point2::new(-0.5, 0.5),
				Point2::new(0.5, 0.5),
				Point2::new(0.5, -0.5),
				Point2::new(-0.5, -0.5),
			)
		);
	}

	#[test]
	fn local_transform() {
		let img = dessin!(Image(rotate = Rotation2::new(-45_f32.to_radians())));
		let img_pos = img.position(&Transform2::default());

		assert_f32_near!(img_pos.rotation, -45_f32.to_radians());
		assert_f32_near!(img_pos.bounding_box.width(), 1.);
		assert_f32_near!(img_pos.bounding_box.left(), Point2::new(0., SQRT_2 / 2.).x);
		assert_f32_near!(img_pos.bounding_box.top(), Point2::new(0., SQRT_2 / 2.).y);
	}

	#[test]
	fn global_transform() {
		let img = dessin!(Image());
		let parent_transform = Transform2::default() * Rotation2::new(-45_f32.to_radians());
		let img_pos = img.position(&parent_transform);

		assert_f32_near!(img_pos.rotation, -45_f32.to_radians());
		assert_f32_near!(img_pos.bounding_box.width(), 1.);
		assert_f32_near!(img_pos.bounding_box.left(), Point2::new(0., SQRT_2 / 2.).x);
		assert_f32_near!(img_pos.bounding_box.top(), Point2::new(0., SQRT_2 / 2.).y);
	}

	#[test]
	fn combined_transform() {
		let img = dessin!(Image());
		let img_pos = img.position(&Transform2::default());
		let empty_image = DynamicImage::default();
		println!("Base = {img_pos:?}\n");

		assert_eq!(
			img_pos,
			ImagePosition {
				bounding_box: BoundingBox::new(
					Point2::new(-0.5, 0.5),
					Point2::new(0.5, 0.5),
					Point2::new(0.5, -0.5),
					Point2::new(-0.5, -0.5),
				),
				rotation: 0.,
				image: &empty_image,
			}
		);

		let img = dessin!({ img }(rotate = Rotation2::new(-45_f32.to_radians())));
		let img_pos = img.position(&Transform2::default());
		println!("Rot(-45deg) = {img_pos:?}\n");
		assert_f32_near!(img_pos.rotation, -45_f32.to_radians());
		assert_f32_near!(img_pos.bounding_box.width(), 1.);
		assert_f32_near!(img_pos.bounding_box.left(), Point2::new(0., SQRT_2 / 2.).x);
		assert_f32_near!(img_pos.bounding_box.top(), Point2::new(0., SQRT_2 / 2.).y);

		let img = dessin!({ img }(translate = Translation2::new(1., 0.)));
		let img_pos = img.position(&Transform2::default());
		println!("Translate_x(1) = {img_pos:?}\n");
		assert_f32_near!(img_pos.rotation, -45_f32.to_radians());
		assert_f32_near!(img_pos.bounding_box.width(), 1.);
		assert_f32_near!(img_pos.bounding_box.left(), Point2::new(1., SQRT_2 / 2.).x);
		assert_f32_near!(img_pos.bounding_box.top(), Point2::new(1., SQRT_2 / 2.).y);
		assert_f32_near!(
			img_pos.bounding_box.right(),
			Point2::new(SQRT_2 / 2. + 1., 0.).x
		);
		assert_f32_near!(
			img_pos.bounding_box.top(),
			Point2::new(SQRT_2 / 2. + 1., 0.).y
		);

		let img = dessin!({ img }(scale = Scale2::new(3., 2.)));
		let img_pos = img.position(&Transform2::default());
		println!("Scale(3, 2) =img_pos:?\n");
		assert_f32_near!(
			img_pos.bounding_box.left(),
			Point2::new(3. * 1., 2. * SQRT_2 / 2.).x
		);
		assert_f32_near!(
			img_pos.bounding_box.top(),
			Point2::new(3. * 1., 2. * SQRT_2 / 2.).y
		);
		assert_f32_near!(
			img_pos.bounding_box.right(),
			Point2::new(3. * (SQRT_2 / 2. + 1.), 2. * 0.).x
		);
		assert_f32_near!(
			img_pos.bounding_box.top(),
			Point2::new(3. * (SQRT_2 / 2. + 1.), 2. * 0.).y
		);
	}
}
