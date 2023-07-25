use dessin::{
    export::{Export, Exporter},
    prelude::*,
};
use image::{DynamicImage, ImageFormat};
use nalgebra::{Point2, Scale2, Transform2, Translation2, Vector2};
use std::{
    fmt::{self, Write},
    io::Cursor,
};

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
    buffer: DynamicImage,
}

impl ImageExporter {
    fn new(width: u32, height: u32) -> Self {
        ImageExporter {
            buffer: DynamicImage::new_rgba32f(width, height),
        }
    }

    fn finalize(self) -> DynamicImage {
        self.buffer
    }

    fn write_style(&mut self, style: StylePosition) -> Result<(), ImageError> {
        // match style.fill {
        //     Some(Fill::Color(color)) => write!(self.acc, "fill='{color}' ")?,
        //     None => write!(self.acc, "fill='none' ")?,
        // }

        // match style.stroke {
        //     Some(Stroke::Dashed {
        //         color,
        //         width,
        //         on,
        //         off,
        //     }) => write!(
        //         self.acc,
        //         "stroke='{color}' stroke-width='{width}' stroke-dasharray='{on},{off}' "
        //     )?,
        //     Some(Stroke::Full { color, width }) => {
        //         write!(self.acc, "stroke='{color}' stroke-width='{width}' ")?
        //     }

        //     None => {}
        // }

        Ok(())
    }

    fn write_curve(&mut self, curve: CurvePosition) -> Result<(), ImageError> {
        // let mut has_start = false;

        // for keypoint in &curve.keypoints {
        //     match keypoint {
        //         KeypointPosition::Point(p) => {
        //             if has_start {
        //                 write!(self.acc, "L ")?;
        //             } else {
        //                 write!(self.acc, "M ")?;
        //                 has_start = true;
        //             }
        //             write!(self.acc, "{} {} ", p.x, p.y)?;
        //         }
        //         KeypointPosition::Bezier(b) => {
        //             if has_start {
        //                 if let Some(v) = b.start {
        //                     write!(self.acc, "L {} {} ", v.x, v.y)?;
        //                 }
        //             } else {
        //                 if let Some(v) = b.start {
        //                     write!(self.acc, "M {} {} ", v.x, v.y)?;
        //                     has_start = true;
        //                 } else {
        //                     return Err(ImageError::CurveHasNoStartingPoint(curve));
        //                 }
        //             }

        //             write!(
        //                     self.acc,
        //                     "C {start_ctrl_x} {start_ctrl_y} {end_ctrl_x} {end_ctrl_y} {end_x} {end_y} ",
        //                     start_ctrl_x = b.start_control.x,
        //                     start_ctrl_y = b.start_control.y,
        //                     end_ctrl_x = b.end_control.x,
        //                     end_ctrl_y = b.end_control.y,
        //                     end_x = b.end.x,
        //                     end_y = b.end.y,
        //                 )?;
        //         }
        //     }

        // has_start = true;
        // }

        // if curve.closed {
        // write!(self.acc, "Z",)?;
        // }

        Ok(())
    }
}

impl Exporter for ImageExporter {
    type Error = ImageError;

    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> {
        // write!(self.acc, "<g ")?;
        // self.write_style(style)?;
        // write!(self.acc, ">")?;

        Ok(())
    }

    fn end_style(&mut self) -> Result<(), Self::Error> {
        // write!(self.acc, "</g>")?;
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

    fn export_ellipse(
        &mut self,
        EllipsePosition {
            center,
            semi_major_axis,
            semi_minor_axis,
            rotation,
        }: EllipsePosition,
    ) -> Result<(), Self::Error> {
        // write!(
        //     self.acc,
        //     r#"<ellipse cx="{cx}" cy="{cy}" rx="{semi_major_axis}" ry="{semi_minor_axis}" "#,
        //     cx = center.x,
        //     cy = center.y
        // )?;

        // if rotation.abs() > 10e-6 {
        //     write!(
        //         self.acc,
        //         r#" transform="rotate({rot})" "#,
        //         rot = -rotation.to_degrees()
        //     )?;
        // }

        // write!(self.acc, "/>")?;

        Ok(())
    }

    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> {
        // write!(self.acc, r#"<path d=""#)?;
        // self.write_curve(curve)?;
        // write!(self.acc, r#""/>"#)?;

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
        let transform = nalgebra::convert::<_, Transform2<f32>>(translation);

        let mut exporter = ImageExporter::new(bb.width().ceil() as u32, bb.height().ceil() as u32);

        self.write_into_exporter(&mut exporter, &transform)?;

        Ok(exporter.finalize())
    }
}
