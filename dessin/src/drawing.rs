use crate::{
    shapes::{embedded::EmbeddedDrawing, Shape},
    Size,
};
use algebr::Vec2;

/// Drawing is a collection of shapes.
/// ```
/// # use dessin::{
/// #     Drawing,
/// #     shape::{
/// #         Text,
/// #         Line,
/// #         Circle,
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
    pub(crate) position: Vec2,
    pub(crate) canvas_size: Size,
    pub(crate) shapes: Vec<Shape>,
}
impl Drawing {
    /// Default constructor, creates an empty drawing.
    pub const fn empty() -> Self {
        Drawing {
            position: Vec2::zero(),
            canvas_size: Vec2::zero(),
            shapes: vec![],
        }
    }

    /// Construct a drawing with a shape.
    pub fn new<T>(shape: T) -> Self
    where
        T: Into<Shape>,
    {
        let s: Shape = shape.into();
        let pos = s.pos;
        let mut d = Drawing::empty().with_canvas_size(pos.size());
        d.position = pos.pos;
        d.add(s);
        d
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

    pub fn into_embedded(self) -> EmbeddedDrawing {
        EmbeddedDrawing::new(self)
    }

    /// Get access to this drawing's shapes.
    /// ```
    /// # use dessin::{
    /// #     Drawing,
    /// #     shape::{
    /// #         Text,
    /// #         Line,
    /// #         Circle,
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

impl Into<Shape> for Drawing {
    fn into(self) -> Shape {
        EmbeddedDrawing::new(self).into()
    }
}
