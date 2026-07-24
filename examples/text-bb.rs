use dessin::prelude::*;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	dessin::font::add_font(include_bytes!("./AtkinsonHyperlegibleNextVF-Variable.ttf"));
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	fn make_dessin(align: TextAlign) -> Shape {
		dessin!([
			TextBox(
				text = "Hello, World!\nThis is a multiline text !\nLook at me mom !",
				font_size = 10.,
				{ align },
			),
			*Circle(radius = 0.5, fill = Srgb::new(0., 0., 1.),),
		])
	}

	// let dessin = dessin!(
	// 	VerticalLayout(
	// 		of = make_dessin(TextAlign::Left),
	// 		of = make_dessin(TextAlign::Center),
	// 		of = make_dessin(TextAlign::Right)
	// 	) > ()
	// );
	let dessin = make_dessin(TextAlign::Center);

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
