use crate::{exporter::IcedExporter, CachedDessin};
use dessin::{
	export::Export,
	nalgebra::{self, Scale2, Transform2, Translation2},
	prelude::*,
};
use iced_widget::renderer::geometry;
use std::ops::Deref;

pub struct IcedShapeRef<'a>(pub &'a Shape, pub crate::Options);
impl<'a, Message, Theme, Renderer: geometry::Renderer>
	iced_widget::canvas::Program<Message, Theme, Renderer> for IcedShapeRef<'a>
{
	type State = ();

	fn draw(
		&self,
		_state: &Self::State,
		renderer: &Renderer,
		_theme: &Theme,
		bounds: iced_core::Rectangle,
		_cursor: iced_core::mouse::Cursor,
	) -> Vec<iced_widget::canvas::Geometry<Renderer>> {
		let mut frame = iced_widget::canvas::Frame::new(renderer, bounds.size());

		let mut exporter = IcedExporter { frame: &mut frame };

		let shape_bb = match self.1.viewport {
			crate::ViewPort::ManualCentered { width, height } => {
				BoundingBox::centered([width, height])
			}
			crate::ViewPort::ManualViewport {
				x,
				y,
				width,
				height,
			} => BoundingBox::at([x, y])
				.transform(&nalgebra::convert(Scale2::new(width, height)))
				.straigthen(),
			crate::ViewPort::AutoCentered => {
				let bb = self.0.local_bounding_box().straigthen();

				let mirror_bb = bb
					.transform(&nalgebra::convert::<_, Transform2<f32>>(Scale2::new(
						-1., -1.,
					)))
					.into_straight();

				bb.join(mirror_bb)
			}
			crate::ViewPort::AutoBoundingBox => self.0.local_bounding_box().straigthen(),
		};

		let translate =
			nalgebra::convert::<_, Transform2<f32>>(Translation2::from(-shape_bb.top_left()));

		let scale_x = bounds.width / shape_bb.width();
		let scale_y = bounds.height / shape_bb.height();

		let scale_min = scale_x.min(scale_y);

		let scale = nalgebra::convert::<_, Transform2<f32>>(Scale2::new(scale_min, scale_min));

		let default_transform = Transform2::identity() * scale * translate;

		self.0.write_into_exporter(
			&mut exporter,
			&default_transform,
			StylePosition {
				fill: None,
				stroke: None,
			},
		);

		vec![frame.into_geometry()]
	}
}

pub struct IcedShape(pub Shape, pub crate::Options);
impl<Message, Theme, Renderer: geometry::Renderer>
	iced_widget::canvas::Program<Message, Theme, Renderer> for IcedShape
{
	type State = ();

	fn draw(
		&self,
		state: &Self::State,
		renderer: &Renderer,
		theme: &Theme,
		bounds: iced_core::Rectangle,
		cursor: iced_core::mouse::Cursor,
	) -> Vec<iced_widget::canvas::Geometry<Renderer>> {
		iced_widget::canvas::Program::<Message, Theme, Renderer>::draw(
			&IcedShapeRef(&self.0, self.1.clone()),
			state,
			renderer,
			theme,
			bounds,
			cursor,
		)
	}
}

pub struct IcedShapeCached<'a, Renderer: geometry::Renderer>(pub &'a CachedDessin<Renderer>);
impl<'a, Renderer: geometry::Renderer> Deref for IcedShapeCached<'a, Renderer> {
	type Target = CachedDessin<Renderer>;

	fn deref(&self) -> &Self::Target {
		self.0
	}
}
impl<'a, Message, Theme, Renderer: geometry::Renderer>
	iced_widget::canvas::Program<Message, Theme, Renderer> for IcedShapeCached<'a, Renderer>
{
	type State = ();

	fn draw(
		&self,
		_state: &Self::State,
		renderer: &Renderer,
		_theme: &Theme,
		bounds: iced_core::Rectangle,
		_cursor: iced_core::mouse::Cursor,
	) -> Vec<iced_widget::canvas::Geometry<Renderer>> {
		let frame = self.cache.draw(renderer, bounds.size(), |frame| {
			let mut exporter = IcedExporter { frame };

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
		});

		vec![frame]
	}
}
