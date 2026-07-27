use crate::{font::FontRef, prelude::*};
use nalgebra::{Transform2, Translation2};

/// Box of text, with auto wrapping text if width is too large
#[derive(Debug, Clone, PartialEq, Shape)]
pub struct TextBox {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	/// Font size
	pub font_size: f32,

	/// Spacing between each line
	pub line_height_scale: f32,

	/// Horizontal align
	pub align: TextAlign,

	/// Vertical align
	pub vertical_align: TextVerticalAlign,

	/// The text
	#[shape(into)]
	pub text: String,

	/// Font weight
	pub weight: FontWeight,
	///
	pub style: FontStyle,

	/// Dimension on the x-axis
	#[shape(some)]
	pub width: Option<f32>,

	/// Dimension on the y-axis
	#[shape(some)]
	pub height: Option<f32>,

	/// Font
	#[shape(into, some)]
	pub font: Option<FontRef>,
}
impl Default for TextBox {
	fn default() -> Self {
		TextBox {
			local_transform: Default::default(),
			font_size: Default::default(),
			line_height_scale: 1.,
			align: Default::default(),
			vertical_align: TextVerticalAlign::Top,
			text: Default::default(),
			weight: Default::default(),
			style: Default::default(),
			width: Default::default(),
			height: Default::default(),
			font: Default::default(),
		}
	}
}
impl TextBox {
	/// Remove height constraint (default)
	#[inline]
	pub fn no_height(&mut self) -> &mut Self {
		self.height = None;
		self
	}
	/// Remove height constraint (default)
	#[inline]
	pub fn without_weight(mut self) -> Self {
		self.no_height();
		self
	}
}

impl From<TextBox> for Shape {
	fn from(
		TextBox {
			local_transform,
			font_size,
			line_height_scale,
			text,
			width,
			height,
			align,
			vertical_align,
			weight,
			style,
			font,
		}: TextBox,
	) -> Self {
		let Some(font_ref) = FontRef::or_default(font) else {
			return Shape::default();
		};

		let mut total_height = 0f32;

		let texts = font::font_holder_mut(|v| {
			let font_system = &mut v.0;

			let mut buffer = cosmic_text::Buffer::new(
				font_system,
				cosmic_text::Metrics::relative(font_size, line_height_scale),
			);

			buffer.set_size(width, height);

			buffer.set_text(
				&text,
				&cosmic_text::Attrs {
					family: fontdb::Family::Name(&*font_ref.family),
					weight,
					style,
					..cosmic_text::Attrs::new()
				},
				cosmic_text::Shaping::Advanced,
				Some(match align {
					TextAlign::Left => cosmic_text::Align::Left,
					TextAlign::Center => cosmic_text::Align::Center,
					TextAlign::Right => cosmic_text::Align::Right,
				}),
			);

			buffer.shape_until_scroll(font_system, true);
			buffer
				.layout_runs()
				.map(|run| {
					let offset = (run.line_i as f32 + 1.) * run.line_height;

					total_height += run.line_height;

					dessin!(Text(
						text = run.text,
						font = font_ref.clone(),
						{ font_size },
						{ align },
						{ style },
						{ weight },
						translate = [0., -offset]
					))
				})
				.collect::<Vec<Text>>()
		});

		let translation_y = match vertical_align {
			TextVerticalAlign::Bottom => total_height,
			TextVerticalAlign::Center => total_height / 2.,
			TextVerticalAlign::Top => 0.,
		};

		Group::default()
			.with_shapes(texts.into_iter().map(Shape::from).collect())
			.with_translate(Translation2::from([0., translation_y]))
			.with_transform(local_transform)
			.into()
	}
}

#[test]
fn one_line() {
	use assert_float_eq::*;

	crate::font::add_font(include_bytes!("../Helvetica.otf"));
	crate::font::set_default_font(crate::font::get("Helvetica").unwrap());

	let text = "it should work, famous last word";

	let shape: Shape = dessin!(
		*TextBox(
			{ text },
			fill = palette::Srgb::<f32>::new(0., 0., 0.).into_linear(),
			font_size = 5.,
			align = TextAlign::Left,
			line_height_scale = 2.,
		) > ()
	);

	let bb = shape.local_bounding_box();
	assert_float_absolute_eq!(bb.height(), 5., 0.001);
}

#[test]
fn two_lines() {
	use assert_float_eq::*;

	crate::font::add_font(include_bytes!("../Helvetica.otf"));
	crate::font::set_default_font(crate::font::get("Helvetica").unwrap());

	let text = "it should work\nfamous last word";

	let shape: Shape = dessin!(*TextBox(
		{ text },
		fill = palette::Srgb::<f32>::new(0., 0., 0.).into_linear(),
		font_size = 5.,
		align = TextAlign::Left,
		line_height_scale = 1.,
	))
	.into();

	let bb = shape.local_bounding_box();

	assert_float_absolute_eq!(bb.height(), 12., 0.0001);
}

#[test]
fn should_break() {
	use assert_float_eq::*;
	use nalgebra::{convert, Translation2};

	crate::font::add_font(include_bytes!("../Helvetica.otf"));
	crate::font::set_default_font(crate::font::get("Helvetica").unwrap());

	let text = "it should work, famous last word";

	let mut shape: Shape = dessin!(
		TextBox(
			{ text },
			font_size = 5.,
			width = 40.,
			align = TextAlign::Left,
			line_height_scale = 1.
		) > ()
	);

	let shapes = shape.get_or_mutate_as_group().shapes.clone();
	assert_eq!(shapes.len(), 2);

	{
		let Shape::Text(text) = shapes[0].clone() else {
			unreachable!()
		};

		// The Y translation is -bb.top() where bb.top() = baseline_y + ascender
		// With font_size = 5.0: baseline_y = 2.5, ascender ≈ 4.39, so bb.top() ≈ 6.89
		let lt = convert::<_, Transform2<f32>>(Translation2::new(0., -6.8896484));

		// assert_eq!(
		// 	text,
		// 	Text {
		// 		local_transform: lt,
		// 		text: "it should work,".to_string(),
		// 		align: TextAlign::Left,
		// 		vertical_align: Default::default(),
		// 		weight: Default::default(),
		// 		style: Default::default(),
		// 		on_curve: None,
		// 		font_size: 5.,
		// 		font: crate::font::default_font().cloned(),
		// 		// width: 0.,
		// 		// height: 0.,
		// 	}
		// );
	}

	{
		let Shape::Text(text) = shapes[1].clone() else {
			unreachable!()
		};

		// Second line: -bb.top() - bb.height() ≈ -6.89 - 5.0 ≈ -11.89
		let lt = convert::<_, Transform2<f32>>(Translation2::new(0., -11.889648));

		// assert_eq!(
		// 	text,
		// 	Text {
		// 		local_transform: lt,
		// 		text: "famous last word".to_string(),
		// 		align: TextAlign::Left,
		// 		vertical_align: Default::default(),
		// 		weight: Default::default(),
		// 		style: Default::default(),
		// 		on_curve: None,
		// 		font_size: 5.,
		// 		font: crate::font::default_font().cloned()
		// 	}
		// );
	}

	let bb = shape.local_bounding_box();
	assert_float_absolute_eq!(bb.height(), 10., 0.001);
}
