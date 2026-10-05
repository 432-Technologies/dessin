use dessin::{
	palette::{Srgb, Srgba},
	prelude::*,
};
use iced::{Element, Font, Length, Theme};
use iced_widget::{button, column, lazy, text};

fn make_dessin(align: TextAlign, vertical_align: TextVerticalAlign) -> Shape {
	let width = 110.;
	let height = 110.;

	dessin!([
			*Rectangle(
				{width},
				{ height },
				translate = [width / 2., -height / 2.],
				stroke = (Srgb::new(1., 0., 0.), 1.),
			),
			*TextBox(
				text = "Hello, World!\nThis is a multiline text !\nAnd here is a very long line of text !\nAnd it works with 🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪🫪",
				{ width },
				font_size = 10.,
				{ align },
				{ vertical_align },
				fill = Srgb::new(0., 0., 0.)
			),
			*Circle(radius = 0.5, fill = Srgb::new(0., 0., 1.),),
		])
}

fn main() {
	let font_bytes = include_bytes!("../../examples/AtkinsonHyperlegibleNextVF-Variable.ttf");
	dessin::font::add_font(font_bytes);
	dessin::font::set_default_font(dessin::font::get("Atkinson Hyperlegible Next VF").unwrap());

	type State = (Shape, bool);
	type Message = ();
	fn boot() -> State {
		(make_dessin(TextAlign::Left, TextVerticalAlign::Top), false)
	}
	fn update(state: &mut State, _message: Message) {
		state.1 = !state.1;
	}
	fn view(state: &State) -> Element<'_, Message, Theme, iced::Renderer> {
		column![
			text("Dessin"),
			dessin_iced::dessin(state.0.clone()).width(300).height(300),
			button("Toggle theme").on_press(()),
			lazy(state.1, |_| {
				dessin_iced::dessin(|theme: &Theme| {
					let palette = theme.palette();

					let (rect_color, text_color) = if palette.is_dark {
						(Srgba::new(1., 1., 1., 1.), Srgba::new(0., 0., 0., 1.))
					} else {
						(Srgba::new(0., 0., 0., 1.), Srgba::new(1., 1., 1., 1.))
					};

					dessin!([
						*Rectangle(fill = rect_color, height = 20., width = 50.),
						*Text(
							text = "SuperText",
							font_size = 10.,
							fill = text_color,
							align = TextAlign::Center,
						)
					])
				})
				.width(300)
				.height(300)
			})
		]
		.into()
	}
	fn theme(state: &State) -> Theme {
		if state.1 {
			Theme::Dark
		} else {
			Theme::Light
		}
	}

	iced::application::<State, Message, Theme, iced::Renderer>(boot, update, view)
		.fonts([font_bytes])
		.font(Font {
			family: iced::font::Family::Name("Atkinson Hyperlegible Mono VF"),
			..iced::Font::DEFAULT
		})
		.theme(theme)
		.run()
		.unwrap();
}
