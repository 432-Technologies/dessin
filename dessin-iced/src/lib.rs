use std::ops::Deref;

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

pub enum IcedError {}

struct IcedExporter<Renderer: geometry::Renderer> {
	frame: Frame<Renderer>,
}
impl<Renderer: geometry::Renderer> Exporter for IcedExporter<Renderer> {
	type Error = IcedError;

	const CAN_EXPORT_ELLIPSE: bool = true;

	fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
		Ok(())
	}

	fn end_style(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn export_ellipse(
		&mut self,
		ellipse: EllipsePosition,
		style_position: StylePosition,
	) -> Result<(), Self::Error> {
		eprintln!("export_ellipse: {ellipse:?} {style_position:?}");

		let circle = canvas::Path::circle(
			Point {
				x: ellipse.center.x,
				y: -ellipse.center.y,
			},
			ellipse.semi_major_axis,
		);

		if let Some(fill) = style_position.fill {
			let fill = match fill {
				Fill::Solid { color } => canvas::Fill {
					style: canvas::Style::Solid(iced_core::Color {
						r: color.red,
						g: color.green,
						b: color.blue,
						a: color.alpha,
					}),
					rule: geometry::fill::Rule::NonZero,
				},
			};

			self.frame.fill(&circle, fill);
		}

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
		eprintln!("export_curve: {curve:?} {style_position:?}");

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

		Ok(())
	}

	fn export_text(&mut self, text: TextPosition, style: StylePosition) -> Result<(), Self::Error> {
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
		bounds: iced_core::Rectangle,
		_cursor: iced_core::mouse::Cursor,
	) -> Vec<canvas::Geometry<Renderer>> {
		let mut exporter = IcedExporter {
			frame: canvas::Frame::new(renderer, bounds.size()),
		};

		let shape_bb = self.local_bounding_box().straigthen();

		let scale_x = bounds.width / shape_bb.width();
		let scale_y = bounds.height / shape_bb.height();

		let scale_min = scale_x.min(scale_y);

		let x_diff = bounds.x / scale_min - shape_bb.left();
		let y_diff = bounds.y / scale_min - shape_bb.top();

		let center = shape_bb.center();

		eprintln!(
			"shape_bb = {shape_bb:?}
bb_center = {center:?},
bounds = {bounds:?}

scale_x = {scale_x}
scale_x = {scale_x}

x_diff = {x_diff}
y_diff = {y_diff}
"
		);

		let scale = nalgebra::convert::<_, Transform2<f32>>(Scale2::new(scale_min, scale_min));
		let translate = nalgebra::convert::<_, Transform2<f32>>(Translation2::new(x_diff, y_diff));

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
