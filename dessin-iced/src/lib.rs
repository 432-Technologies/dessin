mod canvas;
mod exporter;

use crate::canvas::{IcedShape, IcedShapeCached, IcedShapeRef};
use dessin::prelude::*;
use iced_widget::renderer::geometry;
use std::ops::{Deref, DerefMut};

pub trait DessinIced<Message, Theme, Renderer: geometry::Renderer> {
	type Out: iced_widget::canvas::Program<Message, Theme, Renderer>;

	fn view(self) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer>;
}

impl<'a, Message, Theme, Renderer: geometry::Renderer> DessinIced<Message, Theme, Renderer>
	for &'a Shape
{
	type Out = IcedShapeRef<'a>;

	fn view(self) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShapeRef(self))
	}
}

impl<Message, Theme, Renderer: geometry::Renderer> DessinIced<Message, Theme, Renderer> for Shape {
	type Out = IcedShape;

	fn view(self) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShape(self))
	}
}

pub struct CachedDessin<Renderer: geometry::Renderer> {
	shape: Shape,
	cache: iced_widget::canvas::Cache<Renderer>,
}
impl<Renderer: geometry::Renderer> Deref for CachedDessin<Renderer> {
	type Target = Shape;

	fn deref(&self) -> &Self::Target {
		&self.shape
	}
}
impl<Renderer: geometry::Renderer> DerefMut for CachedDessin<Renderer> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.request_redraw();
		&mut self.shape
	}
}
impl<Renderer: geometry::Renderer> CachedDessin<Renderer> {
	pub fn new(shape: Shape) -> Self {
		CachedDessin {
			shape,
			cache: iced_widget::canvas::Cache::new(),
		}
	}

	pub fn request_redraw(&mut self) {
		self.cache.clear();
	}
}
impl<'a, Message, Theme, Renderer: geometry::Renderer + 'a> DessinIced<Message, Theme, Renderer>
	for &'a CachedDessin<Renderer>
{
	type Out = IcedShapeCached<'a, Renderer>;

	fn view(self) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShapeCached(self))
	}
}
