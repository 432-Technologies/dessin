use std::fs::write;

use dessin::{
    shape::{Circle, Color, Fill, Image, ImageFormat, Line, Stroke, Text},
    style::TextAlign,
    vec2, Drawing,
};
use dessin_pdf::ToPDF;

pub fn main() {
    let mut drawing = Drawing::empty().with_canvas_size(vec2(300., 300.));
    drawing
        .add(
            Text::new("Hello World".to_owned())
                .at(vec2(0., 0.))
                .with_align(TextAlign::Center)
                .with_fill(Fill::Color(Color::U32(0xFF0000)))
                .with_font_size(16.),
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
        )
        .add(
            Image::new(ImageFormat::PNG(
                include_bytes!("rustacean-flat-happy.png").to_vec(),
            ))
            .at(vec2(0., 0.))
            .with_size(vec2(50., 40.)),
        );

    write(
        "./test.pdf",
        drawing.to_pdf().unwrap().into_bytes().unwrap(),
    )
    .unwrap();
}
