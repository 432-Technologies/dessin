use dessin::{
    export::{Export, Exporter},
    font::FontGroup,
    prelude::*,
};
use nalgebra::Translation2;
use printpdf::{
    BuiltinFont, IndirectFontRef, Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference,
    Point,
};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum PDFError {
    PrintPDF(printpdf::Error),
    WriteError(fmt::Error),
    CurveHasNoStartingPoint(Curve),
    UnknownBuiltinFont(String),
}
impl From<fmt::Error> for PDFError {
    fn from(e: fmt::Error) -> Self {
        PDFError::WriteError(e)
    }
}
impl From<printpdf::Error> for PDFError {
    fn from(e: printpdf::Error) -> Self {
        PDFError::PrintPDF(e)
    }
}

type PDFFontHolder = HashMap<String, FontGroup<IndirectFontRef>>;

#[derive(Default)]
pub struct PDFOptions {
    pub size: Option<(f32, f32)>,
    pub fonts: PDFFontHolder,
}

pub struct PDFExporter {
    layer: PdfLayerReference,
    fonts: PDFFontHolder,
}
impl PDFExporter {
    pub fn new(layer: PdfLayerReference, fonts: PDFFontHolder) -> Self {
        PDFExporter { layer, fonts }
    }
    pub fn new_with_default_font(layer: PdfLayerReference) -> Self {
        PDFExporter {
            layer,
            fonts: PDFFontHolder::default(),
        }
    }
}

impl Exporter for PDFExporter {
    type Error = PDFError;
    const CAN_EXPORT_ELLIPSE: bool = false;

    fn start_style(
        &mut self,
        StylePosition { fill, stroke }: StylePosition,
    ) -> Result<(), Self::Error> {
        if let Some(fill) = fill {
            let (r, g, b) = match fill {
                Fill::Color(c) => (
                    c.into_format::<f32, f32>().red,
                    c.into_format::<f32, f32>().green,
                    c.into_format::<f32, f32>().blue,
                ),
            };

            self.layer
                .set_fill_color(printpdf::Color::Rgb(printpdf::Rgb {
                    r,
                    g,
                    b,
                    icc_profile: None,
                }));
        }

        if let Some(stroke) = stroke {
            let ((r, g, b), w) = match stroke {
                Stroke::Full { color, width } => (color.as_rgb_f32(), width),
                Stroke::Dashed {
                    color,
                    width,
                    on,
                    off,
                } => {
                    self.layer.set_line_dash_pattern(printpdf::LineDashPattern {
                        offset: 0,
                        dash_1: Some(on as i64),
                        gap_1: Some(off as i64),
                        dash_2: None,
                        gap_2: None,
                        dash_3: None,
                        gap_3: None,
                    });

                    (color.as_rgb_f32(), width)
                }
            };

            self.layer
                .set_outline_color(printpdf::Color::Rgb(printpdf::Rgb {
                    r,
                    g,
                    b,
                    icc_profile: None,
                }));

            self.layer
                .set_outline_thickness(printpdf::Mm(w).into_pt().0);
        }

        Ok(())
    }

    fn end_style(&mut self) -> Result<(), Self::Error> {
        self.layer
            .set_outline_color(printpdf::Color::Rgb(printpdf::Rgb {
                r: 0.,
                g: 0.,
                b: 0.,
                icc_profile: None,
            }));
        self.layer.set_outline_thickness(0.);
        self.layer.set_line_dash_pattern(printpdf::LineDashPattern {
            offset: 0,
            dash_1: None,
            gap_1: None,
            dash_2: None,
            gap_2: None,
            dash_3: None,
            gap_3: None,
        });

        self.layer
            .set_fill_color(printpdf::Color::Rgb(printpdf::Rgb {
                r: 0.,
                g: 0.,
                b: 0.,
                icc_profile: None,
            }));

        Ok(())
    }

