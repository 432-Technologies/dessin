use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SvgExporter;
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
	let triangle: Shape = dessin!([*Triangle(
		width_x_axis = 4.,
		size_axis_angle = 12.,
		angle = 0.5,
		fill = Srgb::new(1.0, 0.0, 0.498),
		stroke = Stroke::new_dashed(Srgb::new(0.0, 0.0, 0.0), 0.1, 0.2, 0.1),
		rotate = Rotation2::new(-10_f32.to_radians())
	),]);

	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/any_triangle.svg"),
		SvgExporter::default().export(&triangle).unwrap(),
	)
	.unwrap();
}
