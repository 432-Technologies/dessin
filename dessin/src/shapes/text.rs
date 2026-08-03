/// Font storage
pub mod font;

use crate::prelude::*;
use font::FontRef;
pub use fontdb::{Style as FontStyle, Weight as FontWeight};
use na::{Point2, Vector2};
use nalgebra::{self as na, Transform2};

/// TextAlign
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
	#[default]
	/// Left
	Left,
	/// Center
	Center,
	/// Right
	Right,
}

/// TextVerticalAlign
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextVerticalAlign {
	#[default]
	/// Bottom
	Bottom,
	/// Center
	Center,
	/// Top
	Top,
}

#[derive(Debug, Clone, PartialEq, Shape)]
///
pub struct Text {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	#[shape(into)]
	///
	pub text: String,

	///
	pub align: TextAlign,

	///
	pub vertical_align: TextVerticalAlign,

	///
	pub weight: FontWeight,
	///
	pub style: FontStyle,

	#[shape(into, some, option_fn)]
	///
	pub on_curve: Option<Curve>,

	///
	pub font_size: f32,

	#[shape(into, some, option_fn)]
	///
	pub font: Option<FontRef>,
}
impl Default for Text {
	fn default() -> Self {
		Text {
			text: Default::default(),
			local_transform: Default::default(),
			align: Default::default(),
			vertical_align: Default::default(),
			weight: Default::default(),
			style: Default::default(),
			on_curve: Default::default(),
			font_size: 10.,
			font: Default::default(),
		}
	}
}

impl From<Text> for Shape {
	fn from(
		Text {
			local_transform,
			text,
			align,
			vertical_align,
			weight,
			style,
			on_curve,
			font_size,
			font,
		}: Text,
	) -> Self {
		let Some(font) = FontRef::or_default(font.clone()) else {
			return Default::default();
		};

		let size = font::font_holder_mut(|v| {
			let font_system = &mut v.0;

			let mut buffer = cosmic_text::Buffer::new(
				font_system,
				cosmic_text::Metrics::relative(font_size, 1.),
			);

			buffer.set_text(
				&text,
				&cosmic_text::Attrs {
					family: fontdb::Family::Name(&*font.family),
					weight,
					style,
					..cosmic_text::Attrs::new()
				},
				cosmic_text::Shaping::Advanced,
				None,
			);

			buffer.shape_until_scroll(font_system, true);
			let cosmic_text::LayoutRun {
				line_y,
				line_top,
				line_height,
				line_w,
				..
			} = buffer.layout_runs().next()?;

			// In cosmic_text (Y down):
			// - line_top is the Y position of the top of the line (smaller = higher)
			// - line_y is the Y position of the baseline
			// - line_height is the distance to the next baseline
			//
			// Distances from baseline:
			// - Ascender height (baseline to top) = line_y - line_top
			// - Descender height (baseline to bottom) ≈ line_height - (line_y - line_top)
			//
			// In dessin (Y up), with baseline at baseline_y:
			// - Top of text = baseline_y + (line_y - line_top)  // above baseline
			// - Bottom of text = baseline_y - (line_height - (line_y - line_top))  // below baseline
			let ascender = line_y - line_top;
			let descender = line_height - ascender;

			Some((line_w, ascender - descender))
		});

		let Some((width, height)) = size else {
			return Default::default();
		};

		Shape::Text(TextShape {
			local_transform,
			text,
			align,
			vertical_align,
			weight,
			style,
			on_curve,
			font_size,
			font,
			width,
			height,
		})
	}
}

///
#[derive(Debug, Clone, PartialEq)]
pub struct TextShape {
	///
	pub local_transform: Transform2<f32>,

	///
	pub text: String,

	///
	pub align: TextAlign,

	///
	pub vertical_align: TextVerticalAlign,

	///
	pub weight: FontWeight,
	///
	pub style: FontStyle,

	///
	pub on_curve: Option<Curve>,

	///
	pub font_size: f32,

	///
	pub font: FontRef,