    fn export_image(
        &mut self,
        ImagePosition {
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left,
            center: _,
            width,
            height,
            rotation,
            image,
        }: ImagePosition,
    ) -> Result<(), Self::Error> {
        let width_px = image.width();
        let height_px = image.height();

        let dpi = 300.;
        let raw_width = width_px as f32 * 25.4 / dpi;
        let raw_height = height_px as f32 * 25.4 / dpi;

        let scale_width = width / raw_width;
        let scale_height = height / raw_height;

        printpdf::Image::from_dynamic_image(image).add_to_layer(
            self.layer.clone(),
            printpdf::ImageTransform {
                translate_x: Some(Mm(bottom_left.x)),
                translate_y: Some(Mm(bottom_left.y)),
                rotate: Some(printpdf::ImageRotation {
                    angle_ccw_degrees: rotation.to_degrees(),
                    rotation_center_x: printpdf::Px((width_px / 2) as usize),
                    rotation_center_y: printpdf::Px((height_px / 2) as usize),
                }),
                scale_x: Some(scale_width),
                scale_y: Some(scale_height),
                dpi: Some(dpi),
            },
        );

        Ok(())
    }

    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> {
        let points1 = curve
            .keypoints
            .iter()
            .enumerate()
            .flat_map(|(i, key_point)| {
                let next_control = matches!(curve.keypoints.get(i + 1), Some(KeypointPosition::Bezier(b)) if b.start.is_none());
                match key_point {
                    KeypointPosition::Point(p) => {
                        vec![(Point::new(Mm(p.x), Mm(p.y)), next_control)]
                    }
                    KeypointPosition::Bezier(b) => {
                        let mut res = vec![];
                        if let Some(start) = b.start {
                            res.push((Point::new(Mm(start.x), Mm(start.y)), true));
                        }
                        res.append(&mut vec![
                                (
                                    Point::new(Mm(b.start_control.x), Mm(b.start_control.y)),
                                    true,
                                ),
                                (Point::new(Mm(b.end_control.x), Mm(b.end_control.y)), false),
                                (Point::new(Mm(b.end.x), Mm(b.end.y)), next_control),
                            ]);
                        res
                    }
                }
            })
            .collect();

        let line = Line {
            points: points1,
            is_closed: curve.closed,
        };
        self.layer.add_line(line);
        Ok(())
    }

    fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error> {
        let font = text
            .font
            .as_ref()
            .map(|f| f.font_family())
            .unwrap_or("default");
        let font = self
            .fonts
            .get(font)
            .and_then(|font| match text.font_weight {
                FontWeight::Regular => Some(font.regular.clone()),
                FontWeight::Bold => font.bold.clone(),
                FontWeight::Italic => font.italic.clone(),
                FontWeight::BoldItalic => font.bold_italic.clone(),
            })
            .unwrap();
        self.layer.begin_text_section();
        self.layer.set_font(&font, text.font_size);
        // if let Some(te) = text.on_curve {
        //     self.layer.add_polygon()
        //     todo!()
        // }
        let rotation = text.direction.y.atan2(text.direction.x).to_degrees();
        self.layer
            .set_text_rendering_mode(printpdf::TextRenderingMode::Fill);
        self.layer
            .set_text_matrix(printpdf::TextMatrix::TranslateRotate(
                Mm(text.reference_start.x).into_pt(),
                Mm(text.reference_start.y).into_pt(),
                rotation,
            ));

        // self.layer.set_line_height(text.font_size);
        // self.layer.set_word_spacing(3000.0);
        // self.layer.set_character_spacing(10.0);
        self.layer.write_text(text.text, &font);
        self.layer.end_text_section();
        Ok(())
    }
}

pub trait ToPDF {
    fn write_to_pdf_with_options(
        &self,
        layer: PdfLayerReference,
        options: PDFOptions,
    ) -> Result<(), PDFError>;
    #[inline]
    fn write_to_pdf(&self, layer: PdfLayerReference) -> Result<(), PDFError> {
        self.write_to_pdf_with_options(layer, PDFOptions::default())
    }

