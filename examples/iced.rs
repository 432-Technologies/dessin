use dessin::{
	nalgebra::{Point2, Rotation2},
	prelude::*,
};
use dessin_iced::IcedShape;
use iced::{widget::canvas, Element, Renderer, Theme};
use palette::{rgb::Rgba, Srgba};
use project_root::get_project_root;
use std::{f32::consts::FRAC_PI_6, fs};

fn main() {
	// let line: Shape = dessin!([
	// 	// creates a little circle as reference for a movement
	// 	*Circle(radius = 0.1),
	// 	// creates a line
	// 	*Line(
	// 		// chooses the starting point of the line
	// 		from = Point2::new(1., 0.),
	// 		// chooses the ending point of the line
	// 		to = Point2::new(12., 5.2),
	// 		// not needed here
	// 		fill = Srgba::new(1.0, 0.392, 0.392, 1.0),
	// 		stroke = Stroke::new_solid(Srgba::new(1.0, 0.392, 0.392, 1.0), 0.05),
	// 		translate = [5., 1.]
	// 	)
	// ]);

	// // prints in svg version
	// fs::write(
	// 	get_project_root().unwrap().join("examples/out/line.svg"),
	// 	dessin_svg::to_string(&line).unwrap(),
	// )
	// .unwrap();

	iced::application(boot, update, view).run().unwrap();
}

fn boot() -> () {
	()
}

fn update(state: &mut (), message: ()) {}

fn view(state: &()) -> Element<'_, ()> {
	let dessin = dessin!([
		*Circle(
			radius = 10.,
			translate = [5., 5.],
			fill = Fill::Solid {
				color: Rgba::new(1., 1., 0., 1.),
			},
		),
		*Circle(
			radius = 5.,
			translate = [0., 0.],
			fill = Fill::Solid {
				color: Rgba::new(1., 0., 0., 1.),
			},
		)
	]);

	canvas::<IcedShape, (), Theme, Renderer>(IcedShape(dessin))
		.width(500.)
		.height(500.)
		.into()
}
