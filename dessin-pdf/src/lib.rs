use dessin::font::FontRef;
use dessin::{
    export::{Export, Exporter},
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
    OrphelinLayer,
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

type PDFFontHolder = HashMap<(FontRef, FontWeight), IndirectFontRef>;

#[derive(Default)]
pub struct PDFOptions {
    pub size: Option<(f32, f32)>,
    pub used_font: PDFFontHolder,
}

pub struct PDFExporter<'a> {
    layer: PdfLayerReference,
    doc: &'a PdfDocumentReference,
    used_font: PDFFontHolder,
}
impl<'a> PDFExporter<'a> {
    pub fn new_with_font(
        layer: PdfLayerReference,
        doc: &'a PdfDocumentReference,
        used_font: PDFFontHolder,
    ) -> Self {
        PDFExporter {
            layer,
            doc,
            used_font,
        }
    }
    pub fn new(layer: PdfLayerReference, doc: &'a PdfDocumentReference) -> Self {
        let stock: PDFFontHolder = HashMap::default();
        PDFExporter {
            layer,
            doc,
            used_font: stock,
        }
    }
}

impl Exporter for PDFExporter<'_> {
    type Error = PDFError;
    const CAN_EXPORT_ELLIPSE: bool = false;

    fn start_style(
        &mut self,
        StylePosition { fill, stroke }: StylePosition,
    ) -> Result<(), Self::Error> {
        if let Some(fill) = fill {
            let (r, g, b) = match fill {
                Fill::Color(c) => c.as_rgb_f32(),
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

    fn export_text(
        &mut self,
        TextPosition {
            text,
            align: _,
            font_weight,
            on_curve: _,
            font_size,
            reference_start,
            direction,
            font,
        }: TextPosition,
    ) -> Result<(), Self::Error> {
        let font = font.clone().unwrap_or(FontRef::default());

        // search if (font_ref, font_weight) is stocked in used_font
        let font = self
            .used_font
            .entry((font.clone(), font_weight))
            .or_insert_with(|| match font::get(font.clone()).get(font_weight) {
                dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b) => {
                    // self.doc.add_external_font(b.as_slice())? --
                    if let Err(err) = self.doc.add_external_font(b.as_slice()) {
                        // eprintln!("Failed to add external font : {}", err); --
                        panic!("Failed to add external font : {}", err)
                    } else {
                        self.doc.add_external_font(b.as_slice()).unwrap()
                    }
                }
            });

        // let will_survive = font::get(font); --
        // let font = will_survive.get(font_weight);
        // let font: IndirectFontRef = match font {
        //     dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b) => {
        //         self.doc.add_external_font(b.as_slice())?
        //     }
        // }; --

        self.layer.begin_text_section();
        self.layer.set_font(&font, font_size);
        // if let Some(te) = text.on_curve {
        //     self.layer.add_polygon()
        //     todo!()
        // }
        let rotation = direction.y.atan2(direction.x).to_degrees();
        self.layer
            .set_text_rendering_mode(printpdf::TextRenderingMode::Fill);
        self.layer
            .set_text_matrix(printpdf::TextMatrix::TranslateRotate(
                Mm(reference_start.x).into_pt(),
                Mm(reference_start.y).into_pt(),
                rotation,
            ));

        self.layer.write_text(text, &font);
        self.layer.end_text_section();

        Ok(())
    }
}

pub trait ToPDF {
    fn write_to_pdf_with_options(
        &self,
        layer: PdfLayerReference,
        options: PDFOptions,
        doc: &PdfDocumentReference,
    ) -> Result<(), PDFError>;
    #[inline]
    fn write_to_pdf(
        &self,
        layer: PdfLayerReference,
        doc: &PdfDocumentReference,
    ) -> Result<(), PDFError> {
        self.write_to_pdf_with_options(layer, PDFOptions::default(), doc)
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
        doc: &PdfDocumentReference,
    ) -> Result<(), PDFError> {
        let (width, height) = options.size.unwrap_or_else(|| {
            let bb = self.local_bounding_box();
            (bb.width(), bb.height())
        });
        let mut exporter = PDFExporter::new_with_font(layer, doc, options.used_font);
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

        self.write_to_pdf_with_options(layer, options, &doc)?;

        Ok(doc)
    }
}
