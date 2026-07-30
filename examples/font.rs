use dessin::prelude::*;
use dessin_svg::SvgExporter;
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

	let layout = dessin!(
		*VerticalLayout(
			of = normal_text,
			of = bold_text,
			of = italic_text,
			fill = Srgb::new(0., 0., 0.)
		) > ()
	);

	let bb = layout.local_bounding_box();

	let dessin = dessin!(
		[
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
		] > (scale = [10., 10.])
	);

	fs::write(
		get_project_root().unwrap().join("examples/out/font.pdf"),
		dessin_pdf::to_pdf(&dessin).unwrap(),
	)
	.unwrap();

	fs::write(
		get_project_root().unwrap().join("examples/out/font.svg"),
		SvgExporter::default().export(&dessin).unwrap(),
	)
	.unwrap();
}
