use ::image::ImageFormat;
use dessin::{
	export::{Export, Exporter, ViewPort},
	font::fontdb,
	palette::Srgba,
	prelude::*,
};
use nalgebra::Scale2;
use std::{
	collections::HashSet,
	fmt::{self, Write},
	io::Cursor,
	sync::{atomic::AtomicU32, LazyLock},
};

#[derive(Debug)]
pub enum SvgError {
	WriteError(fmt::Error),
	CurveHasNoStartingPoint(CurvePosition),
	NoDefaultFont,
}
impl fmt::Display for SvgError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{self:?}")
	}
}
impl From<fmt::Error> for SvgError {
	fn from(value: fmt::Error) -> Self {
		SvgError::WriteError(value)
	}
}
impl std::error::Error for SvgError {}

impl SvgExporter {
	fn write_style(&mut self, style: StylePosition) -> Result<(), SvgError> {
		match style.fill {
			Some(Fill::Solid { color }) => {
				write!(self.acc, "fill='#{:X}' ", Srgba::<u8>::from_format(color))?;
			}

			None => write!(self.acc, "fill='none' ")?,
		}

		match style.stroke {
			Some(Stroke::Dashed {
				color,
				width,
				on,
				off,
			}) => write!(
				self.acc,
				"stroke='#{:X}' stroke-width='{width}' stroke-dasharray='{on},{off}' ",
				Srgba::<u8>::from_format(color)
			)?,
			Some(Stroke::Solid { color, width }) => write!(
				self.acc,
				"stroke='#{:X}' stroke-width='{width}' ",
				Srgba::<u8>::from_format(color)
			)?,
			None => {}
		}

		Ok(())
	}

	#[allow(unused)]
	fn write_curve(&mut self, curve: CurvePosition) -> Result<(), SvgError> {
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
					} else if let Some(v) = b.start {
						write!(self.acc, "M {} {} ", v.x, v.y)?;
						has_start = true;
					} else {
						return Err(SvgError::CurveHasNoStartingPoint(curve));
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
			}

			has_start = true;
		}

		if curve.closed {
			write!(self.acc, "Z")?;
		}

		Ok(())
	}
}

impl Exporter for SvgExporter {
	type Error = SvgError;

	const CAN_EXPORT_ELLIPSE: bool = true;

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

	fn start_block(&mut self, _metadata: &[(String, String)]) -> Result<(), Self::Error> {
		if !_metadata.is_empty() {
			write!(self.acc, "<g ")?;
			for (key, value) in _metadata {
				write!(self.acc, r"{key}={value} ")?;
			}
			write!(self.acc, ">")?;
		}

		Ok(())
	}