    fn to_pdf_with_options(&self, options: PDFOptions) -> Result<PdfDocumentReference, PDFError>;
    #[inline]
    fn to_pdf_bytes_with_options(&self, options: PDFOptions) -> Result<Vec<u8>, PDFError> {
        Ok(self.to_pdf_with_options(options)?.save_to_bytes()?)
    }

    #[inline]
    fn to_pdf(&self) -> Result<PdfDocumentReference, PDFError> {
        self.to_pdf_with_options(PDFOptions::default())
    }
    #[inline]
    fn to_pdf_bytes(&self) -> Result<Vec<u8>, PDFError> {
        Ok(self.to_pdf()?.save_to_bytes()?)
    }
}
impl ToPDF for Shape {
    fn write_to_pdf_with_options(
        &self,
        layer: PdfLayerReference,
        options: PDFOptions,
    ) -> Result<(), PDFError> {
        let (width, height) = options.size.unwrap_or_else(|| {
            let bb = self.local_bounding_box();
            (bb.width(), bb.height())
        });
        let mut exporter = PDFExporter::new(layer, options.fonts);
        let translation = Translation2::new(width / 2., height / 2.);
        let parent_transform = nalgebra::convert(translation);

        self.write_into_exporter(&mut exporter, &parent_transform)
    }

    fn to_pdf_with_options(
        &self,
        mut options: PDFOptions,
    ) -> Result<PdfDocumentReference, PDFError> {
        let size = options.size.get_or_insert_with(|| {
            let bb = self.local_bounding_box();
            (bb.width(), bb.height())
        });
        let (doc, page, layer) = PdfDocument::new("", Mm(size.0), Mm(size.1), "Layer 1");
        let layer = doc.get_page(page).get_layer(layer);
        let default_regular = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        let default_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
        let default_italic = doc.add_builtin_font(BuiltinFont::HelveticaOblique).unwrap();
        let default_bold_italic = doc
            .add_builtin_font(BuiltinFont::HelveticaBoldOblique)
            .unwrap();
        options.fonts.insert(
            "default".to_string(),
            dessin::font::FontGroup {
                regular: default_regular,
                bold: Some(default_bold),
                bold_italic: Some(default_bold_italic),
                italic: Some(default_italic),
            },
        );

        for (
            key,
            dessin::font::FontGroup {
                regular,
                bold,
                italic,
                bold_italic,
            },
        ) in dessin::font::fonts()
        {
            let regular = match regular {
                // dessin::font::Font::ByName(n) => doc.add_builtin_font(find_builtin_font(&n)?)?,
                dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b) => {
                    doc.add_external_font(b.as_slice())?
                }
            };

            let bold = match bold {
                // Some(dessin::font::Font::ByName(n)) => {
                //     Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                // }
                Some(dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b)) => {
                    Some(doc.add_external_font(b.as_slice())?)
                }
                None => None,
            };

            let italic = match italic {
                // Some(dessin::font::Font::ByName(n)) => {
                //     Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                // }
                Some(dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b)) => {
                    Some(doc.add_external_font(b.as_slice())?)
                }
                None => None,
            };

            let bold_italic = match bold_italic {
                // Some(dessin::font::Font::ByName(n)) => {
                //     Some(doc.add_builtin_font(find_builtin_font(&n)?)?)
                // }
                Some(dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b)) => {
                    Some(doc.add_external_font(b.as_slice())?)
                }
                None => None,
            };
            let fonts_group = dessin::font::FontGroup {
                regular,
                bold,
                bold_italic,
                italic,
            };
            options.fonts.insert(key, fonts_group);
        }
        self.write_to_pdf_with_options(layer, options)?;

        Ok(doc)
    }
}

// impl ToPDF for Curve {
//     fn draw_on_layer_with_parent_transform(
//         &self,
//         layer: &PdfLayerReference,
//         parent_transform: &Transform2<f32>,
//     ) -> Result<(), PDFError> {
//         fn place_keypoints(
//             curve: &Curve,
//             parent_transform: &Transform2<f32>,
//         ) -> Result<Vec<(printpdf::Point, bool)>, PDFError> {
//             let CurvePosition {
//                 keypoints,
//                 closed: _,
//             } = curve.position(parent_transform);

