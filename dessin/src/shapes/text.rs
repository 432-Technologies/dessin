/// Font storage
pub mod font;

use crate::prelude::*;
use font::FontRef;
pub use fontdb::{Style as FontStyle, Weight as FontWeight};
use na::{Point2, Unit, Vector2};
use nalgebra::{self as na, Transform2};

pub(crate) struct TextSize<'a> {
	font_ref: font::FontRef,
	weight: FontWeight,
	style: FontStyle,
	text: &'a str,
	font_size: f32,
	line_height_scale: f32,
}
impl<'a> TextSize<'a> {
	pub(crate) fn new(font_ref: font::FontRef) -> Self {
		Self {
			font_ref,
			weight: FontWeight::NORMAL,
			style: FontStyle::Normal,
			text: "",
			font_size: 10.,
			line_height_scale: 1.,
		}
	}
	pub(crate) fn weight(mut self, weight: FontWeight) -> Self {
		self.weight = weight;
		self
	}
	pub(crate) fn style(mut self, style: FontStyle) -> Self {
		self.style = style;
		self
	}
	pub(crate) fn font_size(mut self, font_size: f32) -> Self {
		self.font_size = font_size;
		self
	}
	pub(crate) fn line_height_scale(mut self, line_height_scale: f32) -> Self {
		self.line_height_scale = line_height_scale;
		self
	}
	pub(crate) fn text(mut self, text: &'a str) -> Self {
		self.text = text;
		self
	}
	pub(crate) fn set_text(&mut self, text: &'a str) {
		self.text = text;
	}
	pub(crate) fn compute_width(&self) -> Option<TextRun> {
		font::font_holder_mut(|v| {
			let font_system = &mut v.0;

			let mut buffer = cosmic_text::Buffer::new(
				font_system,
				cosmic_text::Metrics::relative(self.font_size, self.line_height_scale),
			);

			buffer.set_text(
				self.text,
				&cosmic_text::Attrs {
					// family: fontdb::Family::Name(()),
					..cosmic_text::Attrs::new()
				},
				cosmic_text::Shaping::Advanced,
				None,
			);

			buffer.shape_until_scroll(font_system, true);
			buffer.layout_runs().next().map(|v| TextRun {
				line_y: v.line_y,
				line_top: v.line_top,
				line_height: v.line_height,
				line_w: v.line_w,
			})
		})
	}
}

pub(crate) struct TextRun {
	/// Y offset to baseline of line
	pub line_y: f32,
	/// Y offset to top of line
	pub line_top: f32,
	/// Y offset to next line
	pub line_height: f32,
	/// Width of line
	pub line_w: f32,
}

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

///
pub struct TextPosition<'a> {
	///
	pub text: &'a str,
	///
	pub align: TextAlign,
	///
	pub weight: FontWeight,
	///
	pub style: FontStyle,
	///
	pub on_curve: Option<CurvePosition>,
	///
	pub font_size: f32,
	///
	pub reference_start: Point2<f32>,
	///
	pub direction: Unit<Vector2<f32>>,
	///
	pub font: &'a Option<FontRef>,
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
impl Text {
	///
	pub fn position<'a>(&'a self, parent_transform: &Transform2<f32>) -> TextPosition<'a> {
		let transform = self.global_transform(parent_transform);

		let font_size = self.font_size * (transform * Vector2::new(0., 1.)).magnitude();
		let reference_start = transform
			* Point2::new(
				0.,
				match self.vertical_align {
					TextVerticalAlign::Bottom => font_size / 2.,
					TextVerticalAlign::Center => 0.,
					TextVerticalAlign::Top => -font_size / 2.,
				},
			);

		TextPosition {
			text: &self.text,
			align: self.align,
			weight: self.weight,
			style: self.style,
			on_curve: self.on_curve.as_ref().map(|v| v.position(&transform)),
			font_size,
			reference_start,
			direction: Unit::new_normalize(transform * Vector2::new(1., 0.)),
			font: &self.font,
		}
	}
}

impl From<Text> for Shape {
	fn from(v: Text) -> Self {
		Shape::Text(v)
	}
}

impl ShapeBoundingBox for Text {
	fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
		let Some(font_ref) = self
			.font
			.as_ref()
			.or_else(|| crate::font::default_font())
			.cloned()
		else {
			return BoundingBox::zero().as_unparticular();
		};

		let Some(text_run) = TextSize::new(font_ref)
			.font_size(self.font_size)
			.text(&self.text)
			.weight(self.weight)
			.style(self.style)
			.compute_width()
		else {
			return BoundingBox::zero().as_unparticular();
		};

		let TextRun {
			line_y,
			line_top,
			line_height,
			line_w: width,
		} = text_run;

		// Baseline Y offset matches `position()` - the text baseline is NOT at the origin
		let baseline_y = match self.vertical_align {
			TextVerticalAlign::Bottom => self.font_size / 2.,
			TextVerticalAlign::Center => 0.,
			TextVerticalAlign::Top => -self.font_size / 2.,
		};

		let (left, right) = match self.align {
			TextAlign::Left => (0., width),
			TextAlign::Center => (-width / 2., width / 2.),
			TextAlign::Right => (width, 0.),
		};

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
		let top = baseline_y + ascender;
		let bottom = baseline_y - descender;

		BoundingBox::new(
			[left, top].into(),
			[right, top].into(),
			[right, bottom].into(),
			[left, bottom].into(),
		)
		.transform(self.local_transform())
	}
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
