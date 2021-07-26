use crate::{
    position::Rect,
    shapes::{
        arc::Arc, circle::Circle, embedded::EmbeddedDrawing, image::Image, line::Line, text::Text,
        Shape, ShapeType,
    },
    Size,
};
use algebr::{vec2, Vec2};

/// Drawing is a collection of shapes.
/// ```
/// # use dessin::{
/// #     Drawing,
/// #     shape::{
/// #         Text,
/// #         Line,
/// #         Circle,
/// #         Arc,
/// #         { Image, ImageFormat },
/// #         EmbeddedDrawing,
/// #     },
/// #     vec2,
/// #     Angle,
/// # };
///
/// let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
///
/// drawing.add(
///         Text::new("Hello World".to_owned())
///             .at(vec2(50., 50.))
///     )
///     .add(
///         Line::from(vec2(0., 0.)).to(vec2(100., 100.))
///     )
///     .add(
///         Circle::new()
///             .at(vec2(50., 50.)).with_radius(10.)
///     )
///     .add(
///         Arc::new()
///             .at(vec2(50., 50.))
///             .with_inner_radius(10.)
///             .with_outer_radius(20.)
///             .with_start_angle(Angle::deg(0.))
///             .with_end_angle(Angle::deg(180.))
///     )
///     .add(
///         Image::new(ImageFormat::PNG(include_bytes!("../rustacean-flat-happy.png").to_vec()))
///             .at(vec2(50., 50.))
///             .with_size(vec2(10., 10.))
///     );
///     
/// let other_drawing = Drawing::empty()
///     .with_canvas_size(vec2(210., 297.))
///     .add(
///         EmbeddedDrawing::new(drawing)
///             .at(vec2(100., 100.))
///             .with_size(vec2(10., 10.))
///     );
/// ```
#[derive(Debug, Clone)]
pub struct Drawing {
    pub(crate) canvas_size: Size,
    pub(crate) shapes: Vec<Shape>,
}
impl Drawing {
    /// Default constructor, creates an empty drawing.
    pub const fn empty() -> Self {
        Drawing {
            canvas_size: vec2(0., 0.),
            shapes: vec![],
        }
    }

    pub const fn with_canvas_size(mut self, canvas_size: Vec2) -> Self {
        self.canvas_size = canvas_size;
        self
    }

    pub const fn canvas_size(&self) -> Vec2 {
        self.canvas_size
    }

    pub fn add<T>(&mut self, shape: T) -> &mut Self
    where
        T: Into<Shape>,
    {
        self.shapes.push(shape.into());
        self
    }

    /// Get access to this drawing's shapes.
    /// ```
    /// # use dessin::{
    /// #     Drawing,
    /// #     shape::{
    /// #         Text,
    /// #         Line,
    /// #         Circle,
    /// #         Arc,
    /// #         { Image, ImageFormat },
    /// #     },
    /// #     vec2,
    /// # };
    ///
    /// let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
    ///
    /// drawing.add(
    ///         Text::new("Hello World".to_owned())
    ///             .at(vec2(50., 50.))
    ///     )
    ///     .add(
    ///         Line::from(vec2(0., 0.)).to(vec2(100., 100.))
    ///     );
    ///
    /// let shapes = drawing.shapes();
    /// dbg!("{:?}", shapes);
    /// ```
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }
}
