use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SvgExporter;
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
	let optical_effect: Shape = dessin!([
		for n in 0..11 {
			dessin!([*ThickArc(
				outer_radius = 10f32,
				inner_radius = 0f32,
				span_angle = PI / 10_f32,
				fill = Srgba::new(0.0, 0.0, 0.0, 1.0),
				rotate = Rotation2::new(PI * (n as f32) / 5_f32)
			)])
		},
		*Circle(radius = 1., fill = Srgba::new(1.0, 1.0, 1.0, 1.0),),
		*Rectangle(
			width = 15.,
			height = 15.,
			stroke = Stroke::new_solid(Srgba::new(0.0, 0.0, 0.0, 0.01), 1.)
		)
	]);

	let fond = optical_effect.local_bounding_box();

	let truc = dessin!([
		*Rectangle(
			width = fond.width(),
			height = fond.height(),
			fill = Srgba::new(0.588, 0.588, 0.588, 0.2),
		),
		{ optical_effect }
	]);

	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/optical-effect.svg"),
		SvgExporter::default()
			.viewport(dessin::export::ViewPort::Manual(
				dessin::shapes::BoundingBox::centered([14., 14.]),
			))
			.export(&truc)
			.unwrap(),
	)
	.unwrap();
}
