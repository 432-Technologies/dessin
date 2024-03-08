use ::image::ImageFormat;
use dessin::palette::IntoColor;
use dessin::{
    export::{Export, Exporter},
    palette::Srgb,
    prelude::*,
};
use nalgebra::{Scale2, Transform2};
use std::{
    fmt::{self, Write},
    io::Cursor,
};

#[derive(Debug)]
pub enum SVGError {
    WriteError(fmt::Error),
    CurveHasNoStartingPoint(CurvePosition),
}
impl fmt::Display for SVGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl From<fmt::Error> for SVGError {
    fn from(value: fmt::Error) -> Self {
        SVGError::WriteError(value)
    }
}
impl std::error::Error for SVGError {}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ViewPort {
    /// Create a viewport centered around (0, 0), with size (width, height)
    ManualCentered { width: f32, height: f32 },
    /// Create a viewport centered around (x, y), with size (width, height)
    ManualViewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Create a Viewport centered around (0, 0), with auto size that include all [Shapes][`dessin::prelude::Shape`]
    AutoCentered,
    #[default]
    /// Create a Viewport centered around the centered of the shapes, with auto size that include all [Shapes][`dessin::prelude::Shape`]
    AutoBoundingBox,
}

#[derive(Default, Clone)]
pub struct SVGOptions {
    pub viewport: ViewPort,
}

pub struct SVGExporter {
    acc: String,
}

