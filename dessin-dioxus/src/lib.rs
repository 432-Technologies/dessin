use ::image::ImageFormat;
use dessin::prelude::*;
use dioxus::prelude::*;
use font::FontRef;
use nalgebra::{Scale2, Transform2};
use std::{
	collections::HashSet,
	fmt::{self},
	io::Cursor,
};

#[derive(Debug)]
pub enum SVGError {
	WriteError(fmt::Error),
	CurveHasNoStartingPoint(CurvePosition),
	RenderError(RenderError),
}
impl fmt::Display for SVGError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{self:?}")
	}
}
impl From<fmt::Error> for SVGError {
	fn from(value: fmt::Error) -> Self {
		SVGError::WriteError(value)
	}
}
impl From<RenderError> for SVGError {
	fn from(value: RenderError) -> Self {
		SVGError::RenderError(value)
	}
}
impl std::error::Error for SVGError {}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ViewPort {
	/// Create a viewport centered around (0, 0), with size (width, height)
	ManualCentered { width: f32, height: f32 },
	/// Create a viewport centered around (x, y), with size (width, height)
	ManualViewport {
		x: f32,
		y: f32,
		width: f32,
		height: f32,
	},
	/// Create a Viewport centered around (0, 0), with auto size that include all [Shapes][`dessin::prelude::Shape`]
	AutoCentered,
	#[default]
	/// Create a Viewport centered around the centered of the shapes, with auto size that include all [Shapes][`dessin::prelude::Shape`]
	AutoBoundingBox,
}

#[derive(Default, Clone, PartialEq)]
pub struct SVGOptions {
	pub viewport: ViewPort,
}

#[component]
pub fn SVG(shape: ReadOnlySignal<Shape>, options: Option<SVGOptions>) -> Element {
	let options = options.unwrap_or_default();

	let view_box = use_memo(move || {
		let shape = shape();

		let (min_x, min_y, span_x, span_y) = match options.viewport {
			ViewPort::ManualCentered { width, height } => {
				(-width / 2., -height / 2., width, height)
			}
			ViewPort::ManualViewport {
				x,
				y,
				width,
				height,
			} => (x - width / 2., y - height / 2., width, height),
			ViewPort::AutoCentered => {
				let bb = shape.local_bounding_box().straigthen();

				let mirror_bb = bb
					.transform(&nalgebra::convert::<_, Transform2<f32>>(Scale2::new(
						-1., -1.,
					)))
					.into_straight();

				let overall_bb = bb.join(mirror_bb);

				(
					-overall_bb.width() / 2.,
					-overall_bb.height() / 2.,
					overall_bb.width(),
					overall_bb.height(),
				)
			}
			ViewPort::AutoBoundingBox => {
				let bb = shape.local_bounding_box().straigthen();

				(bb.top_left().x, -bb.top_left().y, bb.width(), bb.height())
			}
		};

		format!("{min_x} {min_y} {span_x} {span_y}")
	});

	let used_font = use_signal(|| HashSet::new());

	rsx! {
		svg {
			view_box,
			Shaper {
				shape,
				parent_transform: nalgebra::convert(Scale2::new(1., -1.)),
				used_font,
			}
		}
	}
}

