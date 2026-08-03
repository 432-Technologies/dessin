use dessin::export::ViewPort;
use dessin::palette::Srgba;
use dessin::{
	export::{Export, Exporter},
	font::FontRef,
	prelude::*,
};
use krilla::{
	color::rgb,
	geom::{Path, PathBuilder, Point, Size, Transform},
	image::Image,
	num::NormalizedF32,
	page::PageSettings,
	paint::{
		Fill as KrillaFill, FillRule, LineCap, LineJoin, Paint, Stroke as KrillaStroke, StrokeDash,
	},
	text::{Font, Tag, TextDirection},
	Document,
};
use nalgebra::{Scale2, Transform2, Translation2};
use std::{collections::HashMap, fmt, fs};

pub mod reexport {
	pub use krilla;
}

/// PDF export errors.
#[derive(Debug, thiserror::Error)]
pub enum PdfError {
	#[error("Krilla Font error")]
	KrillaFontError,
	#[error("Krilla Serialize error: {0}")]
	KrillaSerializeError(String),
	#[error("Curve has no starting point: {0:?}")]
	CurveHasNoStartingPoint(Curve),
	#[error("Unknown builtin font: {0}")]
	UnknownBuiltinFont(String),
	#[error("Can't parse font `{0}`")]
	CantParseFont(String),
	#[error("Internal error: No layer started")]
	NoLayerStarted,
	#[error("Can't load font")]
	CantLoadFont(#[source] std::io::Error),
	#[error("No default font set")]
	NoDefaultFont,
	#[error("Unknown font")]
	UnknownFont(String),
	#[error("Write error")]
	WriteError(#[source] fmt::Error),
}

/// Helper to convert an f32 color component (0.0–1.0) to u8 (0–255).
fn to_u8(v: f32) -> u8 {
	(v.clamp(0.0, 1.0) * 255.0) as u8
}

/// Convert a palette Srgba color to a krilla rgb::Color.
fn to_krilla_color(color: Srgba) -> rgb::Color {
	rgb::Color::new(to_u8(color.red), to_u8(color.green), to_u8(color.blue))
}

/// Build a krilla Fill from a dessin Fill.
fn to_krilla_fill(fill: &Fill) -> Option<KrillaFill> {
	match fill {
		Fill::Solid { color } => {
			let paint: Paint = to_krilla_color(*color).into();
			Some(KrillaFill {
				paint,
				opacity: NormalizedF32::ONE,
				rule: FillRule::NonZero,
			})
		}
	}
}

/// Build a krilla Stroke from a dessin Stroke.
fn to_krilla_stroke(stroke: &Stroke) -> Option<KrillaStroke> {
	match stroke {
		Stroke::Solid { color, width } => {
			let paint: Paint = to_krilla_color(*color).into();
			Some(KrillaStroke {
				paint,
				width: *width,
				miter_limit: 10.0,
				line_cap: LineCap::Butt,
				line_join: LineJoin::Miter,
				opacity: NormalizedF32::ONE,
				dash: None,
			})
		}
		Stroke::Dashed {
			color,
			width,
			on,
			off,
		} => {
			let paint: Paint = to_krilla_color(*color).into();
			Some(KrillaStroke {
				paint,
				width: *width,
				miter_limit: 10.0,
				line_cap: LineCap::Butt,
				line_join: LineJoin::Miter,
				opacity: NormalizedF32::ONE,
				dash: Some(StrokeDash {
					array: vec![*on, *off],
					offset: 0.0,
				}),
			})
		}
	}
}

struct SurfaceExporter<'a> {
	surface: krilla::surface::Surface<'a>,
	used_font: &'a mut HashMap<(FontRef, font::fontdb::Weight, font::fontdb::Style), Font>,
}
impl Exporter for SurfaceExporter<'_> {
	type Error = PdfError;

	const CAN_EXPORT_ELLIPSE: bool = false;

	fn start_style(
		&mut self,
		StylePosition { fill, stroke }: StylePosition,
	) -> Result<(), Self::Error> {
		self.surface.push_isolated();

		if let Some(ref f) = fill {
			if let Some(krilla_fill) = to_krilla_fill(f) {
				self.surface.set_fill(Some(krilla_fill));
			} else {
				self.surface.set_fill(None);
			}
		} else {
			self.surface.set_fill(None);
		}

		if let Some(ref s) = stroke {
			if let Some(krilla_stroke) = to_krilla_stroke(s) {
				self.surface.set_stroke(Some(krilla_stroke));
			} else {
				self.surface.set_stroke(None);
			}
		} else {
			self.surface.set_stroke(None);
		}

		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		self.surface.pop();

		Ok(())
	}

	fn export_image(
		&mut self,
		ImagePosition {
			bounding_box,
			rotation,
			image,
		}: ImagePosition,
	) -> Result<(), Self::Error> {
		let img = Image::from_rgba8(image.to_rgba8().into_raw(), image.width(), image.height());

		let angle_deg = -rotation.to_degrees();
		let rad = angle_deg.to_radians();
		let c = rad.cos();
		let s = rad.sin();

		let transform = Transform::from_row(c, -s, s, c, bounding_box.left(), bounding_box.top());

		self.surface.push_transform(&transform);

		self.surface.draw_image(
			img,
			Size::from_wh(bounding_box.width(), bounding_box.height()).expect("Invalid image size"),
		);

		self.surface.pop();

		Ok(())
	}

