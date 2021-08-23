use dessin::{
    shape::{Fill, Stroke, Style},
    shape::{ImageFormat, Keypoint},
    style::{FontWeight, TextAlign},
    vec2, Shape, ShapeType, Vec2,
};
use printpdf::{
    image::EncodableLayout, Color, Image, IndirectFontRef, Line, LineCapStyle, LineDashPattern, Mm,
    PdfLayerReference, Point, Rgb,
};
use rusttype::{Font, Scale};

use crate::{ToPDFPart, ARIAL_BOLD, ARIAL_BOLD_ITALIC, ARIAL_ITALIC, ARIAL_REGULAR};

fn point(v: Vec2) -> Point {
    Point::new(Mm(v.x as f64), Mm(v.y as f64))
}

fn color(c: dessin::style::Color) -> Color {
    let c = c.rgba();
    if let dessin::style::Color::RGBA { r, g, b, a: _ } = c {
        Color::Rgb(Rgb {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            icc_profile: None,
        })
    } else {
        unreachable!()
    }
}

fn setup_style(style: &Option<Style>, _: &IndirectFontRef, layer: &PdfLayerReference) {
    if let Some(fill) = style.as_ref().map(|v| v.fill).flatten() {
        match fill {
            Fill::Color(c) => {
                layer.set_fill_color(color(c));
            }
        }
    } else {
        layer.set_fill_color(Color::Rgb(Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            icc_profile: None,
        }));
    }

    if let Some(stroke) = style.as_ref().map(|v| v.stroke).flatten() {
        match stroke {
            Stroke::Full { color: c, width } => {
                layer.set_outline_color(color(c));
                layer.set_outline_thickness(width as f64);
                layer.set_line_dash_pattern(LineDashPattern::default());
                layer.set_line_cap_style(LineCapStyle::Butt);
            }
            Stroke::Dashed {
                color: c,
                width,
                on,
                off,
            } => {
                layer.set_outline_color(color(c));
                layer.set_outline_thickness(width as f64);
                layer.set_line_dash_pattern(LineDashPattern::new(
                    0,
                    Some(on as i64),
                    Some(off as i64),
                    None,
                    None,
                    None,
                    None,
                ));
                layer.set_line_cap_style(LineCapStyle::Butt);
            }
        }
    } else {
        layer.set_outline_color(Color::Rgb(Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            icc_profile: None,
        }));
        layer.set_outline_thickness(0.);
        layer.set_line_dash_pattern(LineDashPattern::default());
        layer.set_line_cap_style(LineCapStyle::Butt);
    }
}

