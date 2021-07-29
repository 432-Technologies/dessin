use std::fs::write;

use dessin::{
    shape::{Circle, Color, EmbeddedDrawing, Fill, Image, ImageFormat, Line, Stroke, Text},
    style::TextAlign,
    vec2, Drawing,
};
use dessin_svg::ToSVG;

pub fn main() {
    let mut drawing = Drawing::empty().with_canvas_size(vec2(300., 300.));
    drawing
        .add(
            Text::new("Hello World".to_owned())
                .at(vec2(0., 0.))
                .with_align(TextAlign::Center)
                .with_fill(Fill::Color(Color::U32(0xFF0000))),
        )
        .add(
            Line::from(vec2(-50., 5.))
                .to(vec2(50., 5.))
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000),
                    width: 2.,
                    on: 4.,
                    off: 1.,
                }),
        )
        .add(
            Line::from(vec2(-50., -15.))
                .to(vec2(50., -15.))
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000),
                    width: 2.,
                    on: 4.,
                    off: 1.,
                }),
        )
        .add(
            Line::from(vec2(-50., -15.))
                .to(vec2(-50., 5.))
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000),
                    width: 2.,
                    on: 4.,
                    off: 1.,
                }),
        )
        .add(
            Line::from(vec2(50., -15.))
                .to(vec2(50., 5.))
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000),
                    width: 2.,
                    on: 4.,
                    off: 1.,
                }),
        )
        .add(
            Circle::new()
                .at(vec2(0., 0.))
                .with_radius(100.)
                .with_stroke(Stroke::Full {
                    color: Color::U32(0xFF0000),
                    width: 5.,
                }),
        );

    let res = format!(r#"<html><body>{}"#, drawing.to_svg().unwrap());

    let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
    parent
        .add(
            EmbeddedDrawing::new(drawing)
                .at(vec2(-35., -35.))
                .with_size(vec2(10., 10.)),
        )
        .add(
            Text::new("Meta Hello World".to_owned())
                .at(vec2(-30., -32.))
                .with_font_size(8.)
                .with_fill(Fill::Color(Color::U32(0x1000FF))),
        )
        .add(
            Line::from(vec2(-40., -40.))
                .to(vec2(40., -40.))
                .with_stroke(Stroke::Full {
                    color: Color::U32(0x1000FF),
                    width: 1.,
                }),
        )
        .add(
            Line::from(vec2(-40., -30.))
                .to(vec2(40., -30.))
                .with_stroke(Stroke::Full {
                    color: Color::U32(0x1000FF),
                    width: 1.,
                }),
        )
        .add(
            Image::new(ImageFormat::PNG(
                include_bytes!("rustacean-flat-happy.png").to_vec(),
            ))
            .at(vec2(-25., 0.))
            .with_size(vec2(50., 40.)),
        );

    write(
        "./test.html",
        format!(r#"{}{}</body></html>"#, res, parent.to_svg().unwrap()),
    )
    .unwrap();
}
