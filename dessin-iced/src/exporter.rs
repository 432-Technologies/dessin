use dessin::{export::Exporter, prelude::*};
use iced_core::{Point, Rectangle, Size};
use iced_widget::renderer::geometry::{self, Frame};
use std::{convert::Infallible, sync::RwLock};

pub struct IcedExporter<'a, Renderer: geometry::Renderer> {
	pub frame: &'a mut Frame<Renderer>,
}
impl<'a, Renderer: geometry::Renderer> Exporter for IcedExporter<'a, Renderer> {
	type Error = Infallible;

	const CAN_EXPORT_ELLIPSE: bool = false;

	fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn export_image(&mut self, image: ImagePosition) -> Result<(), Self::Error> {
		let mut png = std::io::Cursor::new(Vec::new());
		image
			.image
			.write_to(&mut png, dessin::image::ImageFormat::Png)
			.unwrap();

		let img_bytes =
			iced_widget::canvas::Image::new(iced_core::image::Handle::from_bytes(png.into_inner()))
				.filter_method(iced_core::image::FilterMethod::Linear)
				.rotation(image.rotation);

		self.frame.draw_image(
			Rectangle::new(
				Point::new(image.top_left.x, image.top_left.y),
				Size {
					width: image.width,
					height: image.height,
				},
			),
			img_bytes,
		);

		Ok(())
	}

	fn export_curve(
		&mut self,
		curve: CurvePosition,
		style_position: StylePosition,
	) -> Result<(), Self::Error> {
		let path = iced_widget::canvas::Path::new(|builder| {
			for kp in curve.keypoints {
				match kp {
					KeypointPosition::Point(p) => {
						builder.line_to(iced_core::Point { x: p.x, y: -p.y });
					}
					KeypointPosition::Bezier(bezier) => {
						if let Some(start) = bezier.start {
							builder.line_to(iced_core::Point {
								x: start.x,
								y: -start.y,
							});
						}

						builder.bezier_curve_to(
							iced_core::Point {
								x: bezier.start_control.x,
								y: -bezier.start_control.y,
							},
							iced_core::Point {
								x: bezier.end_control.x,
								y: -bezier.end_control.y,
							},
							iced_core::Point {
								x: bezier.end.x,
								y: -bezier.end.y,
							},
						);
					}
				}
			}

			if curve.closed {
				builder.close();
			}
		});

		if let Some(fill) = style_position.fill {
			let fill = match fill {
				Fill::Solid { color } => iced_widget::canvas::Fill {
					style: iced_widget::canvas::Style::Solid(iced_core::Color {
						r: color.red,
						g: color.green,
						b: color.blue,
						a: color.alpha,
					}),
					rule: geometry::fill::Rule::EvenOdd,
				},
			};

			self.frame.fill(&path, fill);
		}

		if let Some(stroke) = style_position.stroke {
			let stroke = match stroke {
				Stroke::Solid { color, width } => iced_widget::canvas::Stroke {
					style: iced_widget::canvas::Style::Solid(iced_core::Color {
						r: color.red,
						g: color.green,
						b: color.blue,
						a: color.alpha,
					}),
					width,
					..Default::default()
				},
				Stroke::Dashed {
					color,
					width,
					on,
					off,
				} => iced_widget::canvas::Stroke {
					style: iced_widget::canvas::Style::Solid(iced_core::Color {
						r: color.red,
						g: color.green,
						b: color.blue,
						a: color.alpha,
					}),
					width,
					line_dash: iced_widget::canvas::LineDash {
						segments: &[on, off],
						offset: 0,
					},
					..Default::default()
				},
			};

			self.frame.stroke(&path, stroke);
		}

		Ok(())
	}

	fn export_text(&mut self, text: TextPosition, style: StylePosition) -> Result<(), Self::Error> {
		let color = match style.fill {
			Some(Fill::Solid { color }) => iced_core::Color {
				r: color.red,
				g: color.green,
				b: color.blue,
				a: color.alpha,
			},
			_ => iced_core::Color {
				r: 0.,
				g: 0.,
				b: 0.,
				a: 0.,
			},
		};

		static FONT_REFS: RwLock<Vec<std::sync::Arc<&'static str>>> = RwLock::new(Vec::new());

		let font = text
			.font
			.as_ref()
			.map(|font| {
				let mut refs = FONT_REFS.write().unwrap();

				if let Some(r) = refs.iter().find(|&v| **v == &**font).cloned() {
					iced_core::Font::with_name(*r)
				} else {
					let f = &*font.to_string().leak();

					refs.push(std::sync::Arc::new(f));

					iced_core::Font::with_name(f)
				}
			})
			.unwrap_or(iced_core::Font::DEFAULT);

		self.frame.fill_text(iced_widget::canvas::Text {
			content: text.text.to_owned(),
			position: Point {
				x: text.reference_start.x,
				y: -text.reference_start.y,
			},
			max_width: f32::MAX,
			color,
			size: iced_core::Pixels(text.font_size),
			line_height: iced_core::text::LineHeight::Relative(1.),
			font,
			align_x: match text.align {
				TextAlign::Left => iced_core::text::Alignment::Left,
				TextAlign::Center => iced_core::text::Alignment::Center,
				TextAlign::Right => iced_core::text::Alignment::Right,
			},
			align_y: iced_core::alignment::Vertical::Center,
			shaping: iced_core::text::Shaping::Basic,
		});

		Ok(())
	}
}
