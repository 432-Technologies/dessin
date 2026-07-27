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
	text::{Font, TextDirection},
	Document,
};
use nalgebra::{Transform2, Translation2};
use std::{collections::HashMap, fmt, fs};

#[derive(Debug, thiserror::Error)]
pub enum PDFError {
	#[error("Krilla Image error: {0}")]
	KrillaImageError(String),
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

/// Mapping from FontRef to krilla Font.
type PDFFontHolder = HashMap<FontRef, Font>;

#[derive(Default)]
pub struct PDFOptions {
	pub size: Option<(f32, f32)>,
	/// Pre-loaded fonts to reuse across pages.
	pub used_font: PDFFontHolder,
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

/// Detect image format from bytes and create a krilla Image.
fn image_from_bytes(data: &[u8]) -> Result<Image, PDFError> {
	// Try PNG
	if let Ok(img) = Image::from_png(data.to_vec().into(), true) {
		return Ok(img);
	}
	// Try JPEG
	if let Ok(img) = Image::from_jpeg(data.to_vec().into(), true) {
		return Ok(img);
	}
	// Try WebP
	if let Ok(img) = Image::from_webp(data.to_vec().into(), true) {
		return Ok(img);
	}
	// Try GIF
	if let Ok(img) = Image::from_gif(data.to_vec().into(), true) {
		return Ok(img);
	}
	Err(PDFError::KrillaImageError(
		"Could not decode image (tried PNG, JPEG, WebP, GIF)".to_string(),
	))
}

/// Collects shape data for later replay onto a krilla surface.
/// Avoids storing `&mut Surface` so the surface reference can be released
/// before calling `surface.finish()`.
pub struct PDFExporter {
	shape: Shape,
	parent_transform: Transform2<f32>,
	style: StylePosition,
	used_font: PDFFontHolder,
}

/// Runtime exporter that draws onto a krilla surface.
/// The surface is provided as `&mut` and borrowed only during export.
struct SurfaceExporter<'a, 's> {
	surface: &'s mut krilla::surface::Surface<'a>,
	used_font: PDFFontHolder,
	push_depth: usize,
	page_height_pt: f32,
}

impl PDFExporter {
	/// Collect shape data for later replay onto a krilla surface.
	pub fn new_with_font(
		shape: Shape,
		parent_transform: Transform2<f32>,
		style: StylePosition,
		used_font: PDFFontHolder,
	) -> Self {
		PDFExporter {
			shape,
			parent_transform,
			style,
			used_font,
		}
	}

	/// Replay the collected shape onto a krilla surface.
	pub fn apply(
		self,
		surface: &mut krilla::surface::Surface<'_>,
		page_height_pt: f32,
	) -> Result<(), PDFError> {
		let mut exporter = SurfaceExporter {
			surface,
			used_font: self.used_font,
			push_depth: 0,
			page_height_pt,
		};

		self.shape
			.write_into_exporter(&mut exporter, &self.parent_transform, self.style)?;

		Ok(())
	}
}

impl<'a, 's> SurfaceExporter<'a, 's> {
	/// Get a mutable reference to the underlying surface.
	fn surface(&mut self) -> &mut krilla::surface::Surface<'a> {
		&mut self.surface
	}
}

impl Exporter for SurfaceExporter<'_, '_> {
	type Error = PDFError;

	const CAN_EXPORT_ELLIPSE: bool = false;

	fn start_style(
		&mut self,
		StylePosition { fill, stroke }: StylePosition,
	) -> Result<(), Self::Error> {
		// Push an isolated layer for this style group.
		self.surface().push_isolated();
		self.push_depth += 1;

		// Set fill.
		if let Some(ref f) = fill {
			if let Some(krilla_fill) = to_krilla_fill(f) {
				self.surface().set_fill(Some(krilla_fill));
			}
		}

		// Set stroke.
		if let Some(ref s) = stroke {
			if let Some(krilla_stroke) = to_krilla_stroke(s) {
				self.surface().set_stroke(Some(krilla_stroke));
			}
		}

		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		if self.push_depth > 0 {
			self.surface().pop();
			self.push_depth -= 1;
		}
		Ok(())
	}

