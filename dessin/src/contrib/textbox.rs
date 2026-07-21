use crate::{font::FontRef, prelude::*};
use nalgebra::Transform2;

/// Box of text, with auto wrapping text if width is too large
#[derive(Debug, Clone, PartialEq, Shape)]
pub struct TextBox {
	/// [`ShapeOp`]
	#[local_transform]
	pub local_transform: Transform2<f32>,

	/// Font size
	pub font_size: f32,

	/// Spacing between each line
	pub line_spacing: f32,

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
	pub width: f32,

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
			line_spacing: Default::default(),
			align: Default::default(),
			vertical_align: TextVerticalAlign::Top,
			text: Default::default(),
			weight: Default::default(),
			style: Default::default(),
			width: f32::MAX,
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
			line_spacing,
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
		let Some(font_ref) = font
			.as_ref()
			.or_else(|| crate::font::default_font())
			.cloned()
		else {
			return Shape::default();
		};

		let mut lines = vec![];
		let mut height = height.unwrap_or(f32::MAX);

		let mut text_size = TextSize::new(font_ref.clone())
			.weight(weight)
			.style(style)
			.font_size(font_size)
			.line_height_scale((font_size + line_spacing) / font_size);

		for line in text.lines() {
			let mut len = 0.;
			let mut acc = String::new();

			if height - font_size < 0. {
				break;
			}

			for word in line.split_whitespace() {
				if word.is_empty() {
					continue;
				}

				text_size.set_text(word);
				let word_size = text_size.compute_width();

				if len + word_size > width {
					lines.push(std::mem::take(&mut acc));

					acc = word.to_owned();
					len = word_size;

					height -= font_size + line_spacing;
				} else {
					len += word_size;

					if !acc.is_empty() {
						acc.push(' ');
					}

					acc.push_str(word)
				}
			}

			if !acc.is_empty() {
				lines.push(acc)
			}
		}

		let (vertical_align, _) = match vertical_align {
			TextVerticalAlign::Bottom => (TextVerticalAlign::Top, 1.),
			TextVerticalAlign::Center => (TextVerticalAlign::Center, -1.),
			TextVerticalAlign::Top => (TextVerticalAlign::Bottom, -1.),
		};

		dessin!(
			VerticalLayout(
				extend = lines.into_iter().map(|text| {
					dessin!(Text(
						{ text },
						{ align },
						{ vertical_align },
						{ weight },
						{ font_size },
						font = font_ref.clone(),
					))
					.into()
				}),
				gap = line_spacing,
				transform = local_transform,
			) > ()
		)
	}
}

#[test]
fn one_line() {
	use assert_float_eq::*;

	let text = "it should work, famous last word";

	let shape: Shape = dessin!(
		*TextBox(
			{ text },
			fill = palette::Srgb::<f32>::new(0., 0., 0.).into_linear(),
			font_size = 5.,
			align = TextAlign::Left,
			line_spacing = 2.,
		) > ()
	);

	let bb = shape.local_bounding_box();
	assert_float_absolute_eq!(bb.height(), 5., 0.001);
}

#[test]
fn two_lines() {
	use assert_float_eq::*;

	let text = "it should work\nfamous last word";

	let shape: Shape = dessin!(*TextBox(
		{ text },
		fill = palette::Srgb::<f32>::new(0., 0., 0.).into_linear(),
		font_size = 5.,
		align = TextAlign::Left,
		line_spacing = 2.,
	))
	.into();

	let bb = shape.local_bounding_box();

	assert_float_absolute_eq!(bb.height(), 12., 0.0001);
}

#[test]
fn should_break() {
	use assert_float_eq::*;
	use nalgebra::{convert, Translation2};

	let text = "it should work, famous last word";

	let mut shape: Shape = dessin!(
		TextBox(
			{ text },
			font_size = 5.,
			width = 40.,
			align = TextAlign::Left,
		) > ()
	);

	let shapes = shape.get_or_mutate_as_group().shapes.clone();
	assert_eq!(shapes.len(), 2);

	{
		let Shape::Text(text) = shapes[0].clone() else {
			unreachable!()
		};

		let lt = convert::<_, Transform2<f32>>(Translation2::new(0., (5. / 2.) * -1.));

		assert_eq!(
			text,
			Text {
				local_transform: lt,
				text: "it should work,".to_string(),
				align: TextAlign::Left,
				vertical_align: Default::default(),
				weight: Default::default(),
				style: Default::default(),
				on_curve: None,
				font_size: 5.,
				font: None
			}
		);
	}

	{
		let Shape::Text(text) = shapes[1].clone() else {
			unreachable!()
		};

		let lt = convert::<_, Transform2<f32>>(Translation2::new(0., ((5. / 2.) + 5.) * -1.));

		assert_eq!(
			text,
			Text {
				local_transform: lt,
				text: "famous last word".to_string(),
				align: TextAlign::Left,
				vertical_align: Default::default(),
				weight: Default::default(),
				style: Default::default(),
				on_curve: None,
				font_size: 5.,
				font: None
			}
		);
	}

	let bb = shape.local_bounding_box();
	assert_float_absolute_eq!(bb.height(), 10., 0.001);
}
