//! Documentation to export a dessin in a specific format.
//!
//! You should probably head to [`Exporter`][crate::export::Exporter]
//!
//!
//! ## Examples
//! Examples can be found for [PDF](https://docs.rs/dessin-pdf/) or [SVG](https://docs.rs/dessin-svg/)
use crate::prelude::*;
use nalgebra::Transform2;

/// Orchestrator of the export
///
/// The Export walks the dessin graph and orchestrate an [`Exporter`] of a given format.
/// Unless you need a specific export behavious, you should not need to implement this trait as it is already imiplemented for [`Shape`][crate::shapes::Shape].
pub trait Export<E>
where
    E: Exporter,
{
    /// Start the export of a dessin.
    ///
    /// Example in a Dummy [`Exporter`]:
    /// ```
    /// # fn main() {
    /// # use dessin::{prelude::*, export::*};
    /// struct MyDummyExporter;
    /// impl Exporter for MyDummyExporter { // Hidden implementation
    /// # type Error = ();
    /// # fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> { Ok(()) }
    /// # fn end_style(&mut self) -> Result<(), Self::Error> { Ok(()) }
    /// # fn export_image(&mut self, image: ImagePosition) -> Result<(), Self::Error> { Ok(()) }
    /// # fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error> { Ok(()) }
    /// # fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> { Ok(()) }
    /// # fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error> { Ok(()) }
    /// }
    ///
    /// fn export_shape(shape: Shape) {
    /// 	let mut my_dummy_exporter = MyDummyExporter;
    ///
    /// 	shape.write_into_exporter( // Start walking the dessin
    /// 		&mut my_dummy_exporter,
    /// 		&Default::default(),
    /// 	);
    /// }
    ///
    /// export_shape(dessin!());
    /// # }
    /// ```
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
            Shape::Dynamic { .. } => todo!(),
        }
    }
}

/// Writer to a given format
///
/// Implementation hint:
/// The implementation is a bit opiniated as you don't have the control about when a given will be called by the [`Export`].
/// You should probably store a state inside the [`Exporter`] and mutate it as each function is called.
///
/// As a user isn't expect to call the method in this module directly, you still have a bit of control over the export.
///
/// Here is an idea of how the export in [SVG](https://docs.rs/dessin-svg/) is done:
/// We need to add the closing tag `</svg>` after exporting everything.
/// ```
/// # use dessin::{prelude::*, export::*};
/// struct SVGExport { state: String }
/// impl SVGExport {
/// 	fn new() -> Self {
/// 		SVGExport { state: "<svg>".to_string() }
/// 	}
///
/// 	fn finish(self) -> String { // Add the closing tag and give the accumulated state
/// 		format!("{}</svg>", self.state) // Closing tag
/// 	}
/// }
/// impl Exporter for SVGExport { // Hidden implementation
/// # type Error = ();
/// # fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> { Ok(()) }
/// # fn end_style(&mut self) -> Result<(), Self::Error> { Ok(()) }
/// # fn export_image(&mut self, image: ImagePosition) -> Result<(), Self::Error> { Ok(()) }
/// # fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error> { Ok(()) }
/// # fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> { Ok(()) }
/// # fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error> { Ok(()) }
/// }
///
/// trait ToSVG {
/// 	fn to_svg(&self) -> String;
/// }
/// impl ToSVG for Shape {
/// 	fn to_svg(&self) -> String {
/// 		let mut exporter = SVGExport::new();
///
/// 		self.write_into_exporter( // Start walking the dessin
/// 			&mut exporter,
/// 			&Default::default(), // In the real implementation, we need to mirror the Y axis, as the positive side is in the DOWN side
/// 		).unwrap();
///
/// 		exporter.finish()
/// 	}
/// }
///
/// fn main() {
/// 	let svg = dessin!().to_svg();
/// }
/// ```
pub trait Exporter {
    /// Export error
    type Error;

    /// Enter a scope of style
    ///
    /// All [`Shape`][crate::shapes::Shape] between [`start_style`][Exporter::start_style] and [`end_style`][Exporter::end_style] must have this style applied to them.
    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error>;
    /// End a scope of style
    fn end_style(&mut self) -> Result<(), Self::Error>;

    /// Export an [`Image`][crate::shapes::image::Image]
    fn export_image(&mut self, image: ImagePosition) -> Result<(), Self::Error>;
    /// Export an [`Ellipse`][crate::shapes::ellipse::Ellipse]
    fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error>;
    /// Export a [`Curve`][crate::shapes::curve::Curve]
    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error>;
    /// Export a [`Text`][crate::shapes::text::Text]
    fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error>;
}
