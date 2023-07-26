use ::image::{DynamicImage, Rgba32FImage};
use dessin::{
    export::{Export, Exporter},
    prelude::*,
};
use nalgebra::{Point2, Transform2, Translation2, Vector2};
use std::fmt;

#[derive(Debug)]
pub enum ImageError {
    WriteError(fmt::Error),
    CurveHasNoStartingPoint(CurvePosition),
}
impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl From<fmt::Error> for ImageError {
    fn from(value: fmt::Error) -> Self {
        ImageError::WriteError(value)
    }
}
impl std::error::Error for ImageError {}

#[derive(Default)]
pub struct ImageOptions {
    pub canvas: Option<(f32, f32)>,
}

pub struct ImageExporter {
    buffer: Rgba32FImage,
    style: Vec<StylePosition>,
}

impl ImageExporter {
    fn new(width: u32, height: u32) -> Self {
        ImageExporter {
            buffer: Rgba32FImage::new(width, height),
            style: vec![],
        }
    }

    fn finalize(self) -> Rgba32FImage {
        self.buffer
    }

    fn style(&self) -> StylePosition {
        let mut acc = StylePosition {
            stroke: None,
            fill: None,
        };

        for style in self.style.iter().rev() {
            match (acc.fill, style.fill) {
                (None, Some(s)) => acc.fill = Some(s),
                _ => {}
            }

            match (acc.stroke, style.stroke) {
                (None, Some(s)) => acc.stroke = Some(s),
                _ => {}
            }

            if acc.fill.is_some() && acc.fill.is_some() {
                break;
            }
        }

        acc
    }
}

impl Exporter for ImageExporter {
    type Error = ImageError;
    const CAN_EXPORT_ELLIPSE: bool = false;

    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> {
        self.style.push(style);
        Ok(())
    }

    fn end_style(&mut self) -> Result<(), Self::Error> {
        self.style.pop();
        Ok(())
    }

    fn export_image(
        &mut self,
        ImagePosition {
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left: _,
            center,
            width,
            height,
            rotation,
            image,
        }: ImagePosition,
    ) -> Result<(), Self::Error> {
        // let mut raw_image = Cursor::new(vec![]);
        // image.write_to(&mut raw_image, ImageFormat::Png).unwrap();

        // let data = data_encoding::BASE64.encode(&raw_image.into_inner());

        // write!(
        //     self.acc,
        //     r#"<image width="{width}" height="{height}" x="{x}" y="{y}" "#,
        //     x = center.x - width / 2.,
        //     y = center.y - height / 2.,
        // )?;

        // if rotation.abs() > 10e-6 {
        //     write!(
        //         self.acc,
        //         r#" transform="rotate({rot})" "#,
        //         rot = -rotation.to_degrees()
        //     )?;
        // }

        // write!(self.acc, r#"href="data:image/png;base64,{data}"/>"#,)?;

        Ok(())
    }

    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> {
        let style = self.style();

        if let Some(Fill::Color(c)) = style.fill {
            // draw_hollow_ellipse_mut(&mut self.buffer, (center.x as i32, center.y as i32), wi)
            // imageproc::d
        }

        let stroke_color = style
            .stroke
            .map(|v| match v {
                Stroke::Full { color, width } => color,
                Stroke::Dashed {
                    color,
                    width,
                    on,
                    off,
                } => color,
            })
            .or_else(|| {
                style.fill.map(|v| match v {
                    Fill::Color(c) => c,
                })
            });

        match stroke_color {
            Some(c) => {
                let (r, g, b, a) = c.as_rgba_f32();
                let color = ::image::Rgba([r, g, b, a]);

                let mut first_point = None;
                let mut last_point = None;

                for k in &curve.keypoints {
                    match (&mut last_point, k) {
                        (x @ None, KeypointPosition::Point(p)) => {
                            *x = Some(*p);
                            first_point = Some(*p);
                        }
                        (
                            None,
                            KeypointPosition::Bezier(Bezier {
                                start,
                                start_control,
                                end_control,
                                end,
                            }),
                        ) => {
                            let start = start.ok_or_else(|| {
                                ImageError::CurveHasNoStartingPoint(curve.clone())
                            })?;

                            first_point = Some(start);
                            last_point = Some(*end);

                            imageproc::drawing::draw_cubic_bezier_curve_mut(
                                &mut self.buffer,
                                (start.x, start.y),
                                (end.x, end.y),
                                (start_control.x, start_control.y),
                                (end_control.x, end_control.y),
                                color,
                            );
                        }
                        (Some(_p), KeypointPosition::Point(p)) => {
                            imageproc::drawing::draw_line_segment_mut(
                                &mut self.buffer,
                                (_p.x, _p.y),
                                (p.x, p.y),
                                color,
                            );
                            *_p = *p;
                        }
                        (
                            Some(_p),
                            KeypointPosition::Bezier(Bezier {
                                start,
                                start_control,
                                end_control,
                                end,
                            }),
                        ) => {
                            let start = match start {
                                Some(p) => {
                                    imageproc::drawing::draw_line_segment_mut(
                                        &mut self.buffer,
                                        (_p.x, _p.y),
                                        (p.x, p.y),
                                        color,
                                    );
                                    p
                                }
                                None => _p,
                            };

                            imageproc::drawing::draw_cubic_bezier_curve_mut(
                                &mut self.buffer,
                                (start.x, start.y),
                                (end.x, end.y),
                                (start_control.x, start_control.y),
                                (end_control.x, end_control.y),
                                color,
                            );

                            *_p = *end;
                        }
                    }

                    if first_point.is_none() {
                        match k {
                            KeypointPosition::Point(p) => first_point = Some(*p),
                            KeypointPosition::Bezier(Bezier { start, .. }) => {
                                first_point = Some(start.ok_or_else(|| {
                                    ImageError::CurveHasNoStartingPoint(curve.clone())
                                })?)
                            }
                        }
                    }
                }

                if curve.closed {
                    match (last_point, first_point) {
                        (Some(_p), Some(p)) => imageproc::drawing::draw_line_segment_mut(
                            &mut self.buffer,
                            (_p.x, _p.y),
                            (p.x, p.y),
                            color,
                        ),
                        _ => {}
                    }
                }
            }
            None => {}
        }

        Ok(())
    }

