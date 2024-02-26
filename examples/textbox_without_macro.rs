use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SVG;
use project_root::get_project_root;
use std::fs;

fn main() {
    // creates a text
    let text = TextBox::default();

    let mut text = Style::new(text);

    text.font_size(5.);

    text.line_spacing(1.);

    text.text("Here we write some text");

    text.width(20.);

    text.height(10.);

    text.font_weight(FontWeight::Italic);

    // chooses centered vertical allign
    text.vertical_align(TextVerticalAlign::Center);

    // selects to align the beginning of the text on the left
    text.align(TextAlign::Left);

    // paints the inside of the text in bright orange
    text.fill(Fill::Color(rgb(255, 191, 0)));

    text.stroke(Stroke::Full {
        color: rgb(150, 10, 10),
        width: 0.1,
    });

    // chooses a rotation of -6 radians in the trigonometric direction
    text.rotate(Rotation2::new(6_f32.to_radians()));

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/text.svg"),
        SVG::from(text).to_string().unwrap(),
    )
    .unwrap();
}