impl SVGExporter {
    fn new(min_x: f32, min_y: f32, span_x: f32, span_y: f32) -> Self {
        const SCHEME: &str =
            r#"xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#;

        let fonts = dessin::font::fonts()
            .iter()
            .map(|(font_name, font_data)| {
                [
                    ("Regular", Some(&font_data.regular)),
                    ("Bold", font_data.bold.as_ref()),
                    ("Italic", font_data.italic.as_ref()),
                    ("BoldItalic", font_data.bold_italic.as_ref()),
                ]
                .into_iter()
                .filter_map(|(variant, data)| data.map(|v| (variant, v)))
                .filter_map(move |(variant, data)| {
                    let (mime, bytes) = match data {
                        dessin::font::Font::OTF(bytes) => ("font/otf", bytes),
                        dessin::font::Font::TTF(bytes) => ("font/ttf", bytes),
                        // dessin::font::Font::ByName(_) => return None,
                    };

                    let font = data_encoding::BASE64.encode(&bytes);
                    Some(format!(r#"<style>@font-face{{font-family:{font_name}{variant};src:url("data:{mime};base64,{font}");}}</style>"#))
                })
            })
            .flatten()
            .collect::<String>();

        let acc = format!(
            r#"<svg viewBox="{min_x} {min_y} {span_x} {span_y}" {SCHEME}><defs>{fonts}</defs>"#,
        );

        SVGExporter { acc }
    }

    fn write_style(&mut self, style: StylePosition) -> Result<(), SVGError> {
        match style.fill {
            Some(color) => write!(
                self.acc,
                "fill='rgb({} {} {} / {:.3})' ",
                color.red * 255.,
                color.green * 255.,
                color.blue * 255.,
                color.alpha
            )?, // pass [0;1] number to [0;255] for a working CSS code (not needed for alpha)

            None => write!(self.acc, "fill='none' ")?,
        }

        match style.stroke {
            Some(Stroke::Dashed {
                color,
                width,
                on,
                off,
            }) => write!(
                self.acc,
                "stroke='rgb({} {} {} / {:.3})' stroke-width='{width}' stroke-dasharray='{on},{off}' ",
                color.red * 255.,
                color.green * 255.,
                color.blue * 255.,
                color.alpha
            )?,
            Some(Stroke::Full { color, width }) => {
                write!(self.acc, "stroke='{:?}' stroke-width='{width}' ", color)?
            }

            None => {}
        }

        Ok(())
    }

    #[allow(unused)]
    fn write_curve(&mut self, curve: CurvePosition) -> Result<(), SVGError> {
        let mut has_start = false;

        for keypoint in &curve.keypoints {
            match keypoint {
                KeypointPosition::Point(p) => {
                    if has_start {
                        write!(self.acc, "L ")?;
                    } else {
                        write!(self.acc, "M ")?;
                        has_start = true;
                    }
                    write!(self.acc, "{} {} ", p.x, p.y)?;
                }
                KeypointPosition::Bezier(b) => {
                    if has_start {
                        if let Some(v) = b.start {
                            write!(self.acc, "L {} {} ", v.x, v.y)?;
                        }
                    } else {
                        if let Some(v) = b.start {
                            write!(self.acc, "M {} {} ", v.x, v.y)?;
                            has_start = true;
                        } else {
                            return Err(SVGError::CurveHasNoStartingPoint(curve));
                        }
                    }

                    write!(
                            self.acc,
                            "C {start_ctrl_x} {start_ctrl_y} {end_ctrl_x} {end_ctrl_y} {end_x} {end_y} ",
                            start_ctrl_x = b.start_control.x,
                            start_ctrl_y = b.start_control.y,
                            end_ctrl_x = b.end_control.x,
                            end_ctrl_y = b.end_control.y,
                            end_x = b.end.x,
                            end_y = b.end.y,
                        )?;
                }
            }

            has_start = true;
        }

        if curve.closed {
            write!(self.acc, "Z",)?;
        }

        Ok(())
    }

    fn finish(self) -> String {
        format!("{}</svg>", self.acc)
    }
}

impl Exporter for SVGExporter {
    type Error = SVGError;
    const CAN_EXPORT_ELLIPSE: bool = true;

    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> {
        write!(self.acc, "<g ")?;
        self.write_style(style)?;
        write!(self.acc, ">")?;

        Ok(())
    }

    fn end_style(&mut self) -> Result<(), Self::Error> {
        write!(self.acc, "</g>")?;
        Ok(())
    }

    fn start_block(&mut self, _metadata: &[(String, String)]) -> Result<(), Self::Error> {
        if !_metadata.is_empty() {
            write!(self.acc, "<g ")?;
            for (key, value) in _metadata {
                write!(self.acc, r#"{key}={value} "#)?;
            }
            write!(self.acc, ">")?;
        }

        Ok(())
    }

    fn end_block(&mut self, _metadata: &[(String, String)]) -> Result<(), Self::Error> {
        if !_metadata.is_empty() {
            write!(self.acc, "</g>")?;
        }
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
        let mut raw_image = Cursor::new(vec![]);
        image.write_to(&mut raw_image, ImageFormat::Png).unwrap();

        let data = data_encoding::BASE64.encode(&raw_image.into_inner());

        write!(
            self.acc,
            r#"<image width="{width}" height="{height}" x="{x}" y="{y}" "#,
            x = center.x - width / 2.,
            y = center.y - height / 2.,
        )?;

        if rotation.abs() > 10e-6 {
            write!(
                self.acc,
                r#" transform="rotate({rot})" "#,
                rot = (-rotation.to_degrees() + 360.) % 360.
            )?;
        }

        write!(self.acc, r#"href="data:image/png;base64,{data}"/>"#,)?;

        Ok(())
    }

    fn export_ellipse(
        &mut self,
        EllipsePosition {
            center,
            semi_major_axis,
            semi_minor_axis,
            rotation,
        }: EllipsePosition,
    ) -> Result<(), Self::Error> {
        write!(
            self.acc,
            r#"<ellipse rx="{semi_major_axis}" ry="{semi_minor_axis}" transform=""#,
        )?;

        write!(
            self.acc,
            r#"translate({cx} {cy}) "#,
            cx = center.x,
            cy = center.y
        )?;

        if rotation.abs() > 10e-6 {
            write!(self.acc, r#"rotate({rot}) "#, rot = -rotation.to_degrees())?;
        }

        write!(self.acc, r#""/>"#)?;

        Ok(())
    }

    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> {
        write!(self.acc, r#"<path d=""#)?;
        self.write_curve(curve)?;
        write!(self.acc, r#""/>"#)?;

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
        let id = rand::random::<u64>().to_string();

        let weight = match font_weight {
            FontWeight::Bold | FontWeight::BoldItalic => "bold",
            _ => "normal",
        };
        let text_style = match font_weight {
            FontWeight::Italic | FontWeight::BoldItalic => "italic",
            _ => "normal",
        };
        let align = match align {
            TextAlign::Center => "middle",
            TextAlign::Left => "start",
            TextAlign::Right => "end",
        };

        let text = text.replace("<", "&lt;").replace(">", "&gt;");
        let font = font
            .as_ref()
            .map(|v| v.name(font_weight))
            .unwrap_or_else(|| dessin::font::FontRef::default().name(font_weight));

        write!(
            self.acc,
            r#"<text font-family="{font}" text-anchor="{align}" font-size="{font_size}px" font-weight="{weight}" text-style="{text_style}" transform=""#,
        )?;

        write!(
            self.acc,
            r#"translate({cx} {cy}) "#,
            cx = reference_start.x,
            cy = reference_start.y
        )?;

        let rotation = direction.y.atan2(direction.x);
        if rotation.abs() > 10e-6 {
            write!(self.acc, r#"rotate({rot}) "#, rot = rotation.to_degrees())?;
        }

        write!(self.acc, r#"">"#)?;

        if let Some(curve) = on_curve {
            write!(self.acc, r#"<path id="{id}" d=""#)?;
            self.write_curve(curve)?;
            write!(self.acc, r#""/>"#)?;

            write!(self.acc, r##"<textPath href="#{id}">{text}</textPath>"##)?;
        } else {
            write!(self.acc, "{text}")?;
        }
        write!(self.acc, r#"</text>"#)?;

        Ok(())
    }
}

pub fn to_string_with_options(shape: &Shape, options: SVGOptions) -> Result<String, SVGError> {
    let (min_x, min_y, span_x, span_y) = match options.viewport {
        ViewPort::ManualCentered { width, height } => (-width / 2., -height / 2., width, height),
        ViewPort::ManualViewport {
            x,
            y,
            width,
            height,
        } => (x - width / 2., y - height / 2., width, height),
        ViewPort::AutoCentered => {
            let bb = shape.local_bounding_box().straigthen();

            let mirror_bb = bb
                .transform(&nalgebra::convert::<_, Transform2<f32>>(Scale2::new(
                    -1., -1.,
                )))
                .into_straight();

            let overall_bb = bb.join(mirror_bb);

            (
                -overall_bb.width() / 2.,
                -overall_bb.height() / 2.,
                overall_bb.width(),
                overall_bb.height(),
            )
        }
        ViewPort::AutoBoundingBox => {
            let bb = shape.local_bounding_box().straigthen();

            (bb.top_left().x, -bb.top_left().y, bb.width(), bb.height())
        }
    };

    let mut exporter = SVGExporter::new(min_x, min_y, span_x, span_y);

    let parent_transform = nalgebra::convert(Scale2::new(1., -1.));
    shape.write_into_exporter(&mut exporter, &parent_transform)?;

    Ok(exporter.finish())
}

pub fn to_string(shape: &Shape) -> Result<String, SVGError> {
    to_string_with_options(shape, SVGOptions::default())
}
