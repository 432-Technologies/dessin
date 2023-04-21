use dessin::prelude::*;
use nalgebra::{Transform2, Translation2};
use once_cell::sync::OnceCell;
use printpdf::{Mm, PdfDocument, PdfDocumentReference, PdfLayerReference};
use std::{
    io::{self},
    sync::{Arc, RwLock},
};

static FONT_HOLDER: OnceCell<Arc<RwLock<Vec<dessin::font::FontGroup<printpdf::IndirectFontRef>>>>> =
    OnceCell::new();

#[derive(Debug)]
pub enum PDFError {
    PrintPDF(printpdf::Error),
    WriteError(io::Error),
    CurveHasNoStartingPoint(Curve),
    UnknownBuiltinFont(String),
}
impl From<io::Error> for PDFError {
    fn from(e: io::Error) -> Self {
        PDFError::WriteError(e)
    }
}
impl From<printpdf::Error> for PDFError {
    fn from(e: printpdf::Error) -> Self {
        PDFError::PrintPDF(e)
    }
}

pub trait ToPDF {
    // fn write_raw_pdf<W: Write>(
    //     &self,
    //     w: &mut W,
    //     parent_transform: &Transform2<f32>,
    // ) -> Result<(), PDFError>;

    // fn write_pdf<W: Write>(&self, w: &mut W) -> Result<(), PDFError> {
    //     let max_x = 150.;
    //     let max_y = 150.;

    //     write!(
    //         w,
    //         r#"<pdf viewBox="{offset_x} {offset_y} {max_x} {max_y}" xmlns="http://www.w3.org/2000/pdf" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
    //         offset_x = -max_x / 2.,
    //         offset_y = -max_y / 2.,
    //     )?;

    //     self.write_raw_pdf(w, &Transform2::default())?;

    //     write!(w, "</pdf>")?;

    //     Ok(())
    // }

    /// Draw the `Dessin` on the given layer
    ///
    /// ```
    /// # use printpdf::{Mm, PdfDocument};
    /// # use dessin_pdf::{PDFError, ToPDF};
    /// # use dessin::prelude::*;
    /// # fn main() -> Result<(), PDFError> {
    ///
    /// // See https://docs.rs/printpdf/latest/printpdf/ for more infos
    /// let (doc, page1, layer1) =
    /// 	PdfDocument::new("printpdf graphics test", Mm(297.0), Mm(210.0), "Layer 1");
    /// let current_layer = doc.get_page(page1).get_layer(layer1);
    ///
    /// dessin!( /* Shapes here */ ).draw_on_layer(&current_layer)?;
    ///
    /// # 	Ok(());
    /// # }
    /// ```
    #[inline]
    fn draw_on_layer(
        &self,
        layer: &PdfLayerReference,
        width: f32,
        height: f32,
    ) -> Result<(), PDFError> {
        let translation = Translation2::new(width / 2., height / 2.);
        self.draw_on_layer_with_parent_transform(layer, &nalgebra::convert(translation))
    }

    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError>;

