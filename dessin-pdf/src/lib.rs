use dessin::{
	export::{Export, Exporter},
	font::FontRef,
	prelude::*,
};
use nalgebra::Translation2;
use printpdf::{
	Color, FontId, Layer, LayerIntent, LayerInternalId, LayerSubtype, Line, LinePoint, Mm, Op,
	PaintMode, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, Point, Polygon, PolygonRing, Px,
	RawImage, Rgb, TextItem, TextMatrix, TextRenderingMode, WindingOrder, XObjectRotation,
	XObjectTransform,
};
use std::{collections::HashMap, convert::identity, fmt};

#[derive(Debug, thiserror::Error)]
pub enum PDFError {
	#[error("PrintPDF Image error: {0}")]
	PrintPDFImageError(String),
	#[error("{0}")]
	WriteError(#[from] fmt::Error),
	#[error("Curve has no starting point: {0:?}")]
	CurveHasNoStartingPoint(Curve),
	#[error("Unknown builtin font: {0}")]
	UnknownBuiltinFont(String),
	#[error("Orphelin layer")]
	OrphelinLayer,
	#[error("Can't parse font `{0} {1:?}`")]
	CantParseFont(FontRef, FontWeight),
	#[error("Internal error: No layer started")]
	NoLayerStarted,
}

type PDFFontHolder = HashMap<(FontRef, FontWeight), FontId>;

#[derive(Default)]
pub struct PDFOptions {
	pub size: Option<(f32, f32)>,
	pub used_font: PDFFontHolder,
}

pub struct PDFExporter<'a> {
	doc: &'a mut PdfDocument,
	used_font: PDFFontHolder,
	content: Vec<Op>,
	layers: Vec<LayerInternalId>,
}
impl<'a> PDFExporter<'a> {
	pub fn new_with_font(doc: &'a mut PdfDocument, used_font: PDFFontHolder) -> Self {
		PDFExporter {
			doc,
			used_font,
			content: vec![],
			layers: vec![],
		}
	}

	pub fn new(doc: &'a mut PdfDocument) -> Self {
		PDFExporter::new_with_font(doc, HashMap::default())
	}
}

