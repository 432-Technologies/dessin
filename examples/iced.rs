use dessin::prelude::*;
use dessin_iced::DessinIced;
use iced::{
	widget::{column, container},
	Background, Color, Element,
};
use palette::rgb::Rgba;

fn main() {
	if std::env::var("NO_ICED") == Ok("1".to_string()) {
		return;
	}

	iced::application(boot, update, view).run().unwrap();
}

fn boot() -> () {
	()
}

fn update(_state: &mut (), _message: ()) {}

fn view(_state: &()) -> Element<'_, ()> {
	let dessin1 = dessin!([
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

	let dessin2 = dessin!({ dessin1.clone() }(scale = [100., 50.]));

	column![
		container(dessin2.view().width(50.).height(50.))
			.style(|_| container::background(Background::Color(Color::from_rgb(1., 0., 0.)))),
		container(dessin1.view().width(500.).height(500.))
			.style(|_| container::background(Background::Color(Color::from_rgb(0., 0., 1.)))),
	]
	.spacing(10.)
	.into()
}
