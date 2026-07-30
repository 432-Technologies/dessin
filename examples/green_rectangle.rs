use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SvgExporter;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	let rectangle: Shape = dessin!([*Rectangle(
		width = 11.,
		height = 6.,
		fill = Srgb::new(0.0, 1.0, 0.0),
		stroke = Stroke::new_solid(Srgb::new(0.576, 0.576, 0.576), 0.1),
		rotate = Rotation2::new(6_f32.to_radians()),
	),]);

	// prints in svg version
	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/green_rectangle.svg"),
		SvgExporter::default().export(&rectangle).unwrap(),
	)
	.unwrap();
}
