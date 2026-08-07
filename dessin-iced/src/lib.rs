// mod canvas;
mod exporter;

// use crate::canvas::{IcedShape, IcedShapeCached, IcedShapeRef};
use dessin::{
	export::{Export, ViewPort},
	nalgebra::{self, Scale2, Transform2, Translation2},
	prelude::*,
};
use iced_core::{Element, Length, Size, Widget};
use iced_widget::{canvas, renderer::geometry};

pub fn dessin(dessin: Shape) -> Dessin {
	Dessin {
		// wrap dessin in a group to have a free bounding box cache
		dessin: Group::from(dessin).into(),
		viewport: Default::default(),
		width: Length::Fill,
		height: Length::Fill,
	}
}
pub struct Dessin {
	dessin: Shape,
	viewport: ViewPort,
	width: Length,
	height: Length,
}
impl Dessin {
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
impl<Message, Theme, Renderer: iced_core::Renderer + geometry::Renderer>
	Widget<Message, Theme, Renderer> for Dessin
{
	fn size(&self) -> iced_core::Size<iced_core::Length> {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn layout(
		&mut self,
		_tree: &mut iced_core::widget::Tree,
		_renderer: &Renderer,
		limits: &iced_core::layout::Limits,
	) -> iced_core::layout::Node {
		let size = limits.resolve(self.width, self.height, Size::ZERO);
		iced_core::layout::Node::new(size)
	}

	fn draw(
		&self,
		_tree: &iced_core::widget::Tree,
		renderer: &mut Renderer,
		_theme: &Theme,
		_style: &iced_core::renderer::Style,
		layout: iced_core::Layout<'_>,
		_cursor: iced_core::mouse::Cursor,
		_viewport: &iced_core::Rectangle,
	) {
		let bounds = layout.bounds();
		if bounds.width < 1.0 || bounds.height < 1.0 {
			return;
		}

		let bb = self.viewport.bounding_box(&self.dessin);

		let width_margin = bounds.width - bb.width();
		let height_margin = bounds.height - bb.height();

		let scale_factor = if width_margin <= height_margin {
			bounds.width / bb.width()
		} else {
			bounds.height / bb.height()
		};

		renderer.with_translation(iced_core::Vector::new(bounds.x, bounds.y), |renderer| {
			let mut exporter = exporter::FrameWriter {
				frame: iced_widget::canvas::Frame::new(renderer, bounds.size()),
			};

			let scale: Transform2<f32> = nalgebra::convert(Scale2::new(scale_factor, scale_factor));
			let translation: Transform2<f32> =
				nalgebra::convert(Translation2::new(-bb.left(), -bb.top()));

			let parent_transform = scale * translation;

			self.dessin
				.write_into_exporter(&mut exporter, &parent_transform, Default::default())
				.unwrap();

			renderer.draw_geometry(exporter.frame.into_geometry());
		});
	}
}
impl<'a, Message, Theme, Renderer: iced_core::Renderer + geometry::Renderer> From<Dessin>
	for Element<'a, Message, Theme, Renderer>
{
	fn from(value: Dessin) -> Self {
		Element::new(value)
	}
}