	pub width: f32,
	pub height: f32,
}
impl TextShape {
	pub fn position<'a>(&'a self, parent_transform: &Transform2<f32>) -> TextPosition<'a> {
		let bounding_box = self.global_bounding_box(parent_transform);

		let transform = parent_transform * self.local_transform;
		let font_size = self.font_size * (transform * Vector2::new(0., 1.)).magnitude();

		let rot_dir = bounding_box.top_right() - bounding_box.top_left();
		let rotation = rot_dir.y.atan2(rot_dir.x);

		TextPosition {
			text: &self.text,
			weight: self.weight,
			style: self.style,
			on_curve: self.on_curve.as_ref().map(|v| v.position(&transform)),
			font_size,
			reference_start: bounding_box.bottom_left,
			bounding_box,
			rotation,
			font: &self.font,
		}
	}
}
impl ShapeOp for TextShape {
	fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
		self.local_transform = transform_matrix * self.local_transform;
		self
	}

	fn local_transform(&self) -> &Transform2<f32> {
		&self.local_transform
	}
}
impl ShapeBoundingBox for TextShape {
	fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
		let (min_x, max_x) = match self.align {
			TextAlign::Left => (0., self.width),
			TextAlign::Center => (-self.width / 2., self.width / 2.),
			TextAlign::Right => (-self.width, 0.),
		};

		let (min_y, max_y) = match self.vertical_align {
			TextVerticalAlign::Bottom => (0., self.height),
			TextVerticalAlign::Center => (-self.height / 2., self.height / 2.),
			TextVerticalAlign::Top => (-self.height, 0.),
		};

		BoundingBox::mins_maxs(min_x, min_y, max_x, max_y).transform(&self.local_transform)
	}
}

pub struct TextPosition<'a> {
	pub text: &'a str,
	pub weight: FontWeight,
	pub style: FontStyle,
	pub on_curve: Option<CurvePosition>,
	pub font_size: f32,
	pub reference_start: Point2<f32>,
	pub rotation: f32,
	pub bounding_box: BoundingBox<UnParticular>,
	pub font: &'a FontRef,
}

#[cfg(test)]
mod tests {
	use crate::{
		export::{Export, Exporter},
		prelude::*,
	};
	use nalgebra::{Point2, Rotation2};
	use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

	#[test]
	fn rotate_group() {
		let dessin = dessin!(
			[
				Circle(translate = [0., 25.]),
				Text(
					text = "1",
					font_size = 30.,
					vertical_align = TextVerticalAlign::Center,
					translate = [0., 25.],
				),
				Text(
					text = "2",
					font_size = 40.,
					vertical_align = TextVerticalAlign::Center,
					translate = [0., 0.]
				),
				Text(
					text = "3",
					font_size = 15.,
					vertical_align = TextVerticalAlign::Center,
					translate = [0., -30.]
				),
			] > (rotate = Rotation2::new(FRAC_PI_4))
		);

		struct Exp;
		impl Exporter for Exp {
			type Error = ();

			fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
				Ok(())
			}

			fn end_style(&mut self) -> Result<(), Self::Error> {
				Ok(())
			}

			fn export_image(&mut self, _image: ImagePosition) -> Result<(), Self::Error> {
				Ok(())
			}

			fn export_ellipse(
				&mut self,
				ellipse: EllipsePosition,
				_style: StylePosition,
			) -> Result<(), Self::Error> {
				let expected_position = Point2::new(-25. * FRAC_1_SQRT_2, 25. * FRAC_1_SQRT_2);
				assert!(
					(ellipse.center - expected_position).magnitude() < 10e-6,
					"left = {}, right = {}",
					ellipse.center,
					expected_position,
				);

				Ok(())
			}

			fn export_curve(
				&mut self,
				_curve: CurvePosition,
				StylePosition { fill: _, stroke: _ }: StylePosition,
			) -> Result<(), Self::Error> {
				Ok(())
			}

			fn export_text(
				&mut self,
				text: TextPosition,
				_style: StylePosition,
			) -> Result<(), Self::Error> {
				match text.text {
					"1" => {
						let expected_position =
							Point2::new(-25. * FRAC_1_SQRT_2, 25. * FRAC_1_SQRT_2);
						assert!(
							(text.reference_start - expected_position).magnitude() < 10e-6,
							"left = {}, right = {}",
							text.reference_start,
							expected_position,
						);
					}
					"2" => {
						let expected_position = Point2::new(0., 0.);
						assert!(
							(text.reference_start - expected_position).magnitude() < 10e-6,
							"left = {}, right = {}",
							text.reference_start,
							expected_position,
						);
					}
					"3" => {
						let expected_position =
							Point2::new(30. * FRAC_1_SQRT_2, -30. * FRAC_1_SQRT_2);
						assert!(
							(text.reference_start - expected_position).magnitude() < 10e-6,
							"left = {}, right = {}",
							text.reference_start,
							expected_position,
						);
					}
					_ => {}
				}

				Ok(())
			}
		}

		dessin
			.write_into_exporter(
				&mut Exp,
				&Default::default(),
				StylePosition {
					fill: None,
					stroke: None,
				},
			)
			.unwrap();
	}
}
