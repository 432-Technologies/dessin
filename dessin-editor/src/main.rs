mod macros;

use dessin::{
	nalgebra::{Point2, Vector2},
	palette::Srgba,
	prelude::*,
};
use dessin_dioxus::{SVGOptions, ViewPort, SVG};
use dioxus::{logger::tracing::debug, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	let mut shape = use_signal(|| {
		Shape::from(WindToggle {
			color: Srgba::new(1., 0.2, 0.1, 1.),
			morph: 0.,
		})
	});

	rsx! {
		script {
			src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4",
		}
		document::Link { rel: "icon", href: FAVICON }

		div {
			class: "flex w-screen h-screen",

			div {
				class: "w-40",
				Inspector {}
			}
			div {
				class: "flex-1",
				Canvas {
					shape,
					canvas_size: Signal::new(CanvasSize {
						x: 0.,
						y: 0.,
						width: 10.,
						height: 10.,
					}),
				}
			}
		}
	}
}

#[component]
fn Inspector() -> Element {
	rsx! {}
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
	handle_prev: Point2<f32>,
	pos: Point2<f32>,
	handle_next: Point2<f32>,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct CanvasSize {
	x: f32,
	y: f32,
	width: f32,
	height: f32,
}
impl From<CanvasSize> for ViewPort {
	fn from(
		CanvasSize {
			x,
			y,
			width,
			height,
		}: CanvasSize,
	) -> Self {
		ViewPort::ManualViewport {
			x,
			y,
			width,
			height,
		}
	}
}

#[component]
fn Canvas(shape: Signal<Shape>, canvas_size: Signal<CanvasSize>) -> Element {
	let mut size = use_signal(|| Vector2::new(0., 0.));
	let width = use_memo(move || size().min());

	let points = use_signal(|| {
		vec![Point {
			pos: Point2::new(1., 1.),
			handle_prev: Point2::new(1., 1.),
			handle_next: Point2::new(1., 1.),
		}]
	});

	rsx! {
		div {
			class: "w-full h-full relative",
			onresize: move |ev| {
				let s = ev.data.get_content_box_size().unwrap();
				size.set(Vector2::new(s.width, s.height));
			},

			div {
				div {}
				for point in points() {
					div {
						class: "absolute",
						style: "left:{point.pos.x }px;top:{point.pos.y}px",
					}
				}
				// style: "width:{width}px;height:{width}px;",
				// SVG {
				// 	class: "w-full h-full",
				// 	shape,
				// 	options: SVGOptions {
				// 		viewport: canvas_size().into(),
				// 	}
				// }
			}
		}
	}
}

#[derive(Default, Shape)]
pub struct WindToggle {
	pub morph: f32,
	#[shape(into)]
	pub color: Srgba,
}
impl From<WindToggle> for Shape {
	fn from(value: WindToggle) -> Self {
		const C: f32 = 0.55342686;
		const H: f32 = -0.05;
		const B: f32 = -0.15;
		const R1: f32 = 0.1;
		const R2: f32 = R1 + H - B;
		const C1: f32 = C * R1;
		const C2: f32 = C * R2;

		const X1: f32 = 0.;
		const X2: f32 = 0.3;

		dessin!(
			[
				*Curve(
					fill = value.color,
					// stroke = Stroke::Full {
					// 	color: value.color.into(),
					// 	width: 0.01
					// },
					extend = [
						lerp!(
							value.morph,
							Point ([-0.6, -0.2] => [X2 + -1., H])
						),
						lerp!(
							value.morph,
							Point ([-0.6, -0.2] => [X2 + -1., B])
						),
						lerp!(
							value.morph,
							Point ([-0.5, -0.9] => [X2 + -0.2, B])
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [-0.5, -0.9] => [X2 + -0.2 + C1, B],
								end_control: [-0.33, -0.9] => [X2 + -0.2 + R1, B - R1 + C1],
								end: [-0.33, -0.9] => [X2 + -0.2 + R1, B - R1],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [-0.33, -0.9] => [X2 + -0.2 + R1, B - R1 - C1],
								end_control: [0., -0.9] => [X2 + -0.2 + C1, B - 2. * R1],
								end: [0., -0.9] => [X2 + -0.2, B - 2. * R1],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0., -0.9] => [X2 + -0.2 - C1, B - 2. * R1],
								end_control: [0.33, -0.9] => [X2 + -0.2 - R1, B - R1 - C1],
								end: [0.33, -0.9] => [0., -0.9] => [-0.3, -0.9] => [X2 + -0.2 - R1, B - R1],
							}
						),
						lerp!(
							value.morph,
							Point ([0.5, -0.9] => [0.1, -0.9] => [-0.5, -0.9] => [X2 + -0.2 - R2, H - R2])
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.5, -0.9] => [0.2, -0.9] => [X2 + -0.2 - R2, H - R2 - C2],
								end_control: [0.6, -0.2] => [0.8, -1.] => [X2 + -0.2 - C2, H - 2. * R2],
								end: [0.6, -0.2] => [0.8, -0.9] => [X2 + -0.2, H - 2. * R2],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.6, -0.2] => [0.8, 0.] => [X2 + -0.2 + C2, H - 2. * R2],
								end_control: [0.0, -0.4] => [0.7, 0.2] => [X2 + -0.2 + R2, H - R2 - C2],
								end: [0.0, -0.4] => [0.5, 0.2] => [X2 + -0.2 + R2, H - R2],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.0, -0.4] => [X2 + -0.2 + R2, H - R2 + C2],
								end_control: [0., -0.4] => [X2 + -0.2 + C2, H],
								end: [0., -0.4] => [X2 + -0.2, H],
							}
						),
						lerp!(
							value.morph,
							Point ([0., -0.4] => [X2 + -0.6, H])
						),
					],
					closed,
				) > (scale = [1., -1.]),
				*Curve(
					fill = value.color,
					// stroke = Stroke::Full {
					// 	color: value.color.into(),
					// 	width: 0.01
					// },
					extend = [
						lerp!(
							value.morph,
							Point ([-1., 0.] => [X1 + -1., H])
						),
						lerp!(
							value.morph,
							Point ([-1., 0.] => [X1 + -1., B])
						),
						lerp!(
							value.morph,
							Point ([-0.66, -1.] => [X1 + -0.2, B])
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [-0.66, -1.] => [X1 + -0.2 + C1, B],
								end_control: [-0.33, -1.] => [X1 + -0.2 + R1, B - R1 + C1],
								end: [-0.33, -1.] => [X1 + -0.2 + R1, B - R1],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [-0.33, -1.] => [X1 + -0.2 + R1, B - R1 - C1],
								end_control: [0., -1.] => [X1 + -0.2 + C1, B - 2. * R1],
								end: [0., -1.] => [X1 + -0.2, B - 2. * R1],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0., -1.] => [X1 + -0.2 - C1, B - 2. * R1],
								end_control: [0.33, -1.] => [X1 + -0.2 - R1, B - R1 - C1],
								end: [0.33, -1.] => [0., -0.9] => [-0.3, -0.9] => [X1 + -0.2 - R1, B - R1],
							}
						),
						lerp!(
							value.morph,
							Point ([0.66, -1.] => [0.1, -1.] => [-0.5, -1.] => [X1 + -0.2 - R2, H - R2])
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.66, -1.] => [0.2, -1.] => [X1 + -0.2 - R2, H - R2 - C2],
								end_control: [1., 0.] => [0.8, -1.] => [X1 + -0.2 - C2, H - 2. * R2],
								end: [1., 0.] => [0.8, -0.9] => [X1 + -0.2, H - 2. * R2],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.05, 0.3] => [0.8, 0.] => [X1 + -0.2 + C2, H - 2. * R2],
								end_control: [0.05, 0.3] => [0.7, 0.2] => [X1 + -0.2 + R2, H - R2 - C2],
								end: [0.05, 0.3] => [0.5, 0.2] => [X1 + -0.2 + R2, H - R2],
							}
						),
						lerp!(
							value.morph,
							Bezier {
								start_control: [0.05, 0.3] => [X1 + -0.2 + R2, H - R2 + C2],
								end_control: [0., -0.4] => [X1 + -0.2 + C2, H],
								end: [0., -0.4] => [X1 + -0.2, H],
							}
						),
						lerp!(
							value.morph,
							Point ([-0.05, 0.3] => [X1 + -0.6, H])
						),
					],
					closed,
				)
			] > (
				translate = Point2::from(lerp!(value.morph, [0., 0.] => [0.25, 0.])),
				scale = Point2::from(lerp!(value.morph, [1., 1.] => [1.5, 1.5]))
			)
		)
	}
}
