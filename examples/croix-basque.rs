use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_pdf::PdfExporter;
use dessin_svg::SvgExporter;
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::FRAC_PI_2, fs};

#[derive(Default)]
struct Leaf;
impl From<Leaf> for Shape {
	fn from(_: Leaf) -> Self {
		dessin!(
			Curve(
				then = dessin!(Arc(
					start_angle = FRAC_PI_2,
					end_angle = -FRAC_PI_2,
					radius = 20f32,
				)),
				then = dessin!(Arc(
					start_angle = FRAC_PI_2,
					end_angle = -FRAC_PI_2,
					radius = 10f32,
					translate = [0., -10.],
				))
				.as_curve()
				.reversed(),
				then = dessin!(Arc(
					start_angle = -FRAC_PI_2,
					end_angle = FRAC_PI_2,
					radius = 10f32,
					translate = [0., 10.],
				)),
				closed
			) > ()
		)
	}
}

fn main() {
	let color: Srgba = Srgba::<u8>::new(0xE5, 0x0F, 0x0F, 0xFF).into_format();

	let croix_basque: Shape = dessin!(for n in 0..4 {
		dessin!([Leaf()
			> *(
				translate = [0., 20.],
				rotate = Rotation2::new(n as f32 * FRAC_PI_2),
				fill = color,
			)])
	});

	fs::write(
		get_project_root()
			.unwrap()
			.join("examples/out/croix-basque.pdf"),
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
