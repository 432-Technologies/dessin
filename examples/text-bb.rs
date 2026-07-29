use dessin::prelude::*;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	dessin::font::add_font(include_bytes!("./AtkinsonHyperlegibleNextVF-Variable.ttf"));
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	fn make_dessin(align: TextAlign, vertical_align: TextVerticalAlign) -> Shape {
		let width = 110.;
		let height = 110.;

		dessin!([
			*Rectangle(
				{width},
				{ height },
				translate = [width / 2., -height / 2.],
				stroke = (Srgb::new(1., 0., 0.), 1.),
			),
			TextBox(
				text = "Hello, World!\nThis is a multiline text !\nAnd here is a very long line of text !\nAnd it works with 🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪",
				{ width },
				font_size = 10.,
				{ align },
				{ vertical_align },
			),
			*Circle(radius = 0.5, fill = Srgb::new(0., 0., 1.),),
		])
	}

	let dessin = make_dessin(TextAlign::Left, TextVerticalAlign::Top);

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