	fn export_image(
		&mut self,
		ImagePosition {
			bottom_left,
			width,
			height,
			rotation,
			image,
			..
		}: ImagePosition,
	) -> Result<(), Self::Error> {
		let img = image_from_bytes(image.as_bytes())?;

		// Calculate image dimensions in PDF units (points).
		let (img_width_px, img_height_px) = img.size();
		let dpi = 300.0;
		let raw_width = img_width_px as f32 * 25.4 / dpi;
		let raw_height = img_height_px as f32 * 25.4 / dpi;

		let scale_width = width / raw_width;
		let scale_height = height / raw_height;

		let draw_width = raw_width * scale_width;
		let draw_height = raw_height * scale_height;

		// Build combined transform: rotate first, then translate.
		// Negate rotation because flipping Y axis reverses rotation direction.
		let angle_deg = -rotation.to_degrees();
		let rad = angle_deg.to_radians();
		let c = rad.cos();
		let s = rad.sin();
		let transform = Transform::from_row(
			c,
			-s,
			s,
			c,
			bottom_left.x,
			self.page_height_pt - bottom_left.y,
		);

		self.surface().push_transform(&transform);
		self.push_depth += 1;

		// Draw the image centered at the origin.
		let _ = self.surface().draw_image(
			img,
			Size::from_wh(draw_width, draw_height).expect("Invalid image size"),
		);

		self.surface().pop();
		self.push_depth -= 1;

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
					let pt = Point::from_xy(p.x, self.page_height_pt - p.y);
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
					let start_opt = start.map(|v| Point::from_xy(v.x, self.page_height_pt - v.y));
					let start_control =
						Point::from_xy(start_control.x, self.page_height_pt - start_control.y);
					let end_control =
						Point::from_xy(end_control.x, self.page_height_pt - end_control.y);
					let end = Point::from_xy(end.x, self.page_height_pt - end.y);

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
		self.surface().draw_path(&path);

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
			..
		}: TextPosition,
		StylePosition { .. }: StylePosition,
	) -> Result<(), Self::Error> {
		// Load or reuse font.
		let krilla_font = if let Some(existing) = self.used_font.get(&font) {
			existing.clone()
		} else {
			let (source, _) = font::font_holder(|v| v.0.db().face_source(font.id))
				.ok_or_else(|| PDFError::UnknownFont(font.family.to_string()))?;

			let bytes = match &source {
				font::fontdb::Source::Binary(bytes)
				| font::fontdb::Source::SharedFile(_, bytes) => (**bytes).as_ref().to_vec(),
				font::fontdb::Source::File(path) => {
					fs::read(path).map_err(PDFError::CantLoadFont)?
				}
			};

			let k_font = Font::new(bytes.into(), 0)
				.ok_or_else(|| PDFError::CantParseFont(font.family.to_string()))?;
			self.used_font.insert(font.clone(), k_font.clone());
			k_font
		};

		let rotation = direction.y.atan2(direction.x).to_degrees();

		// Convert mm → pt and compute screen Y for krilla's top-left origin.
		let target_x = reference_start.x;
		let target_y = self.page_height_pt - reference_start.y;

		// Build combined transform: rotate + translate.
		// krilla's draw_text handles its own Y-flip for glyph rendering.
		let angle_rad = rotation.to_radians();
		let c = angle_rad.cos();
		let s = angle_rad.sin();
		let transform = Transform::from_row(c, -s, s, c, target_x, target_y);

		self.surface().push_transform(&transform);
		self.push_depth += 1;

		self.surface().draw_text(
			Point::from_xy(0.0, 0.0),
			krilla_font,
			font_size,
			&text,
			false,
			TextDirection::Auto,
		);

		self.surface().pop();
		self.push_depth -= 1;

		Ok(())
	}
}

/// Collect shape data then replay onto the surface.
/// The surface reference is scoped so it's released before `surface.finish()`.
fn export_shape_to_surface<'a>(
	surface: &mut krilla::surface::Surface<'a>,
	shape: &Shape,
	parent_transform: &Transform2<f32>,
	used_font: PDFFontHolder,
	page_height: f32,
) -> Result<(), PDFError> {
	let collector = {
		// Clone shape and collect style from the top-level wrapper.
		let cloned = shape.clone();
		let style = if let Shape::Style { fill, stroke, .. } = &cloned {
			StylePosition {
				fill: fill.clone(),
				stroke: stroke.clone(),
			}
		} else {
			StylePosition {
				fill: None,
				stroke: None,
			}
		};
		PDFExporter::new_with_font(cloned, (*parent_transform).into(), style, used_font)
	};

	// Replay onto the surface. The surface reference is borrowed only within this scope.
	// After this scope ends, the surface reference is released and we can call finish().
	collector.apply(surface, page_height)?;

	Ok(())
}

/// Build a krilla Document and render the shape onto a page.
pub fn write_to_pdf_with_options(shape: &Shape, options: PDFOptions) -> Result<Vec<u8>, PDFError> {
	let (width, height) = options.size.unwrap_or_else(|| {
		let bb = shape.local_bounding_box();
		(bb.width(), bb.height())
	});

	let mut document = Document::new();
	let mut page =
		document.start_page_with(PageSettings::from_wh(width, height).expect("Invalid page size"));

	let mut surface = page.surface();

	// Center the shape on the page.
	let translation = Translation2::new(0., height);
	let parent_transform = nalgebra::convert(translation);

	// Collect shape data then replay onto the surface.
	// The surface reference is scoped inside export_shape_to_surface,
	// so it's released here and we can safely call finish().
	export_shape_to_surface(
		&mut surface,
		shape,
		&parent_transform,
		options.used_font,
		height,
	)?;

	surface.finish();
	page.finish();

	document
		.finish()
		.map_err(|e| PDFError::KrillaSerializeError(e.to_string()))
}

/// Render a shape to a PDF with custom options, returning the raw bytes.
pub fn to_pdf_with_options(shape: &Shape, options: PDFOptions) -> Result<Vec<u8>, PDFError> {
	write_to_pdf_with_options(shape, options)
}

/// Render a shape to a PDF, returning the raw bytes. Alias for `to_pdf_bytes`.
pub fn to_pdf(shape: &Shape) -> Result<Vec<u8>, PDFError> {
	to_pdf_with_options(shape, PDFOptions::default())
}
