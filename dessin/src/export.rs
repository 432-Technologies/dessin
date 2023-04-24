use nalgebra::Transform2;

use crate::prelude::*;

pub trait Export<E>
where
    E: Exporter,
{
    fn write_into_exporter(
        &self,
        exporter: &mut E,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), <E as Exporter>::Error>;
}

impl<E> Export<E> for Shape
where
    E: Exporter,
{
    fn write_into_exporter(
        &self,
        exporter: &mut E,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), <E as Exporter>::Error> {
        match self {
            Shape::Group {
                local_transform,
                shapes,
            } => {
                let parent_transform = parent_transform * local_transform;
                for shape in shapes {
                    shape.write_into_exporter(exporter, &parent_transform)?;
                }

                Ok(())
            }
            Shape::Style {
                fill,
                stroke,
                shape,
            } => {
                let style = StylePosition {
                    fill: fill.clone(),
                    stroke: stroke.clone().map(|v| *parent_transform * v),
                };

                exporter.start_style(style)?;
                shape.write_into_exporter(exporter, parent_transform)?;
                exporter.end_style()
            }
            Shape::Image(image) => exporter.export_image(image.position(parent_transform)),
            Shape::Ellipse(ellipse) => exporter.export_ellipse(ellipse.position(parent_transform)),
            Shape::Curve(curve) => exporter.export_curve(curve.position(parent_transform)),
            Shape::Text(text) => exporter.export_text(text.position(parent_transform)),
        }
    }
}

pub trait Exporter {
    type Error;

    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error>;
    fn end_style(&mut self) -> Result<(), Self::Error>;

    fn export_image(&mut self, image: ImagePosition) -> Result<(), Self::Error>;
    fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error>;
    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error>;
    fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error>;
}
