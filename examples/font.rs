use dessin::prelude::*;
use palette::Srgb;
use project_root::get_project_root;
use std::{borrow::Cow, fs};

fn main() {
	dessin::font::add_font(include_bytes!("./AtkinsonHyperlegibleNextVF-Variable.ttf"));
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	let normal_text = dessin!(Text(text = "Normal text", font_size = 10.));
	let bold_text = dessin!(Text(
		text = "Bold text",
		weight = FontWeight::BOLD,
		font_size = 10.
	));
	let italic_text = dessin!(Text(
		text = "Italic text",
		style = FontStyle::Italic,
		font_size = 10.
	));

	let dessin = dessin!([
		*VerticalLayout(
			of = normal_text,
			of = bold_text,
			of = italic_text,
			fill = Srgb::new(0., 0., 0.)
		),
		*Rectangle(
			width = 80.,
			height = 40.,
			translate = [40., -10.],
			stroke = (Srgb::new(0., 0., 0.), 1.),
		),
	]);

	fs::write(
		get_project_root().unwrap().join("examples/out/font.svg"),
		dessin_svg::to_string_with_options(
			&dessin,
			dessin_svg::SVGOptions {
				viewport: dessin_svg::ViewPort::AutoBoundingBox,
				skip_svg_tag: false,
				embed_fonts: true,
			},
		)
		.unwrap(),
	)
	.unwrap();
}