    fn to_pdf(&self, width: f32, height: f32) -> Result<PdfDocumentReference, PDFError> {
        let (doc, page, layer) = PdfDocument::new("", Mm(width as f64), Mm(height as f64), "Layer");

        for dessin::font::FontGroup {
            regular,
            bold,
            italic,
            bold_italic,
        } in dessin::font::fonts()
        {
            fn find_builtin_font(f: &str) -> Result<printpdf::BuiltinFont, PDFError> {
                match f {
                    "TimesRoman" => Ok(printpdf::BuiltinFont::TimesRoman),
                    "TimesBold" => Ok(printpdf::BuiltinFont::TimesBold),
                    "TimesItalic" => Ok(printpdf::BuiltinFont::TimesItalic),
                    "TimesBoldItalic" => Ok(printpdf::BuiltinFont::TimesBoldItalic),
                    "Helvetica" => Ok(printpdf::BuiltinFont::Helvetica),
                    "HelveticaBold" => Ok(printpdf::BuiltinFont::HelveticaBold),
                    "HelveticaOblique" => Ok(printpdf::BuiltinFont::HelveticaOblique),
                    "HelveticaBoldOblique" => Ok(printpdf::BuiltinFont::HelveticaBoldOblique),
                    "Courier" => Ok(printpdf::BuiltinFont::Courier),
                    "CourierOblique" => Ok(printpdf::BuiltinFont::CourierOblique),
                    "CourierBold" => Ok(printpdf::BuiltinFont::CourierBold),
                    "CourierBoldOblique" => Ok(printpdf::BuiltinFont::CourierBoldOblique),
                    "Symbol" => Ok(printpdf::BuiltinFont::Symbol),
                    "ZapfDingbats" => Ok(printpdf::BuiltinFont::ZapfDingbats),
                    _ => Err(PDFError::UnknownBuiltinFont(f.to_string())),
                }
            }

            let regular = match regular {
                dessin::font::Font::ByName(n) => doc.add_builtin_font(find_builtin_font(&n)?)?,
                dessin::font::Font::Bytes(b) => doc.add_external_font(b.as_slice())?,
            };

            let bold = match bold {
                Some(dessin::font::Font::ByName(n)) => {
                    Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                }
                Some(dessin::font::Font::Bytes(b)) => Some(doc.add_external_font(b.as_slice())?),
                None => None,
            };

            let italic = match italic {
                Some(dessin::font::Font::ByName(n)) => {
                    Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                }
                Some(dessin::font::Font::Bytes(b)) => Some(doc.add_external_font(b.as_slice())?),
                None => None,
            };

            let bold_italic = match bold_italic {
                Some(dessin::font::Font::ByName(n)) => {
                    Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                }
                Some(dessin::font::Font::Bytes(b)) => Some(doc.add_external_font(b.as_slice())?),
                None => None,
            };

            let fh = FONT_HOLDER.get_or_init(|| Arc::new(RwLock::new(vec![])));
            fh.write().unwrap().push(dessin::font::FontGroup {
                regular,
                bold,
                bold_italic,
                italic,
            });
        }

        let current_layer = doc.get_page(page).get_layer(layer);
        self.draw_on_layer(&current_layer, width, height)?;
        Ok(doc)
    }

    #[inline]
    fn to_pdf_bytes(&self, width: f32, height: f32) -> Result<Vec<u8>, PDFError> {
        let pdf = self.to_pdf(width, height)?;
        Ok(pdf.save_to_bytes()?)
    }
}

impl ToPDF for Shape {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        match self {
            Shape::Image(i) => i.draw_on_layer_with_parent_transform(layer, parent_transform),
            Shape::Text(t) => t.draw_on_layer_with_parent_transform(layer, parent_transform),
            Shape::Ellipse(e) => e.draw_on_layer_with_parent_transform(layer, parent_transform),
            Shape::Curve(c) => c.draw_on_layer_with_parent_transform(layer, parent_transform),
            Shape::Group {
                local_transform: _,
                shapes,
            } => {
                let transform = self.global_transform(parent_transform);

                for s in shapes {
                    s.draw_on_layer_with_parent_transform(layer, &transform)?;
                }

                Ok(())
            }
            Shape::Style {
                fill,
                stroke,
                shape,
            } => {
                if let Some(fill) = fill {
                    let (r, g, b) = match fill {
                        Fill::Color(c) => c.as_rgb_f64(),
                    };

                    layer.set_fill_color(printpdf::Color::Rgb(printpdf::Rgb {
                        r,
                        g,
                        b,
                        icc_profile: None,
                    }));
                }

                if let Some(stroke) = stroke {
                    let ((r, g, b), w) = match *parent_transform * *stroke {
                        Stroke::Full { color, width } => (color.as_rgb_f64(), width),
                        Stroke::Dashed {
                            color,
                            width,
                            on,
                            off,
                        } => (color.as_rgb_f64(), width),
                    };

                    layer.set_outline_color(printpdf::Color::Rgb(printpdf::Rgb {
                        r,
                        g,
                        b,
                        icc_profile: None,
                    }));

                    layer.set_outline_thickness(printpdf::Mm(w as f64).into_pt().0);
                }

                shape.draw_on_layer_with_parent_transform(layer, parent_transform)?;

                if stroke.is_some() {
                    layer.set_outline_color(printpdf::Color::Rgb(printpdf::Rgb {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        icc_profile: None,
                    }));
                    layer.set_outline_thickness(0.);
                }

                if fill.is_some() {
                    layer.set_fill_color(printpdf::Color::Rgb(printpdf::Rgb {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        icc_profile: None,
                    }));
                }

                Ok(())
            }
        }
    }
}

