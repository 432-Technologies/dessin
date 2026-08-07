use dessin::{palette::Srgb, prelude::*};
use iced::{Element, Font, Length};
use iced_widget::{column, text};

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

	type State = (Shape,);
	type Message = ();
	fn boot() -> State {
		(make_dessin(TextAlign::Left, TextVerticalAlign::Top),)
	}
	fn update(_state: &mut State, _message: Message) {}
	fn view(state: &State) -> Element<'_, Message, iced::Theme, iced::Renderer> {
		Element::from(column![
			text("Dessin"),
			dessin_iced::dessin(state.0.clone())
				.width(Length::Fill)
				.height(Length::Fill),
		])
	}

	iced::application::<State, Message, iced::Theme, iced::Renderer>(boot, update, view)
		.font(font_bytes)
		.default_font(Font {
			family: iced::font::Family::Name("Atkinson Hyperlegible Mono VF"),
			..iced::Font::DEFAULT
		})
		.run()
		.unwrap();
}
