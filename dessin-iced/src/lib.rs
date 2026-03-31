mod canvas;
mod exporter;

use crate::canvas::{IcedShape, IcedShapeCached, IcedShapeRef};
use dessin::prelude::*;
use iced_widget::renderer::geometry;
use std::ops::{Deref, DerefMut};

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

#[derive(Default, Clone)]
pub struct Options {
	pub viewport: ViewPort,
}

pub trait DessinIced<Message, Theme, Renderer: geometry::Renderer> {
	type Out: iced_widget::canvas::Program<Message, Theme, Renderer>;

	fn view(self) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer>
	where
		Self: Sized,
	{
		self.view_with(Options::default())
	}

	fn view_with(
		self,
		options: Options,
	) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer>;
}

impl<'a, Message, Theme, Renderer: geometry::Renderer> DessinIced<Message, Theme, Renderer>
	for &'a Shape
{
	type Out = IcedShapeRef<'a>;

	fn view_with(
		self,
		options: Options,
	) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShapeRef(self, options))
	}
}

impl<Message, Theme, Renderer: geometry::Renderer> DessinIced<Message, Theme, Renderer> for Shape {
	type Out = IcedShape;

	fn view_with(
		self,
		options: Options,
	) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShape(self, options))
	}
}

pub struct CachedDessin<Renderer: geometry::Renderer> {
	shape: Shape,
	cache: iced_widget::canvas::Cache<Renderer>,
}
impl<Renderer: geometry::Renderer> Default for CachedDessin<Renderer> {
	fn default() -> Self {
		Self {
			shape: Default::default(),
			cache: iced_widget::canvas::Cache::new(),
		}
	}
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

	fn view_with(
		self,
		options: Options,
	) -> iced_widget::Canvas<Self::Out, Message, Theme, Renderer> {
		iced_widget::canvas(IcedShapeCached(self))
	}
}
