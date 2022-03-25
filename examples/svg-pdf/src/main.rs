use dessin::{
    contrib::{TextBox, TextLayout},
    shape::{Circle, Color, EmbeddedDrawing, Fill, Image, ImageFormat, Line, Stroke, Text},
    style::TextAlign,
    vec2, Drawing,
};
use dessin_pdf::ToPDF;
use dessin_svg::ToSVG;
use std::{error::Error, fs::write};

pub fn dessin() -> Drawing {
    let mut drawing = Drawing::empty().with_canvas_size(vec2(300., 300.));

    const TEXT_BOX_CONTENT: &str =
        "This is a long long test to see if the textbox works as intended.
On top of that, the output should be the same on PDF and SVG!";

    drawing
        .add(
            TextLayout::new(TEXT_BOX_CONTENT.to_owned())
                .add_box(
                    TextBox::new()
                        .at(vec2(-120., 120.))
                        .with_size(vec2(30., 30.))
                        .with_font_size(4.)
                        .with_spacing(4.)
                        .with_fill(Fill::Color(Color::RED))
                        .with_align(TextAlign::Center),
                )
                .add_box(
                    TextBox::new()
                        .at(vec2(120., 120.))
                        .with_size(vec2(30., 60.))
                        .with_font_size(5.)
                        .with_fill(Fill::Color(Color::BLACK))
                        .with_align(TextAlign::Right),
                ),
        )
        .add(
            Text::new("Hello World".to_owned())
                .at(vec2(0., -10.))
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

    drawing
}

pub fn embedded(dessin: Drawing) -> Drawing {
    let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
    parent
        .add(
            EmbeddedDrawing::new(dessin.clone())
                .at(vec2(-35., -35.))
                .with_size(vec2(10., 10.)),
        )
        .add(
            EmbeddedDrawing::new(dessin)
                .at(vec2(35., -35.))
                .with_size(vec2(10., 10.)),
        )
        .add(
            Text::new("Meta Hello World".to_owned())
                .at(vec2(-30., -38.))
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
            Image::new(ImageFormat::JPEG(
                include_bytes!("rustacean-flat-happy.jpg").to_vec(),
            ))
            .at(vec2(-25., 0.))
            .with_size(vec2(50., 40.)),
        );

    parent
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // SVG
    {
        let dessin = dessin();
        write(
            "./dessin.html",
            format!(r#"<html><body>{}</body></html>"#, dessin.to_svg()?),
        )?;
        write(
            "./embedded.html",
            format!(
                r#"<html><body>{}</body></html>"#,
                embedded(dessin).to_svg()?
            ),
        )?;
    }

    // PDF
    {
        let dessin = dessin();
        write("./dessin.pdf", dessin.to_pdf()?.into_bytes()?)?;
        write("./embedded.pdf", embedded(dessin).to_pdf()?.into_bytes()?)?;
    }

    Ok(())
}
