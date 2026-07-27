use dessin::prelude::*;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	dessin::font::add_font(include_bytes!("./AtkinsonHyperlegibleNextVF-Variable.ttf"));
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	let normal_text = dessin!(Text(text = "Normal text", font_size = 10.));
	let bold_text = dessin!(Text(
		text = "Bold text",
		weight = FontWeight::BOLD,
		font_size = 10.,
	));
	let italic_text = dessin!(Text(
		text = "Italic text",
		style = FontStyle::Italic,
		font_size = 10.
	));

	let layout = dessin!(*VerticalLayout(of = normal_text, of = bold_text, of = italic_text,) > ());

	let bb = layout.local_bounding_box();

	let dessin = dessin!([
		*Curve(
			then = bb.top_left(),
			then = bb.top_right(),
			then = bb.bottom_right(),
			then = bb.bottom_left(),
			closed,
			stroke = (Srgb::new(1., 0., 0.), 0.1),
		),
		{ layout },
		*Circle(radius = 0.5, fill = Srgb::new(0., 0., 1.),),
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