#[component]
fn Shaper(
	shape: ReadOnlySignal<Shape>,
	parent_transform: Transform2<f32>,
	used_font: Signal<HashSet<(FontRef, FontWeight)>>,
) -> Element {
	match shape() {
		Shape::Group(dessin::shapes::Group {
			local_transform,
			shapes,
			metadata,
		}) => {
			let parent_transform = parent_transform * local_transform;

			// let attributes = metadata
			// 	.into_iter()
			// 	.map(|(name, value)| Attribute {
			// 		name,
			// 		value: dioxus_core::AttributeValue::Text(value),
			// 		namespace: todo!(),
			// 		volatile: todo!(),
			// 	})
			// 	.collect::<Vec<_>>();

			rsx! {
				g {
					for shape in shapes {
						Shaper {
							shape: shape,
							parent_transform,
							used_font,
						}
					}
				}
			}
		}
		Shape::Style {
			fill,
			stroke,
			shape,
		} => {
			let fill = fill
				.map(|color| {
					format!(
						"rgb({} {} {} / {:.3})",
						(color.red * 255.) as u32,
						(color.green * 255.) as u32,
						(color.blue * 255.) as u32,
						color.alpha
					)
				})
				.unwrap_or_else(|| "none".to_string());

			let (stroke, stroke_width, stroke_dasharray) = match stroke {
				Some(Stroke::Dashed {
					color,
					width,
					on,
					off,
				}) => (
					Some(format!(
						"rgb({} {} {} / {:.3})",
						(color.red * 255.) as u32,
						(color.green * 255.) as u32,
						(color.blue * 255.) as u32,
						color.alpha
					)),
					Some(width),
					Some(format!("{on},{off}")),
				),
				Some(Stroke::Full { color, width }) => (
					Some(format!(
						"rgb({} {} {} / {:.3})",
						(color.red * 255.) as u32,
						(color.green * 255.) as u32,
						(color.blue * 255.) as u32,
						color.alpha
					)),
					Some(width),
					None,
				),
				None => (None, None, None),
			};

			rsx! {
				g {
					fill,
					stroke,
					stroke_width,
					stroke_dasharray,

					Shaper {
						parent_transform,
						shape: *shape,
						used_font,
					}
				}
			}
		}
		Shape::Ellipse(ellipse) => {
			let ellipse = ellipse.position(&parent_transform);
			let x = ellipse.center.x;
			let y = ellipse.center.y;
			let r = -ellipse.rotation.to_degrees();

			rsx! {
				ellipse {
					rx: ellipse.semi_major_axis,
					ry: ellipse.semi_minor_axis,
					transform: "translate({x} {y}) rotate({r})"
				}
			}
		}
		Shape::Image(image) => {
			let image = image.position(&parent_transform);

			let mut raw_image = Cursor::new(vec![]);
			image
				.image
				.write_to(&mut raw_image, ImageFormat::Png)
				.unwrap();

			let data = data_encoding::BASE64.encode(&raw_image.into_inner());

			let r = (-image.rotation.to_degrees() + 360.) % 360.;

			rsx! {
				image {
					width: image.width,
					height: image.height,
					x: image.center.x - image.width / 2.,
					y: image.center.y - image.width / 2.,
					transform: "rotate({r})",
					href: "data:image/png;base64,{data}"
				}
			}
		}
		Shape::Text(text) => {
			let id = rand::random::<u64>().to_string();

			let text = text.position(&parent_transform);

			let weight = match text.font_weight {
				FontWeight::Bold | FontWeight::BoldItalic => "bold",
				_ => "normal",
			};
			let text_style = match text.font_weight {
				FontWeight::Italic | FontWeight::BoldItalic => "italic",
				_ => "normal",
			};
			let align = match text.align {
				TextAlign::Center => "middle",
				TextAlign::Left => "start",
				TextAlign::Right => "end",
			};

			let font = text.font.clone().unwrap_or(FontRef::default());
			used_font.write().insert((font.clone(), text.font_weight));
			let font = font.name(text.font_weight);

			let x = text.reference_start.x;
			let y = text.reference_start.y;
			let r = text.direction.y.atan2(text.direction.x).to_degrees();

			rsx! {
				text {
					font_family:"{font}",
					text_anchor:"{align}",
					font_size:"{text.font_size}px",
					font_weight:"{weight}",
					"text-style":"{text_style}",
					transform:"translate({x} {y}) rotate({r})",
					if let Some(curve) = text.on_curve {
						path {
							id: id.clone(),
							d: write_curve(curve)
						}
						textPath {
							href: id,
							{text.text}
						}
					} else {
						{text.text}
					}
				}
			}
		}
		Shape::Curve(curve) => rsx! {
			path {
				d: write_curve(curve.position(&parent_transform))
			}
		},
		Shape::Dynamic {
			local_transform,
			shaper,
		} => rsx! {
			Shaper {
				parent_transform: parent_transform * local_transform,
				shape: shaper(),
				used_font,
			}
		},
	}
}

fn write_curve(curve: CurvePosition) -> String {
	let mut acc = String::new();
	let mut has_start = false;

	for keypoint in &curve.keypoints {
		match keypoint {
			KeypointPosition::Point(p) => {
				if has_start {
					acc.push_str("L ");
				} else {
					acc.push_str("M ");
					has_start = true;
				}
				acc.push_str(&format!("{} {} ", p.x, p.y));
			}
			KeypointPosition::Bezier(b) => {
				if has_start {
					if let Some(v) = b.start {
						acc.push_str(&format!("L {} {} ", v.x, v.y));
					}
				} else {
					if let Some(v) = b.start {
						acc.push_str(&format!("M {} {} ", v.x, v.y));
						has_start = true;
					} else {
						return String::new();
					}
				}

				acc.push_str(&format!(
					"C {start_ctrl_x} {start_ctrl_y} {end_ctrl_x} {end_ctrl_y} {end_x} {end_y} ",
					start_ctrl_x = b.start_control.x,
					start_ctrl_y = b.start_control.y,
					end_ctrl_x = b.end_control.x,
					end_ctrl_y = b.end_control.y,
					end_x = b.end.x,
					end_y = b.end.y,
				));
			}
		}

		has_start = true;
	}

	if curve.closed {
		acc.push_str(&format!("Z"));
	}

	acc
}
