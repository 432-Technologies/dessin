use dessin::prelude::*;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	dessin::font::add_font(include_bytes!("./AtkinsonHyperlegibleNextVF-Variable.ttf"));
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	let text = dessin!(Text(text = "Normal textjgp", font_size = 10.));
	let bb = text.local_bounding_box();

	let dessin = dessin!([
		{ text },
		*Curve(
			then = bb.top_left(),
			then = bb.top_right(),
			stroke = (Srgb::new(1., 0., 0.), 0.1),
		),
		*Curve(
			then = bb.bottom_right(),
			then = bb.bottom_left(),
			stroke = (Srgb::new(0., 1., 0.), 0.1),
		),
		*Rectangle(
			width = 100.,
			height = 100.,
			translate = [30., 0.],
			stroke = (Srgb::new(0., 0., 1.), 0.1),
		),
		*Circle(radius = 0.5, fill = Srgb::new(0., 0., 1.),),
	]);

	fs::write(
		get_project_root().unwrap().join("examples/out/text-bb.svg"),
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