impl ToPDFPart for Shape {
    fn to_pdf_part(
        &self,
        dpi: f64,
        offset: Vec2,
        font: &IndirectFontRef,
        layer: &PdfLayerReference,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pos = self.pos.position_from_center() + offset;
        let size = self.pos.size();
        match &self.shape_type {
            ShapeType::Text {
                text,
                align,
                font_size,
                font_weight,
            } => {
                let font_calc = match font_weight {
                    FontWeight::Regular => {
                        Font::try_from_bytes(ARIAL_REGULAR).ok_or("Could not parse font")?
                    }
                    FontWeight::Bold => {
                        Font::try_from_bytes(ARIAL_BOLD).ok_or("Could not parse font")?
                    }
                    FontWeight::Italic => {
                        Font::try_from_bytes(ARIAL_ITALIC).ok_or("Could not parse font")?
                    }
                    FontWeight::BoldItalic => {
                        Font::try_from_bytes(ARIAL_BOLD_ITALIC).ok_or("Could not parse font")?
                    }
                };

                let scale = Scale::uniform(*font_size);
                let total_width = font_calc
                    .glyphs_for(text.chars())
                    .scan(None, |last, g| {
                        let pos = if let Some(last) = last {
                            font_calc.pair_kerning(scale, *last, g.clone().id())
                        } else {
                            0.0
                        } + g.clone().scaled(scale).h_metrics().advance_width;
                        *last = Some(g.id());
                        Some(pos)
                    })
                    .sum::<f32>()
                    * 1.124;

                let x = pos.x
                    - match align {
                        TextAlign::Left => 0.,
                        TextAlign::Center => 0.5,
                        TextAlign::Right => 1.,
                    } * total_width;

                let y = pos.y - *font_size / 2.;

                let font_size = *font_size * 2.835; // Found after linear regression.

                setup_style(&self.style, font, layer);

                layer.begin_text_section();
                {
                    layer.set_font(font, font_size as f64);
                    layer.set_text_cursor(Mm(x as f64), Mm(y as f64));
                    layer.write_text(text, font);
                }
                layer.end_text_section();
            }
            ShapeType::Line { from, to } => {
                setup_style(&self.style, font, layer);

                let line = Line {
                    points: vec![(point(*from + offset), false), (point(*to + offset), false)],
                    is_closed: false,
                    has_fill: self
                        .style
                        .as_ref()
                        .map(|v| v.fill.is_some())
                        .unwrap_or(false),
                    has_stroke: self
                        .style
                        .as_ref()
                        .map(|v| v.stroke.is_some())
                        .unwrap_or(false),
                    is_clipping_path: false,
                };

                layer.add_shape(line);
            }
            ShapeType::Circle { radius } => {
                setup_style(&self.style, font, layer);

                let radius = *radius;
                let points = vec![
                    (point(pos + vec2(radius, 0.)), true),
                    (point(pos + vec2(radius, radius * 0.552284749831)), true),
                    (point(pos + vec2(radius * 0.552284749831, radius)), true),
                    (point(pos + vec2(0., radius)), true),
                    (point(pos + vec2(-radius * 0.552284749831, radius)), true),
                    (point(pos + vec2(-radius, radius * 0.552284749831)), true),
                    (point(pos + vec2(-radius, 0.)), true),
                    (point(pos + vec2(-radius, -radius * 0.552284749831)), true),
                    (point(pos + vec2(-radius * 0.552284749831, -radius)), true),
                    (point(pos + vec2(0., -radius)), true),
                    (point(pos + vec2(radius * 0.552284749831, -radius)), true),
                    (point(pos + vec2(radius, -radius * 0.552284749831)), true),
                    (point(pos + vec2(radius, 0.)), false),
                ];

                let line = Line {
                    points,
                    is_closed: true,
                    has_fill: self
                        .style
                        .as_ref()
                        .map(|v| v.fill.is_some())
                        .unwrap_or(false),
                    has_stroke: self
                        .style
                        .as_ref()
                        .map(|v| v.stroke.is_some())
                        .unwrap_or(false),
                    is_clipping_path: false,
                };

                layer.add_shape(line);
            }
            ShapeType::Image { data } => {
                let image = match data {
                    ImageFormat::PNG(data) => Image::try_from(
                        printpdf::image::codecs::png::PngDecoder::new(&mut data.as_bytes())?,
                    ),
                    ImageFormat::JPEG(data) => Image::try_from(
                        printpdf::image::codecs::jpeg::JpegDecoder::new(&mut data.as_bytes())?,
                    ),
                    ImageFormat::Webp(data) => Image::try_from(
                        printpdf::image::codecs::webp::WebPDecoder::new(&mut data.as_bytes())?,
                    ),
                }?;

                let width = Mm::from(image.image.width.into_pt(dpi)).0 as f32;
                let height = Mm::from(image.image.height.into_pt(dpi)).0 as f32;

                let scale_x = size.x / width;
                let scale_y = size.y / height;

                image.add_to_layer(
                    layer.clone(),
                    Some(Mm(10.)),
                    Some(Mm(10.)),
                    None,
                    Some(scale_x as f64),
                    Some(scale_y as f64),
                    Some(300.),
                );
            }
            ShapeType::Drawing(shapes) => {
                shapes
                    .iter()
                    .map(|v| v.to_pdf_part(dpi, offset, font, layer))
                    .collect::<Result<(), Box<dyn std::error::Error>>>()?;
            }
            ShapeType::Path { keypoints, closed } => {
                setup_style(&self.style, font, layer);

                let mut points = Vec::with_capacity(keypoints.len());
                for idx in 0..keypoints.len() {
                    let curr = &keypoints[idx];
                    let next = keypoints
                        .get(idx + 1)
                        .map(|v| {
                            if let Keypoint::Point(..) = v {
                                false
                            } else {
                                true
                            }
                        })
                        .unwrap_or(false);

                    match curr {
                        Keypoint::Point(p) => {
                            points.push((point(*p), next));
                        }
                        Keypoint::BezierQuad { to, control } => {
                            points.push((point(*control), true));
                            points.push((point(*to), next));
                        }
                        Keypoint::BezierCubic {
                            to,
                            control_from,
                            control_to,
                        } => {
                            points.push((point(*control_from), true));
                            points.push((point(*control_to), true));
                            points.push((point(*to), next));
                        }
                    }
                }

                let line = Line {
                    points,
                    is_closed: *closed,
                    has_fill: self
                        .style
                        .as_ref()
                        .map(|v| v.fill.is_some())
                        .unwrap_or(false),
                    has_stroke: self
                        .style
                        .as_ref()
                        .map(|v| v.stroke.is_some())
                        .unwrap_or(false),
                    is_clipping_path: false,
                };

                layer.add_shape(line);
            }
        }

        Ok(())
    }
}
