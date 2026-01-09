use dessin::prelude::*;
use dessin_iced::DessinIced;
use iced::{widget::column, Element};
use palette::rgb::Rgba;

fn main() {
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
		dessin2.view().width(50.).height(50.),
		dessin1.view().width(500.).height(500.),
	]
	.spacing(0.)
	.into()
}