impl ToPDF for Curve {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        fn place_keypoints(
            curve: &Curve,
            parent_transform: &Transform2<f32>,
        ) -> Result<Vec<(printpdf::Point, bool)>, PDFError> {
            let CurvePosition {
                keypoints,
                closed: _,
            } = curve.position(parent_transform);

            let mut points = Vec::with_capacity(curve.keypoints.len());

            for idx in 0..keypoints.len() {
                let k = &keypoints[idx];
                let next_is_bezier = keypoints
                    .get(idx)
                    .map(|v| {
                        if let Keypoint::Bezier(Bezier { start: _, .. }) = v {
                            // start.is_none()
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);

                match k {
                    Keypoint::Curve(c) => {
                        let parent_transform = curve.global_transform(parent_transform);
                        points.extend(place_keypoints(c, &parent_transform)?);
                    }
                    Keypoint::Bezier(Bezier {
                        start,
                        start_control,
                        end_control,
                        end,
                    }) => {
                        if let Some(start) = start {
                            points.push((
                                printpdf::Point::new(Mm(start.x as f64), Mm(start.y as f64)),
                                true,
                            ));
                        }
                        points.push((
                            printpdf::Point::new(
                                Mm(start_control.x as f64),
                                Mm(start_control.y as f64),
                            ),
                            true,
                        ));
                        points.push((
                            printpdf::Point::new(
                                Mm(end_control.x as f64),
                                Mm(end_control.y as f64),
                            ),
                            true,
                        ));
                        points.push((
                            printpdf::Point::new(Mm(end.x as f64), Mm(end.y as f64)),
                            next_is_bezier,
                        ));
                    }
                    Keypoint::Point(p) => points.push((
                        printpdf::Point::new(Mm(p.x as f64), Mm(p.y as f64)),
                        next_is_bezier,
                    )),
                }
            }

            Ok(points)
        }

        let points = place_keypoints(self, parent_transform)?;

        let l = printpdf::Line {
            points,
            is_closed: self.closed,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        layer.add_shape(l);

        Ok(())
    }
}

impl ToPDF for Ellipse {
    #[inline]
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        Curve::from(self.clone()).draw_on_layer_with_parent_transform(layer, parent_transform)
    }
}

impl ToPDF for Text {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        let TextPosition {
            text,
            align,
            font_weight,
            on_curve,
            font_size,
            reference_start: bottom_left,
        } = self.position(parent_transform);

        let fonts = &FONT_HOLDER.get().unwrap().read().unwrap()[self.font.unwrap_or(0)];
        let font = match font_weight {
            FontWeight::Regular => &fonts.regular,
            FontWeight::Bold => fonts.bold.as_ref().unwrap_or_else(|| &fonts.regular),
            FontWeight::BoldItalic => fonts.bold_italic.as_ref().unwrap_or_else(|| &fonts.regular),
            FontWeight::Italic => fonts.italic.as_ref().unwrap_or_else(|| &fonts.regular),
        };

        if let Some(curve) = &self.on_curve {
            todo!()
        } else {
            layer.use_text(
                text,
                font_size as f64,
                Mm(bottom_left.x as f64),
                Mm(bottom_left.y as f64),
                font,
            );
        }

        Ok(())
    }
}

impl ToPDF for Image {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        let ImagePosition {
            center: _,
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left,
            width,
            height,
            rotation,
        } = self.position(parent_transform);
        let Image {
            image,
            local_transform: _,
        } = self;

        let width_px = image.width();
        let height_px = image.height();

        let dpi = 300.;
        let raw_width = width_px as f32 * 25.4 / dpi;
        let raw_height = height_px as f32 * 25.4 / dpi;

        let scale_width = width / raw_width;
        let scale_height = height / raw_height;

        println!("{dpi} {raw_width} {raw_height} {width_px} {height_px}");

        printpdf::Image::from_dynamic_image(image).add_to_layer(
            layer.clone(),
            printpdf::ImageTransform {
                translate_x: Some(Mm(bottom_left.x as f64)),
                translate_y: Some(Mm(bottom_left.y as f64)),
                rotate: Some(printpdf::ImageRotation {
                    angle_ccw_degrees: rotation.to_degrees() as f64,
                    rotation_center_x: printpdf::Px((width_px / 2) as usize),
                    rotation_center_y: printpdf::Px((height_px / 2) as usize),
                }),
                scale_x: Some(scale_width as f64),
                scale_y: Some(scale_height as f64),
                dpi: Some(dpi as f64),
            },
        );

        Ok(())
    }
}
