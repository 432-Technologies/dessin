use dessin::prelude::*;
use nalgebra::{Transform2, Translation2};
use printpdf::{PdfDocument, PdfDocumentReference, PdfLayerReference};
use std::io::{self, Cursor, Write};

const DPI: f64 = 96.;
const ARIAL_REGULAR: &[u8] = include_bytes!("Arial.ttf");
const ARIAL_BOLD: &[u8] = include_bytes!("Arial Bold.ttf");
const ARIAL_ITALIC: &[u8] = include_bytes!("Arial Italic.ttf");
const ARIAL_BOLD_ITALIC: &[u8] = include_bytes!("Arial Bold Italic.ttf");

#[derive(Debug)]
pub enum PDFError {
    WriteError(io::Error),
    CurveHasNoStartingPoint(Curve),
}
impl From<io::Error> for PDFError {
    fn from(e: io::Error) -> Self {
        PDFError::WriteError(e)
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

    // fn to_pdf(&self) -> Result<Vec<u8>, PDFError> {
    //     let mut res = Cursor::new(vec![]);
    //     let (doc, page1, layer1) =
    //         PdfDocument::new("printpdf graphics test", Mm(297.0), Mm(210.0), "Layer 1");
    //     let current_layer = doc.get_page(page1).get_layer(layer1);

    //     self.write_pdf(&mut res)?;
    //     let mut pdf = PdfDocument::new("");
    //     Ok(unsafe { String::from_utf8_unchecked(res.into_inner()) })
    // }
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
                    }))
                }

                if let Some(stroke) = stroke {
                    let ((r, g, b), w) = match stroke {
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
                    }))
                }

                shape.draw_on_layer_with_parent_transform(layer, parent_transform)?;

                if stroke.is_some() {
                    layer.restore_graphics_state();
                    layer.restore_graphics_state();
                }

                if fill.is_some() {
                    layer.restore_graphics_state();
                }

                Ok(())
            }
            x => todo!("{x:?}"),
        }
    }
}

impl ToPDF for Curve {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        let CurvePosition { keypoints, closed } = self.position(parent_transform);

        let mut points = Vec::with_capacity(self.keypoints.len());

        for idx in 0..keypoints.len() {
            let k = &keypoints[idx];
            let next_is_bezier = keypoints
                .get(idx)
                .map(|v| {
                    if let Keypoint::Bezier(Bezier { start, .. }) = v {
                        // start.is_none()
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false);

            match k {
                Keypoint::Curve(c) => {
                    todo!()
                }
                Keypoint::Bezier(Bezier {
                    start,
                    start_control,
                    end_control,
                    end,
                }) => {
                    if let Some(start) = start {
                        points.push((
                            printpdf::Point::new(
                                printpdf::Mm(start.x as f64),
                                printpdf::Mm(start.y as f64),
                            ),
                            true,
                        ));
                    }
                    points.push((
                        printpdf::Point::new(
                            printpdf::Mm(start_control.x as f64),
                            printpdf::Mm(start_control.y as f64),
                        ),
                        true,
                    ));
                    points.push((
                        printpdf::Point::new(
                            printpdf::Mm(end_control.x as f64),
                            printpdf::Mm(end_control.y as f64),
                        ),
                        true,
                    ));
                    points.push((
                        printpdf::Point::new(
                            printpdf::Mm(end.x as f64),
                            printpdf::Mm(end.y as f64),
                        ),
                        next_is_bezier,
                    ));
                }
                Keypoint::Point(p) => points.push((
                    printpdf::Point::new(printpdf::Mm(p.x as f64), printpdf::Mm(p.y as f64)),
                    next_is_bezier,
                )),
            }
        }

        let l = printpdf::Line {
            points,
            is_closed: closed,
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
        self.as_curve()
            .draw_on_layer_with_parent_transform(layer, parent_transform)
    }
}

impl ToPDF for Text {
    fn draw_on_layer_with_parent_transform(
        &self,
        layer: &PdfLayerReference,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), PDFError> {
        let pos = self.position(parent_transform);

        if let Some(curve) = &self.on_curve {
        } else {
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
            top_left: _,
            top_right,
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

        let width = width / raw_width;
        let height = height / raw_height;

        let center_x = (top_right.x + bottom_left.x) / 2.;
        let center_y = (top_right.y + bottom_left.y) / 2.;

        printpdf::Image::from_dynamic_image(image).add_to_layer(
            layer.clone(),
            printpdf::ImageTransform {
                translate_x: Some(printpdf::Mm(center_x as f64)),
                translate_y: Some(printpdf::Mm(center_y as f64)),
                rotate: Some(printpdf::ImageRotation {
                    angle_ccw_degrees: rotation.to_degrees() as f64,
                    rotation_center_x: printpdf::Px((width_px / 2) as usize),
                    rotation_center_y: printpdf::Px((height_px / 2) as usize),
                }),
                scale_x: Some(width as f64),
                scale_y: Some(height as f64),
                dpi: Some(dpi as f64),
            },
        );

        Ok(())
    }
}
// impl ToPDF for d

// pub struct PDF(pub PdfDocumentReference);
// impl PDF {
//     pub fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
//         let mut buff = BufWriter::new(vec![]);
//         self.0.save(&mut buff)?;
//         Ok(buff.into_inner()?)
//     }
// }

// pub trait ToPDF {
//     fn to_pdf(&self) -> Result<PDF, Box<dyn Error>>;
// }

// impl ToPDF for Drawing {
//     fn to_pdf(&self) -> Result<PDF, Box<dyn Error>> {
//         let Vec2 {
//             x: width,
//             y: height,
//         } = self.canvas_size();

//         let (doc, page1, layer1) =
//             PdfDocument::new("PDF", Mm(width as f64), Mm(height as f64), "Layer1");

//         let font = doc.add_external_font(ARIAL_REGULAR)?;
//         let current_layer = doc.get_page(page1).get_layer(layer1);

//         let offset = vec2(self.canvas_size().x / 2., self.canvas_size().x / 2.);

//         self.shapes()
//             .iter()
//             .map(|v| v.to_pdf_part(DPI, offset, &font, &current_layer))
//             .collect::<Result<(), Box<dyn std::error::Error>>>()?;

//         Ok(PDF(doc))
//     }
// }

// trait ToPDFPart {
//     fn to_pdf_part(
//         &self,
//         dpi: f64,
//         offset: Vec2,
//         font: &IndirectFontRef,
//         layer: &PdfLayerReference,
//     ) -> Result<(), Box<dyn Error>>;
// }