//             let mut points = Vec::with_capacity(curve.keypoints.len());

//             for idx in 0..keypoints.len() {
//                 let k = &keypoints[idx];
//                 let next_is_bezier = keypoints
//                     .get(idx)
//                     .map(|v| {
//                         if let Keypoint::Bezier(Bezier { start: _, .. }) = v {
//                             // start.is_none()
//                             true
//                         } else {
//                             false
//                         }
//                     })
//                     .unwrap_or(false);

//                 match k {
//                     Keypoint::Curve(c) => {
//                         let parent_transform = curve.global_transform(parent_transform);
//                         points.extend(place_keypoints(c, &parent_transform)?);
//                     }
//                     Keypoint::Bezier(Bezier {
//                         start,
//                         start_control,
//                         end_control,
//                         end,
//                     }) => {
//                         if let Some(start) = start {
//                             points.push((
//                                 printpdf::Point::new(Mm(start.x as f64), Mm(start.y as f64)),
//                                 true,
//                             ));
//                         }
//                         points.push((
//                             printpdf::Point::new(
//                                 Mm(start_control.x as f64),
//                                 Mm(start_control.y as f64),
//                             ),
//                             true,
//                         ));
//                         points.push((
//                             printpdf::Point::new(
//                                 Mm(end_control.x as f64),
//                                 Mm(end_control.y as f64),
//                             ),
//                             true,
//                         ));
//                         points.push((
//                             printpdf::Point::new(Mm(end.x as f64), Mm(end.y as f64)),
//                             next_is_bezier,
//                         ));
//                     }
//                     Keypoint::Point(p) => points.push((
//                         printpdf::Point::new(Mm(p.x as f64), Mm(p.y as f64)),
//                         next_is_bezier,
//                     )),
//                 }
//             }

//             Ok(points)
//         }

//         let points = place_keypoints(self, parent_transform)?;

//         let l = printpdf::Line {
//             points,
//             is_closed: self.closed,
//             has_fill: false,
//             has_stroke: true,
//             is_clipping_path: false,
//         };

//         layer.add_shape(l);

//         Ok(())
//     }
// }

// impl ToPDF for Ellipse {
//     #[inline]
//     fn draw_on_layer_with_parent_transform(
//         &self,
//         layer: &PdfLayerReference,
//         parent_transform: &Transform2<f32>,
//     ) -> Result<(), PDFError> {
//         Curve::from(self.clone()).draw_on_layer_with_parent_transform(layer, parent_transform)
//     }
// }

// impl ToPDF for Text {
//     fn draw_on_layer_with_parent_transform(
//         &self,
//         layer: &PdfLayerReference,
//         parent_transform: &Transform2<f32>,
//     ) -> Result<(), PDFError> {
//         let TextPosition {
//             text,
//             align,
//             font_weight,
//             on_curve,
//             font_size,
//             reference_start: bottom_left,
//         } = self.position(parent_transform);

//         let fonts = &FONT_HOLDER.get().unwrap().read().unwrap()[self.font.unwrap_or(0)];
//         let font = match font_weight {
//             FontWeight::Regular => &fonts.regular,
//             FontWeight::Bold => fonts.bold.as_ref().unwrap_or_else(|| &fonts.regular),
//             FontWeight::BoldItalic => fonts.bold_italic.as_ref().unwrap_or_else(|| &fonts.regular),
//             FontWeight::Italic => fonts.italic.as_ref().unwrap_or_else(|| &fonts.regular),
//         };

//         if let Some(curve) = &self.on_curve {
//             todo!()
//         } else {
//             layer.use_text(
//                 text,
//                 font_size as f64,
//                 Mm(bottom_left.x as f64),
//                 Mm(bottom_left.y as f64),
//                 font,
//             );
//         }

//         Ok(())
//     }
// }
