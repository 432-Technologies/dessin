use ::image::ImageFormat;
use dessin::{
    export::{Export, Exporter},
    prelude::*,
};
use nalgebra::Scale2;
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

#[derive(Default)]
pub struct SVGOptions {
    pub size: Option<(f32, f32)>,
}

pub struct SVGExporter {
    acc: String,
}

impl SVGExporter {
    fn new((max_x, max_y): (f32, f32)) -> Self {
        SVGExporter {
            acc: format!(
                r#"<svg viewBox="{offset_x} {offset_y} {max_x} {max_y}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
                offset_x = -max_x / 2.,
                offset_y = -max_y / 2.,
            ),
        }
    }

    fn write_style(&mut self, style: StylePosition) -> Result<(), SVGError> {
        match style.fill {
            Some(Fill::Color(color)) => write!(self.acc, "fill='{color}'")?,
            None => write!(self.acc, "fill='none'")?,
        }

        match style.stroke {
            Some(Stroke::Dashed {
                color,
                width,
                on,
                off,
            }) => write!(
                self.acc,
                "stroke='{color}' stroke-width='{width}' stroke-dasharray='{on},{off}'"
            )?,
            Some(Stroke::Full { color, width }) => {
                write!(self.acc, "stroke='{color}' stroke-width='{width}'")?
            }

            None => {}
        }

        Ok(())
    }

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
                KeypointPosition::Close => {
                    if has_start {
                        write!(self.acc, "Z ",)?;
                    } else {
                        return Err(SVGError::CurveHasNoStartingPoint(curve));
                    }
                }
            }

            has_start = true;
        }

        Ok(())
    }

    fn finish(self) -> String {
        format!("{}</svg>", self.acc)
    }
}

impl Exporter for SVGExporter {
    type Error = SVGError;

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
        image.write_to(&mut raw_image, ImageFormat::Png).unwrap(); // TODO: Parse Image format

        let data = data_encoding::BASE64URL.encode(&raw_image.into_inner());

        write!(
            self.acc,
            r#"<image width="{width}" height="{height}" x="{x}" y="{y}" transform="rotate({rotation}rad)" href="data:image/png;base64,{data}" xlink:href="data:image/png;base64,{data}"/>"#,
            x = center.x,
            y = center.y,
        )?;

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
            r#"<ellipse cx="{cx}" cy="{cy}" rx="{semi_major_axis}" ry="{semi_minor_axis}""#,
            cx = center.x,
            cy = center.y
        )?;

        if rotation != 0. {
            write!(
                self.acc,
                r#" transform="rotate({rot}rad)""#,
                rot = -rotation.to_degrees()
            )?;
        }

        write!(self.acc, "/>")?;

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

        write!(
            self.acc,
            r#"<text x="{x}" y="{y}" text-anchor="{align}" font-size="{font_size}px" font-weight="{weight}" text-style="{text_style}">"#,
            x = reference_start.x,
            y = reference_start.y,
        )?;
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

pub trait ToSVG {
    fn to_svg_with_options(&self, options: SVGOptions) -> Result<String, SVGError>;

    fn to_svg(&self) -> Result<String, SVGError> {
        self.to_svg_with_options(SVGOptions::default())
    }
}

impl ToSVG for Shape {
    fn to_svg_with_options(&self, options: SVGOptions) -> Result<String, SVGError> {
        let size = options.size.unwrap_or_else(|| {
            let bb = self
                .local_bounding_box()
                .unwrap_or_else(|| BoundingBox::zero().as_unparticular());
            (bb.width(), bb.height())
        });

        let mut exporter = SVGExporter::new(size);

        let parent_transform = nalgebra::convert(Scale2::new(1., -1.));
        self.write_into_exporter(&mut exporter, &parent_transform)?;

        Ok(exporter.finish())
    }
}
