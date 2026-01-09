use std::{convert::Infallible, ops::Deref};

use dessin::{
	export::{Export, Exporter},
	nalgebra::{self, Scale2, Transform2, Translation2},
	prelude::*,
};
use iced_core::Point;
use iced_widget::{
	canvas,
	renderer::geometry::{self, Frame},
};

struct IcedExporter<Renderer: geometry::Renderer> {
	frame: Frame<Renderer>,
}
impl<Renderer: geometry::Renderer> Exporter for IcedExporter<Renderer> {
	type Error = Infallible;

	const CAN_EXPORT_ELLIPSE: bool = false;

	fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn export_image(&mut self, _image: ImagePosition) -> Result<(), Self::Error> {
		Ok(())
	}

	fn export_curve(
		&mut self,
		curve: CurvePosition,
		style_position: StylePosition,
	) -> Result<(), Self::Error> {
		let path = canvas::Path::new(|builder| {
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
		});

		if let Some(fill) = style_position.fill {
			let fill = match fill {
				Fill::Solid { color } => canvas::Fill {
					style: canvas::Style::Solid(iced_core::Color {
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
				Stroke::Solid { color, width } => canvas::Stroke {
					style: canvas::Style::Solid(iced_core::Color {
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
				} => canvas::Stroke {
					style: canvas::Style::Solid(iced_core::Color {
						r: color.red,
						g: color.green,
						b: color.blue,
						a: color.alpha,
					}),
					width,
					line_dash: canvas::LineDash {
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

		self.frame.fill_text(canvas::Text {
			content: text.text.to_owned(),
			position: Point {
				x: text.reference_start.x,
				y: -text.reference_start.y,
			},
			max_width: f32::MAX,
			color,
			size: iced_core::Pixels(text.font_size),
			line_height: iced_core::text::LineHeight::Relative(1.),
			font: iced_core::Font::DEFAULT,
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

pub struct IcedShape(pub Shape);
impl Deref for IcedShape {
	type Target = Shape;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<Message, Theme, Renderer: geometry::Renderer> canvas::Program<Message, Theme, Renderer>
	for IcedShape
{
	type State = ();

	fn draw(
		&self,
		_state: &Self::State,
		renderer: &Renderer,
		_theme: &Theme,
		mut bounds: iced_core::Rectangle,
		_cursor: iced_core::mouse::Cursor,
	) -> Vec<canvas::Geometry<Renderer>> {
		let mut exporter = IcedExporter {
			frame: canvas::Frame::new(renderer, bounds.size()),
		};

		let shape_bb = self.local_bounding_box().straigthen();

		let translate =
			nalgebra::convert::<_, Transform2<f32>>(Translation2::from(-shape_bb.top_left()));

		let scale_x = bounds.width / shape_bb.width();
		let scale_y = bounds.height / shape_bb.height();

		let scale_min = scale_x.min(scale_y);

		let scale = nalgebra::convert::<_, Transform2<f32>>(Scale2::new(scale_min, scale_min));

		let default_transform = Transform2::identity() * scale * translate;

		self.write_into_exporter(
			&mut exporter,
			&default_transform,
			StylePosition {
				fill: None,
				stroke: None,
			},
		);

		vec![exporter.frame.into_geometry()]

		// let mut frame = canvas::Frame::new(renderer, bounds.size());

		// // We create a `Path` representing a simple circle
		// let circle = canvas::Path::circle(frame.center(), 10.);

		// // And fill it with some color
		// frame.fill(&circle, iced_core::Color::BLACK);

		// // Then, we produce the geometry
		// vec![frame.into_geometry()]
	}
}
