use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_pdf::PdfExporter;
use dessin_svg::SvgExporter;
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
	let croix_basque: Shape = dessin!([
		*Circle(radius = 0.01, fill = Srgba::new(1.0, 0.0, 0.0, 1.0),),
		for n in 0..=4 {
			dessin!([
				*ThickArc(
					start_angle = PI / 2_f32,
					outer_radius = 20f32,
					inner_radius = 0f32,
					span_angle = PI,
					fill = Srgba::new(1.0, 0.0, 0.0, 1.0),
					translate = [0., 20.],
					rotate = Rotation2::new(PI * (n as f32) / 2_f32)
				),
				*Circle(
					radius = 10.,
					fill = Srgba::new(1.0, 0.0, 0.0, 1.0),
					translate = [0., 30.],
					rotate = Rotation2::new(PI * (n as f32) / 2_f32)
				),
				*ThickArc(
					start_angle = PI / 2_f32,
					outer_radius = 10f32,
					inner_radius = 0f32,
					span_angle = PI,
					fill = Srgba::new(1.0, 1.0, 1.0, 1.0),
					translate = [0., 10.],
					rotate = Rotation2::new(PI * (n as f32) / 2_f32)
				)
			])
		}
	]);

	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/croix-basque.svg"),
		PdfExporter::default()
			.export(&croix_basque)
			.unwrap()
			.finish()
			.unwrap(),
	)
	.unwrap();

	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/croix-basque.svg"),
		SvgExporter::default().export(&croix_basque).unwrap(),
	)
	.unwrap();
}