	fn export_curve(
		&mut self,
		curve: CurvePosition,
		StylePosition { .. }: StylePosition,
	) -> Result<(), Self::Error> {
		let mut path_builder = PathBuilder::new();

		let mut prev_point: Option<Point> = None;

		for keypoint in curve.keypoints {
			match keypoint {
				KeypointPosition::Point(p) => {
					let pt = Point::from_xy(p.x, p.y);
					if prev_point.is_none() {
						path_builder.move_to(pt.x, pt.y);
					} else {
						path_builder.line_to(pt.x, pt.y);
					}
					prev_point = Some(pt);
				}
				KeypointPosition::Bezier(Bezier {
					start,
					start_control,
					end_control,
					end,
				}) => {
					let start_opt = start.map(|v| Point::from_xy(v.x, v.y));
					let start_control = Point::from_xy(start_control.x, start_control.y);
					let end_control = Point::from_xy(end_control.x, end_control.y);
					let end = Point::from_xy(end.x, end.y);

					if let Some(s) = start_opt {
						if prev_point.is_none() {
							path_builder.move_to(s.x, s.y);
						} else {
							path_builder.line_to(s.x, s.y);
						}
					}

					path_builder.cubic_to(
						start_control.x,
						start_control.y,
						end_control.x,
						end_control.y,
						end.x,
						end.y,
					);

					prev_point = Some(end);
				}
			}
		}

		if curve.closed {
			path_builder.close();
		}

		let path: Path = path_builder.finish().unwrap();
		self.surface.draw_path(&path);

		Ok(())
	}

	fn export_text(
		&mut self,
		TextPosition {
			text,
			font_size,
			reference_start,
			direction,
			font,
			weight,
			style,
			align,
			on_curve,
		}: TextPosition,
		StylePosition { stroke, fill }: StylePosition,
	) -> Result<(), Self::Error> {
		// Load or reuse font, keyed by (font, weight, style) so that
		// different variations of the same variable font are cached separately.
		let cache_key = (font.clone(), weight, style);

		let krilla_font = if let Some(existing) = self.used_font.get(&cache_key) {
			existing.clone()
		} else {
			let (source, _) = font::font_holder(|v| v.0.db().face_source(font.id))
				.ok_or_else(|| PdfError::UnknownFont(font.family.to_string()))?;

			let bytes = match &source {
				font::fontdb::Source::Binary(bytes)
				| font::fontdb::Source::SharedFile(_, bytes) => (**bytes).as_ref().to_vec(),
				font::fontdb::Source::File(path) => {
					fs::read(path).map_err(PdfError::CantLoadFont)?
				}
			};

			// Set up variation axes for variable fonts.
			let italic_value = if style == font::fontdb::Style::Italic {
				1.0
			} else {
				0.0
			};
			let var_coords = [
				(Tag::new(b"wght"), weight.0 as f32),
				(Tag::new(b"ital"), italic_value),
			];
			let k_font = Font::new_variable(bytes.into(), 0, &var_coords)
				.ok_or_else(|| PdfError::CantParseFont(font.family.to_string()))?;
			self.used_font.insert(cache_key, k_font.clone());
			k_font
		};

		let rotation = direction.y.atan2(direction.x).to_degrees();

		// Convert mm → pt and compute screen Y for krilla's top-left origin.
		let target_x = reference_start.x;
		let target_y = reference_start.y;

		// Build combined transform: rotate + translate.
		// krilla's draw_text handles its own Y-flip for glyph rendering.
		let angle_rad = rotation.to_radians();
		let c = angle_rad.cos();
		let s = angle_rad.sin();
		let transform = Transform::from_row(c, -s, s, c, target_x, target_y);

		self.surface.push_transform(&transform);

		self.surface.draw_text(
			Point::from_xy(0.0, 0.0),
			krilla_font,
			font_size,
			&text,
			false,
			TextDirection::Auto,
		);

		self.surface.pop();

		Ok(())
	}
}

pub struct PdfExporter {
	pdf_document: Document,
	used_font: HashMap<(FontRef, font::fontdb::Weight, font::fontdb::Style), Font>,
	pub viewport: ViewPort,
}
impl Default for PdfExporter {
	fn default() -> Self {
		Self {
			pdf_document: Default::default(),
			used_font: Default::default(),
			viewport: Default::default(),
		}
	}
}
impl PdfExporter {
	pub fn viewport(&mut self, viewport: ViewPort) -> &mut Self {
		self.viewport = viewport;
		self
	}

	pub fn export(&mut self, shape: &Shape) -> Result<Document, PdfError> {
		Ok(self.write_page(shape)?.finish())
	}

	pub fn write_page(&mut self, shape: &Shape) -> Result<&mut Self, PdfError> {
		// A4 in krilla's doc:
		// - width: 21 -> 595.0
		// - height: 29.7 -> 842.0

		const FACTOR: f32 = 595.0 / 21.;

		let bb = self.viewport.bounding_box(shape);

		eprintln!("BB: {:?}", shape.local_bounding_box());

		let mut page = self.pdf_document.start_page_with(
			PageSettings::from_wh(bb.width() * FACTOR, bb.height() * FACTOR).unwrap(),
		);

		let parent_transform =
			nalgebra::convert::<_, Transform2<f32>>(Scale2::new(FACTOR, -FACTOR))
				* nalgebra::convert::<_, Transform2<f32>>(Translation2::new(-bb.left(), -bb.top()));

		shape.write_into_exporter(
			&mut SurfaceExporter {
				surface: page.surface(),
				used_font: &mut self.used_font,
			},
			&parent_transform,
			Default::default(),
		)?;

		page.finish();

		Ok(self)
	}

	pub fn finish(&mut self) -> Document {
		self.used_font = Default::default();
		std::mem::take(&mut self.pdf_document)
	}
}
