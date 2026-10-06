mod exporter;

use dessin::{
	export::{Export, ViewPort},
	nalgebra::{self, Scale2, Transform2, Translation2},
	prelude::*,
};
use iced_core::widget::Meta;
use iced_core::{Length, Size, Theme, Widget};
use iced_widget::renderer::geometry;

pub trait Shapable {
	fn map<T>(&self, theme: &Theme, f: impl FnOnce(&Shape) -> T) -> T;
}
impl Shapable for Shape {
	fn map<T>(&self, _theme: &Theme, f: impl FnOnce(&Shape) -> T) -> T {
		f(self)
	}
}
impl<F: Fn(&Theme) -> Shape> Shapable for F {
	fn map<T>(&self, theme: &Theme, f: impl FnOnce(&Shape) -> T) -> T {
		f(&self(theme))
	}
}

pub fn dessin<S: Shapable>(dessin: S) -> Dessin<S> {
	Dessin {
		dessin,
		viewport: Default::default(),
		width: Length::Fill,
		height: Length::Fill,
	}
}
pub struct Dessin<S: Shapable> {
	dessin: S,
	viewport: ViewPort,
	width: Length,
	height: Length,
}
impl<S: Shapable> Dessin<S> {
	#[must_use]
	pub fn viewport(mut self, viewport: ViewPort) -> Self {
		self.viewport = viewport;
		self
	}

	#[must_use]
	pub fn width(mut self, width: impl Into<Length>) -> Self {
		self.width = width.into();
		self
	}

	#[must_use]
	pub fn height(mut self, height: impl Into<Length>) -> Self {
		self.height = height.into();
		self
	}
}
impl<S: Shapable, Message, Renderer: iced_core::Renderer + geometry::Renderer>
	Widget<Message, Theme, Renderer> for Dessin<S>
{
	fn size(&self) -> iced_core::Size<iced_core::Length> {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn layout(
		&mut self,
		tree: &mut iced_core::widget::Tree,
		_renderer: &Renderer,
		limits: &iced_core::layout::Limits,
	) {
		tree.size = limits.resolve(self.width, self.height, Size::ZERO);
	}

	fn draw(
		&self,
		_tree: &iced_core::widget::Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		_style: &iced_core::renderer::Style,
		layout: iced_core::Layout,
		_cursor: iced_core::mouse::Cursor,
		_viewport: &iced_core::Rectangle,
	) {
		self.dessin.map(theme, |shape| {
			let bounds = layout.bounds();
			if bounds.width < 1.0 || bounds.height < 1.0 {
				return;
			}

			let bb = self.viewport.bounding_box(shape);

			let width_margin = bounds.width - bb.width();
			let height_margin = bounds.height - bb.height();

			let scale_factor = if width_margin <= height_margin {
				bounds.width / bb.width()
			} else {
				bounds.height / bb.height()
			};

			renderer.with_layer(bounds, |renderer| {
				renderer.with_translation(iced_core::Vector::new(bounds.x, bounds.y), |renderer| {
					let mut exporter = exporter::FrameWriter {
						frame: iced_widget::canvas::Frame::new(renderer, bounds.size()),
					};

					let scale: Transform2<f32> =
						nalgebra::convert(Scale2::new(scale_factor, scale_factor));
					let translation: Transform2<f32> =
						nalgebra::convert(Translation2::new(-bb.left(), -bb.top()));

					let parent_transform = scale * translation;

					shape
						.write_into_exporter(&mut exporter, &parent_transform, Default::default())
						.unwrap();

					renderer.draw_geometry(exporter.frame.into_geometry());
				});
			});
		});
	}
}

impl<S: Shapable> Meta for Dessin<S> {
	fn is_void(&self) -> bool {
		false
	}
}