impl Exporter for PDFExporter<'_> {
	type Error = PDFError;

	const CAN_EXPORT_ELLIPSE: bool = false;

	fn start_style(
		&mut self,
		StylePosition { fill, stroke }: StylePosition,
	) -> Result<(), Self::Error> {
		let layer_id = self.doc.add_layer(&Layer {
			creator: String::new(),
			name: String::new(),
			intent: LayerIntent::Design,
			usage: LayerSubtype::Artwork,
		});

		self.content.push(Op::BeginLayer { layer_id });

		if let Some(fill) = fill {
			let (r, g, b) = match fill {
				Fill::Solid { color } => (
					color.into_format::<f32, f32>().red,
					color.into_format::<f32, f32>().green,
					color.into_format::<f32, f32>().blue,
				),
			};

			self.content.push(Op::SetFillColor {
				col: Color::Rgb(Rgb {
					r,
					g,
					b,
					icc_profile: None,
				}),
			});
		}

		if let Some(stroke) = stroke {
			let ((r, g, b), w) = match stroke {
				Stroke::Solid { color, width } => (
					(
						color.into_format::<f32, f32>().red,
						color.into_format::<f32, f32>().green,
						color.into_format::<f32, f32>().blue,
					),
					width,
				),
				Stroke::Dashed {
					color,
					width,
					on: _,
					off: _,
				} => {
					eprintln!("TODO: LineDashPattern");

					// self.content.push(printpdf::LineDashPattern {
					// 	offset: 0,
					// 	dash_1: Some(on as i64),
					// 	gap_1: Some(off as i64),
					// 	dash_2: None,
					// 	gap_2: None,
					// 	dash_3: None,
					// 	gap_3: None,
					// });

					(
						(
							color.into_format::<f32, f32>().red,
							color.into_format::<f32, f32>().green,
							color.into_format::<f32, f32>().blue,
						),
						width,
					)
				}
			};

			self.content.extend([
				Op::SetOutlineColor {
					col: Color::Rgb(Rgb {
						r,
						g,
						b,
						icc_profile: None,
					}),
				},
				Op::SetOutlineThickness {
					pt: printpdf::Mm(w).into_pt(),
				},
			]);
		}

		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		self.content.push(Op::EndLayer {
			layer_id: self.layers.pop().ok_or(PDFError::NoLayerStarted)?,
		});
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

		let img_id = self.doc.add_image(
			&RawImage::decode_from_bytes(image.as_bytes(), &mut vec![])
				.map_err(PDFError::PrintPDFImageError)?,
		);

		self.content.push(Op::UseXobject {
			id: img_id,
			transform: XObjectTransform {
				translate_x: Some(Mm(bottom_left.x).into()),
				translate_y: Some(Mm(bottom_left.y).into()),
				rotate: Some(XObjectRotation {
					angle_ccw_degrees: rotation.to_degrees(),
					rotation_center_x: Px((width_px / 2) as usize),
					rotation_center_y: Px((height_px / 2) as usize),
				}),
				scale_x: Some(scale_width),
				scale_y: Some(scale_height),
				dpi: Some(dpi),
			},
		});

		Ok(())
	}

	fn export_curve(
		&mut self,
		curve: CurvePosition,
		StylePosition { fill, stroke }: StylePosition,
	) -> Result<(), Self::Error> {
		let points = curve
			.keypoints
			.into_iter()
			.flat_map(|v| match v {
				KeypointPosition::Point(p) => [
					Some(LinePoint {
						p: Point::new(Mm(p.x), Mm(p.y)),
						bezier: false,
					}),
					None,
					None,
					None,
				]
				.into_iter(),
				KeypointPosition::Bezier(Bezier {
					start,
					start_control,
					end_control,
					end,
				}) => [
					start.map(|v| LinePoint {
						p: Point::new(Mm(v.x), Mm(v.y)),
						bezier: false,
					}),
					Some(LinePoint {
						p: Point::new(Mm(start_control.x), Mm(start_control.y)),
						bezier: true,
					}),
					Some(LinePoint {
						p: Point::new(Mm(end_control.x), Mm(end_control.y)),
						bezier: true,
					}),
					Some(LinePoint {
						p: Point::new(Mm(end.x), Mm(end.y)),
						bezier: true,
					}),
				]
				.into_iter(),
			})
			.filter_map(identity)
			.collect::<Vec<_>>();

		self.content.push(if curve.closed {
			Op::DrawPolygon {
				polygon: Polygon {
					mode: match (fill, stroke) {
						(Some(_), Some(_)) => PaintMode::FillStroke,
						(Some(_), None) => PaintMode::Fill,
						(None, Some(_)) => PaintMode::Stroke,
						(None, None) => PaintMode::Clip,
					},
					rings: vec![PolygonRing { points }],
					winding_order: WindingOrder::NonZero,
				},
			}
		} else {
			Op::DrawLine {
				line: Line {
					points,
					is_closed: false,
				},
			}
		});

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
		StylePosition { fill, stroke }: StylePosition,
	) -> Result<(), Self::Error> {
		let font = font.clone().unwrap_or(FontRef::default());

		let key = (font.clone(), font_weight);
		if !self.used_font.contains_key(&key) {
			match font::get(&font).get(font_weight) {
				dessin::font::Font::OTF(b) | dessin::font::Font::TTF(b) => {
					let font_id = self.doc.add_font(
						&ParsedFont::from_bytes(&b, 0, &mut vec![])
							.ok_or_else(|| PDFError::CantParseFont(font.clone(), font_weight))?,
					);

					self.used_font.insert(key.clone(), font_id);
				}
			}
		}

		let font = self.used_font[&key].clone();

		let rotation = direction.y.atan2(direction.x).to_degrees();

		self.content.extend([
			Op::SetLineHeight {
				lh: Mm(font_size).into_pt(),
			},
			Op::SetWordSpacing {
				pt: Mm(font_size).into_pt(),
			},
			Op::SetTextRenderingMode {
				mode: match (fill, stroke) {
					(Some(_), Some(_)) => TextRenderingMode::FillStroke,
					(Some(_), None) => TextRenderingMode::Fill,
					(None, Some(_)) => TextRenderingMode::Stroke,
					(None, None) => TextRenderingMode::Clip,
				},
			},
			Op::SetTextMatrix {
				matrix: TextMatrix::TranslateRotate(
					Mm(reference_start.x).into_pt(),
					Mm(reference_start.y).into_pt(),
					rotation,
				),
			},
			Op::WriteText {
				items: vec![TextItem::Text(text.to_string())],
				font,
			},
		]);

		Ok(())
	}
}

pub fn write_to_pdf_with_options(
	shape: &Shape,
	options: PDFOptions,
	doc: &mut PdfDocument,
) -> Result<PdfPage, PDFError> {
	let (width, height) = options.size.unwrap_or_else(|| {
		let bb = shape.local_bounding_box();
		(bb.width(), bb.height())
	});
	let mut exporter = PDFExporter::new_with_font(doc, options.used_font);
	let translation = Translation2::new(width / 2., height / 2.);
	let parent_transform = nalgebra::convert(translation);

	if let Shape::Style { fill, stroke, .. } = shape {
		shape.write_into_exporter(
			&mut exporter,
			&parent_transform,
			StylePosition {
				fill: *fill,
				stroke: *stroke,
			},
		)?
	} else {
		shape.write_into_exporter(
			&mut exporter,
			&parent_transform,
			StylePosition {
				fill: None,
				stroke: None,
			},
		)?
	}

	Ok(PdfPage::new(Mm(width), Mm(height), exporter.content))
}

pub fn to_pdf_with_options(shape: &Shape, options: PDFOptions) -> Result<PdfDocument, PDFError> {
	let mut doc = PdfDocument::new("");

	_ = write_to_pdf_with_options(shape, options, &mut doc)?;

	Ok(doc)
}

pub fn write_to_pdf(shape: &Shape, doc: &mut PdfDocument) -> Result<PdfPage, PDFError> {
	write_to_pdf_with_options(shape, PDFOptions::default(), doc)
}

pub fn to_pdf(shape: &Shape) -> Result<PdfDocument, PDFError> {
	to_pdf_with_options(shape, PDFOptions::default())
}

pub fn to_pdf_bytes(shape: &Shape) -> Result<Vec<u8>, PDFError> {
	Ok(to_pdf(shape)?.save(
		&PdfSaveOptions {
			optimize: true,
			secure: true,
			subset_fonts: true,
			..Default::default()
		},
		&mut vec![],
	))
}