	fn end_block(&mut self, _metadata: &[(String, String)]) -> Result<(), Self::Error> {
		if !_metadata.is_empty() {
			write!(self.acc, "</g>")?;
		}
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
		let mut raw_image = Cursor::new(vec![]);
		image.write_to(&mut raw_image, ImageFormat::Png).unwrap();

		let data = data_encoding::BASE64.encode(&raw_image.into_inner());

		let width = bounding_box.width();
		let height = bounding_box.height();
		let center = bounding_box.center();

		write!(
			self.acc,
			r#"<image width="{width}" height="{height}" x="{x}" y="{y}" "#,
			x = center.x - width / 2.,
			y = center.y - height / 2.,
		)?;

		if rotation.abs() > 10e-6 {
			write!(
				self.acc,
				r#" transform="rotate({rot})" "#,
				rot = (-rotation.to_degrees() + 360.) % 360.
			)?;
		}

		write!(self.acc, r#"href="data:image/png;base64,{data}"/>"#)?;

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
		_: StylePosition,
	) -> Result<(), Self::Error> {
		write!(
			self.acc,
			r#"<ellipse rx="{semi_major_axis}" ry="{semi_minor_axis}" transform=""#,
		)?;

		write!(
			self.acc,
			r"translate({cx} {cy}) ",
			cx = center.x,
			cy = center.y
		)?;

		if rotation.abs() > 10e-6 {
			write!(self.acc, r"rotate({rot}) ", rot = -rotation.to_degrees())?;
		}

		write!(self.acc, r#""/>"#)?;

		Ok(())
	}

	fn export_curve(&mut self, curve: CurvePosition, _: StylePosition) -> Result<(), Self::Error> {
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
			weight: font_weight,
			style: font_style,
			on_curve,
			font_size,
			reference_start,
			bounding_box: _,
			direction,
			font,
		}: TextPosition,
		_: StylePosition,
	) -> Result<(), Self::Error> {
		static ID: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));
		let id = ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel);

		self.used_font.insert(font.id);
		let font_name = &font.family;

		let font_weight = match font_weight {
			FontWeight::LIGHT => "lighter",
			FontWeight::BOLD => "bold",
			FontWeight::BLACK => "bolder",
			_ => "normal",
		};
		let font_style = match font_style {
			FontStyle::Normal => "normal",
			FontStyle::Italic => "italic",
			FontStyle::Oblique => "oblique",
		};
		let align = match align {
			TextAlign::Center => "middle",
			TextAlign::Left => "start",
			TextAlign::Right => "end",
		};

		let text = text.replace('<', "&lt;").replace('>', "&gt;");

		write!(
			self.acc,
			r#"<text font-family="{font_name}" text-anchor="{align}" font-size="{font_size}px" font-weight="{font_weight}" font-style="{font_style}" transform=""#,
		)?;

		write!(
			self.acc,
			r"translate({cx} {cy}) ",
			cx = reference_start.x,
			cy = reference_start.y
		)?;

		let rotation = direction.y.atan2(direction.x);
		if rotation.abs() > 10e-6 {
			write!(self.acc, r"rotate({rot}) ", rot = rotation.to_degrees())?;
		}

		write!(self.acc, r#"">"#)?;

		if let Some(curve) = on_curve {
			write!(self.acc, r#"<path id="{id}" d=""#)?;
			self.write_curve(curve)?;
			write!(self.acc, r#""/>"#)?;

			write!(self.acc, r##"<textPath href="#{id}">{text}</textPath>"##)?;
		} else {
			write!(self.acc, "{text}")?;
		}
		write!(self.acc, r"</text>")?;

		Ok(())
	}
}

#[derive(Debug)]
pub struct SvgExporter {
	acc: String,
	used_font: HashSet<fontdb::ID>,
	pub embed_fonts: bool,
	pub skip_svg_tag: bool,
	pub viewport: ViewPort,
}
impl Default for SvgExporter {
	fn default() -> Self {
		Self {
			acc: Default::default(),
			used_font: Default::default(),

			viewport: Default::default(),
			embed_fonts: true,
			skip_svg_tag: false,
		}
	}
}
impl SvgExporter {
	pub fn embed_fonts(&mut self, embed_fonts: bool) -> &mut Self {
		self.embed_fonts = embed_fonts;
		self
	}

	pub fn skip_svg_tag(&mut self, skip_svg_tag: bool) -> &mut Self {
		self.skip_svg_tag = skip_svg_tag;
		self
	}

	pub fn viewport(&mut self, viewport: ViewPort) -> &mut Self {
		self.viewport = viewport;
		self
	}

	pub fn export(&mut self, shape: &Shape) -> Result<String, SvgError> {
		let bb = self.viewport.bounding_box(shape);
		let parent_transform = nalgebra::convert(Scale2::new(1., -1.));

		shape.write_into_exporter(self, &parent_transform, Default::default())?;

		let acc = std::mem::take(&mut self.acc);
		let used_font = std::mem::take(&mut self.used_font);

		let content = self.embed_fonts.then(|| {
			let fonts = font::font_holder(|holder| {
				let db = holder.0.db();
				db
					.faces()
					.filter(|v| used_font.contains(&v.id))
					.filter_map(|v| match &v.source {
						font::fontdb::Source::Binary(bytes) => Some((
							v.families[0].0.as_str(),
							bytes)),
						_ => None,
					})
					.map(|(font_name, bytes)| {
						let bytes = (**bytes).as_ref();

						let mime = if bytes.starts_with(&[0x4F, 0x54, 0x54, 0x4F]) {"font/otf"} else if bytes.starts_with(&[0x00, 0x01, 0x00, 0x00, 0x00]) {"font/ttf"} else {"font"};

						let encoded_font_bytes = data_encoding::BASE64.encode(bytes.as_ref());
						format!(
							r#"@font-face{{font-family:{font_name};src:url("data:{mime};base64,{encoded_font_bytes}");}}"#
						)
					})

					.collect::<String>()
			});

			format!("<defs><style>{fonts}</style></defs>{acc}")
		}).unwrap_or(acc);

		let svg = if self.skip_svg_tag {
			content
		} else {
			const SCHEME: &str =
				r#"xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#;

			format!(
				r#"<svg viewBox="{min_x} {min_y} {span_x} {span_y}" {SCHEME}>{content}</svg>"#,
				min_x = bb.left(),
				min_y = -bb.top(),
				span_x = bb.width(),
				span_y = bb.height(),
			)
		};

		Ok(svg)
	}
}
