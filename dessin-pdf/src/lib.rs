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

pub struct PDFExporter<'a> {
	/// Stored as a raw pointer to avoid borrow-checker conflicts with
	/// `Surface::Drop`. Safe because the surface always outlives the exporter.
	surface: *mut krilla::surface::Surface<'a>,
	used_font: PDFFontHolder,
	/// Stack depth tracking for push/pop balance.
	push_depth: usize,
}

impl<'a> PDFExporter<'a> {
	pub fn new_with_font(
		surface: &'a mut krilla::surface::Surface<'a>,
		used_font: PDFFontHolder,
	) -> Self {
		PDFExporter {
			surface: surface as *mut _,
			used_font,
			push_depth: 0,
		}
	}

	pub fn new(surface: &'a mut krilla::surface::Surface<'a>) -> Self {
		PDFExporter::new_with_font(surface, HashMap::default())
	}

	/// Get a mutable reference to the underlying surface.
	/// # Safety
	/// The surface is guaranteed to be valid for the lifetime `'a` because
	/// it's created before the exporter and dropped after.
	fn surface(&mut self) -> &mut krilla::surface::Surface<'a> {
		unsafe { &mut *self.surface }
	}
}

impl Exporter for PDFExporter<'_> {
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

		// Flip Y for krilla's top-left origin.
		let target_y = -bottom_left.y;

		// Build combined transform: rotate first, then translate.
		// Matrix = [cos(-θ), -sin(-θ), sin(-θ), cos(-θ), tx, ty]
		let angle_deg = -rotation.to_degrees();
		let rad = angle_deg.to_radians();
		let c = rad.cos();
		let s = rad.sin();
		let transform = Transform::from_row(c, -s, s, c, bottom_left.x, target_y);

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
					let pt = Point::from_xy(p.x, -p.y); // Flip Y.
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
					let start_opt = start.map(|v| Point::from_xy(v.x, -v.y));
					let start_control = Point::from_xy(start_control.x, -start_control.y);
					let end_control = Point::from_xy(end_control.x, -end_control.y);
					let end = Point::from_xy(end.x, -end.y);

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

		// krilla uses top-left origin; flip Y.
		let target_x = reference_start.x;
		let target_y = -reference_start.y;

		// Build combined transform: rotate first, then translate.
		// Matrix = [cos(-θ), -sin(-θ), sin(-θ), cos(-θ), tx, ty]
		let angle_rad = -rotation.to_radians();
		let c = angle_rad.cos();
		let s = angle_rad.sin();
		let transform = Transform::from_row(c, -s, s, c, target_x, target_y);

		self.surface().push_transform(&transform);
		self.push_depth += 1;

		// krilla's draw_text uses font size in PDF units (points).
		// The dessin font_size is in mm, so convert: 1 mm ≈ 2.835 points.
		let font_size_pt = font_size * 2.835;

		self.surface().draw_text(
			Point::from_xy(0.0, 0.0),
			krilla_font,
			font_size_pt,
			&text,
			false,
			TextDirection::Auto,
		);

		self.surface().pop();
		self.push_depth -= 1;

		Ok(())
	}
}

/// Convert mm to PDF points (1 mm ≈ 2.835 points).
fn mm_to_pt(mm: f32) -> f32 {
	mm * 2.835
}

/// Export a shape onto a krilla surface. The Y-axis flip transform must
/// already be pushed before calling this.
///
/// Takes a raw pointer to avoid borrow-checker conflicts with `Surface::Drop`.
/// # Safety
/// The surface must be valid for the duration of this call.
fn export_shape_to_surface<'a>(
	surface_ptr: *mut krilla::surface::Surface<'a>,
	shape: &Shape,
	parent_transform: &Transform2<f32>,
	used_font: PDFFontHolder,
) -> Result<(), PDFError> {
	let surface = unsafe { &mut *surface_ptr };
	let mut exporter = PDFExporter::new_with_font(surface, used_font);

	if let Shape::Style { fill, stroke, .. } = shape {
		shape.write_into_exporter(
			&mut exporter,
			parent_transform,
			StylePosition {
				fill: fill.clone(),
				stroke: stroke.clone(),
			},
		)?
	} else {
		shape.write_into_exporter(
			&mut exporter,
			parent_transform,
			StylePosition {
				fill: None,
				stroke: None,
			},
		)?
	}

	Ok(())
}

/// Build a krilla Document and render the shape onto a page.
pub fn write_to_pdf_with_options(shape: &Shape, options: PDFOptions) -> Result<Vec<u8>, PDFError> {
	let (width_mm, height_mm) = options.size.unwrap_or_else(|| {
		let bb = shape.local_bounding_box();
		(bb.width(), bb.height())
	});

	// Convert to PDF points.
	let width_pt = mm_to_pt(width_mm);
	let height_pt = mm_to_pt(height_mm);

	let mut document = Document::new();
	let mut page = document
		.start_page_with(PageSettings::from_wh(width_pt, height_pt).expect("Invalid page size"));

	let mut surface = page.surface();

	// Flip the Y-axis so that the coordinate system matches the dessin convention
	// (origin at bottom-left). We do this by translating up by height_pt and
	// scaling Y by -1.
	let flip_y = Transform::from_row(1.0, 0.0, 0.0, -1.0, 0.0, height_pt);
	surface.push_transform(&flip_y);

	// Center the shape on the page.
	let translation = Translation2::new(width_mm / 2.0, height_mm / 2.0);
	let parent_transform = nalgebra::convert(translation);

	// Export the shape into the surface. We pass a raw pointer to avoid
	// borrow-checker conflicts with Surface::Drop.
	export_shape_to_surface(
		&mut surface as *mut _,
		shape,
		&parent_transform,
		options.used_font,
	)?;

	// Pop the flip_y transform that we pushed at the beginning.
	surface.pop();
	surface.finish();
	page.finish();

	document
		.finish()
		.map_err(|e| PDFError::KrillaSerializeError(e.to_string()))
}

/// Render a shape to a PDF, returning the raw bytes.
pub fn to_pdf_bytes(shape: &Shape) -> Result<Vec<u8>, PDFError> {
	write_to_pdf_with_options(shape, PDFOptions::default())
}

/// Render a shape to a PDF with custom options, returning the raw bytes.
pub fn to_pdf_with_options(shape: &Shape, options: PDFOptions) -> Result<Vec<u8>, PDFError> {
	write_to_pdf_with_options(shape, options)
}

/// Render a shape to a PDF, returning the raw bytes. Alias for `to_pdf_bytes`.
pub fn to_pdf(shape: &Shape) -> Result<Vec<u8>, PDFError> {
	to_pdf_bytes(shape)
}

/// Write a shape into an existing krilla document.
///
/// This creates a new page on the document and renders the shape onto it.
/// Returns the raw PDF bytes after finishing the document.
///
/// **Deprecated:** Use [`write_to_pdf_with_options`] instead. This function
/// kept for API compatibility but krilla doesn't support incrementally
/// building a document like printpdf did. Instead, each call produces
/// a complete PDF.
#[deprecated(
	since = "0.9.0",
	note = "krilla doesn't support incremental document building. Use `write_to_pdf_with_options` instead."
)]
pub fn write_to_pdf(shape: &Shape, _doc: &mut Document) -> Result<Vec<u8>, PDFError> {
	write_to_pdf_with_options(shape, PDFOptions::default())
}