    fn export_text(
        &mut self,
        TextPosition {
            text,
            align,
            font_weight,
            on_curve,
            font_size,
            reference_start,
            direction,
            font,
        }: TextPosition,
    ) -> Result<(), Self::Error> {
        // let id = rand::random::<u64>().to_string();

        // let weight = match font_weight {
        //     FontWeight::Bold | FontWeight::BoldItalic => "bold",
        //     _ => "normal",
        // };
        // let text_style = match font_weight {
        //     FontWeight::Italic | FontWeight::BoldItalic => "italic",
        //     _ => "normal",
        // };
        // let align = match align {
        //     TextAlign::Center => "middle",
        //     TextAlign::Left => "start",
        //     TextAlign::Right => "end",
        // };

        // let text = text.replace("<", "&lt;").replace(">", "&gt;");
        // let font = font
        //     .as_ref()
        //     .map(|v| v.name(font_weight))
        //     .unwrap_or_else(|| dessin::font::FontRef::default().name(font_weight));

        // write!(
        //     self.acc,
        //     r#"<text x="{x}" y="{y}" font-family="{font}" text-anchor="{align}" font-size="{font_size}px" font-weight="{weight}" text-style="{text_style}""#,
        //     x = reference_start.x,
        //     y = reference_start.y,
        // )?;

        // let rotation = direction.angle(&Vector2::new(1., 0.));
        // if rotation.abs() > 10e-6 {
        //     write!(
        //         self.acc,
        //         r#" transform="rotate({rot})" "#,
        //         rot = -rotation.to_degrees()
        //     )?;
        // }

        // write!(self.acc, r#">"#)?;

        // if let Some(curve) = on_curve {
        //     write!(self.acc, r#"<path id="{id}" d=""#)?;
        //     self.write_curve(curve)?;
        //     write!(self.acc, r#""/>"#)?;

        //     write!(self.acc, r##"<textPath href="#{id}">{text}</textPath>"##)?;
        // } else {
        //     write!(self.acc, "{text}")?;
        // }
        // write!(self.acc, r#"</text>"#)?;

        Ok(())
    }
}

pub trait ToImage {
    fn rasterize(&self) -> Result<DynamicImage, ImageError>;
}

impl ToImage for Shape {
    fn rasterize(&self) -> Result<DynamicImage, ImageError> {
        let bb = self.local_bounding_box().unwrap().straigthen();

        let center: Vector2<f32> = bb.center() - Point2::origin();
        let translation =
            Translation2::from(Vector2::new(bb.width() / 2., bb.height() / 2.) - center);
        let scale = nalgebra::Scale2::new(1., -1.);
        let transform = nalgebra::convert::<_, Transform2<f32>>(translation)
            * nalgebra::convert::<_, Transform2<f32>>(scale);

        let mut exporter = ImageExporter::new(bb.width().ceil() as u32, bb.height().ceil() as u32);

        self.write_into_exporter(&mut exporter, &transform)?;

        Ok(DynamicImage::ImageRgba32F(exporter.finalize()))
    }
}
