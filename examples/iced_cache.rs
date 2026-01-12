use std::fs;

use dessin::prelude::*;
use dessin_iced::{CachedDessin, DessinIced};
use iced::{
	widget::{column, container},
	Background, Color, Element, Renderer,
};
use palette::rgb::Rgba;

type App = (CachedDessin<Renderer>, CachedDessin<Renderer>);

fn main() {
	iced::application(boot, update, view).run().unwrap();
}

fn boot() -> App {
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

	let dessin2 = dessin!([Image(
		image = dessin::image::load_from_memory(include_bytes!("out/432technologies.png")).unwrap(),
		keep_aspect_ratio,
		scale = [10., 10.],
	)]);

	(CachedDessin::new(dessin1), CachedDessin::new(dessin2))
}

fn update(_state: &mut App, _message: ()) {}

fn view(state: &App) -> Element<'_, ()> {
	column![
		container(state.0.view().width(200.).height(200.))
			.style(|_| container::background(Background::Color(Color::from_rgba(1., 0., 0., 0.1)))),
		container(state.1.view().width(200.).height(200.))
			.style(|_| container::background(Background::Color(Color::from_rgba(0., 0., 1., 0.1)))),
	]
	.spacing(10.)
	.into()
}
