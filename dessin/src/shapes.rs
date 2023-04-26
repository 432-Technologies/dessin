pub(crate) mod curve;
pub(crate) mod ellipse;
pub(crate) mod image;
pub(crate) mod text;

pub use self::image::*;
pub use curve::*;
pub use ellipse::*;
use na::{Point2, Rotation2, Scale2};
use nalgebra::{self as na, Transform2, Translation2};
use std::marker::PhantomData;
pub use text::*;

/// Transforming operation on shapes such as:
/// - a translation with [`translate`][ShapeOp::translate]
/// - a scale with [`scale`][ShapeOp::scale]
/// - a rotation with [`rotate`][ShapeOp::rotate]
/// - any other transform with [`transform`][ShapeOp::transform]
pub trait ShapeOp: Into<Shape> + Clone {
    /// Apply an ordinary transform.
    /// You don't need to implement [`translate`][ShapeOp::translate], [`scale`][ShapeOp::scale] or [`rotate`][ShapeOp::rotate]
    /// yourself as a blanket implementation is given with this transform.
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self;

    /// Translation
    #[inline]
    fn translate<T: Into<Translation2<f32>>>(&mut self, translation: T) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(translation.into()));
        self
    }
    /// Scale
    #[inline]
    fn scale<S: Into<Scale2<f32>>>(&mut self, scale: S) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(scale.into()));
        self
    }
    /// Rotation
    #[inline]
    fn rotate<R: Into<Rotation2<f32>>>(&mut self, rotation: R) -> &mut Self {
        self.transform(na::convert::<_, Transform2<f32>>(rotation.into()));
        self
    }

    /// Get own local transform.
    /// Required for the blanket implementation of [`global_transform`][ShapeOp::global_transform].
    fn local_transform(&self) -> &Transform2<f32>;
    /// Absolute transform given the parent transform
    #[inline]
    fn global_transform(&self, parent_transform: &Transform2<f32>) -> Transform2<f32> {
        parent_transform * self.local_transform()
    }
}

/// Same as [`ShapeOp`] but for chaining methods.
/// All shapes that implement [`ShapeOp`] also implement [`ShapeOpWith`] for free.
pub trait ShapeOpWith: ShapeOp {
    /// Transform
    #[inline]
    fn with_transform(mut self, transform_matrix: Transform2<f32>) -> Self {
        self.transform(transform_matrix);
        self
    }

    /// Translate
    #[inline]
    fn with_translate<T: Into<Translation2<f32>>>(mut self, translation: T) -> Self {
        self.translate(translation);
        self
    }
    /// Resize
    #[inline]
    fn with_resize<S: Into<Scale2<f32>>>(mut self, scale: S) -> Self {
        self.scale(scale);
        self
    }
    /// Rotate
    #[inline]
    fn with_rotate<R: Into<Rotation2<f32>>>(mut self, rotation: R) -> Self {
        self.rotate(rotation);
        self
    }
}
impl<T: ShapeOp> ShapeOpWith for T {}

/// Marker discribing the state of a bounding box.
/// With this marker, the bounding box may be skew or rotated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnParticular;
/// Marker discribing the state of a bounding box.
/// With this marker, the sides of the bounding box are guaranteed to be aligned with the X and Y axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Straight;

/// Bounding box used to describe max bound of an shape.
/// Usefull to find the max size of shapes as multiple [`BoundingBox`] can be join together.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox<Type> {
    _ty: PhantomData<Type>,
    top_left: Point2<f32>,
    top_right: Point2<f32>,
    bottom_right: Point2<f32>,
    bottom_left: Point2<f32>,
}
impl<T> BoundingBox<T> {
    /// Top left corner
    ///
    /// ⚠️ There is no guarantee that this is actually the most top and left corner.
    /// [`straigthen`][BoundingBox::straigthen] the [`BoundingBox`] first for this guarantee.
    pub fn top_left(&self) -> Point2<f32> {
        self.top_left
    }
    /// Top right corner
    ///
    /// ⚠️ There is no guarantee that this is actually the most top and right corner.
    /// [`straigthen`][BoundingBox::straigthen] the [`BoundingBox`] first for this guarantee.
    pub fn top_right(&self) -> Point2<f32> {
        self.top_right
    }
    /// bottom right corner
    ///
    /// ⚠️ There is no guarantee that this is actually the most bottom and right corner.
    /// [`straigthen`][BoundingBox::straigthen] the [`BoundingBox`] first for this guarantee.
    pub fn bottom_right(&self) -> Point2<f32> {
        self.bottom_right
    }
    /// Bottom left corner
    ///
    /// ⚠️ There is no guarantee that this is actually the most bottom and left corner.
    /// [`straigthen`][BoundingBox::straigthen] the [`BoundingBox`] first for this guarantee.
    pub fn bottom_left(&self) -> Point2<f32> {
        self.bottom_left
    }

    /// Straighen a [`BoundingBox`] and guarantee that the sides of the bounding box are aligns with the X and Y axis.
    pub fn straigthen(&self) -> BoundingBox<Straight> {
        let top = self
            .top_left
            .y
            .max(self.top_right.y)
            .max(self.bottom_left.y)
            .max(self.bottom_right.y);
        let bottom = self
            .top_left
            .y
            .min(self.top_right.y)
            .min(self.bottom_left.y)
            .min(self.bottom_right.y);

        let right = self
            .top_left
            .x
            .max(self.top_right.x)
            .max(self.bottom_left.x)
            .max(self.bottom_right.x);
        let left = self
            .top_left
            .x
            .min(self.top_right.x)
            .min(self.bottom_left.x)
            .min(self.bottom_right.x);

        BoundingBox {
            _ty: PhantomData,
            top_left: Point2::new(left, top),
            top_right: Point2::new(right, top),
            bottom_right: Point2::new(right, bottom),
            bottom_left: Point2::new(left, bottom),
        }
    }

    /// Apply a transform to a [`BoundingBox`]
    pub fn transform(self, transform: &Transform2<f32>) -> BoundingBox<UnParticular> {
        BoundingBox {
            _ty: PhantomData,
            top_left: transform * self.top_left,
            top_right: transform * self.top_right,
            bottom_right: transform * self.bottom_right,
            bottom_left: transform * self.bottom_left,
        }
    }

    /// Width
    pub fn width(&self) -> f32 {
        (self.top_right - self.top_left).magnitude()
    }

    /// Height
    pub fn height(&self) -> f32 {
        (self.top_right - self.bottom_right).magnitude()
    }
}

impl BoundingBox<UnParticular> {
    /// Create a [`BoundingBox`] from each corner
    pub fn new(
        top_left: Point2<f32>,
        top_right: Point2<f32>,
        bottom_right: Point2<f32>,
        bottom_left: Point2<f32>,
    ) -> Self {
        BoundingBox {
            _ty: PhantomData,
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl BoundingBox<Straight> {
    /// [`BoundingBox`] center at the origin
    pub fn zero() -> Self {
        BoundingBox {
            _ty: PhantomData,
            top_left: Point2::origin(),
            top_right: Point2::origin(),
            bottom_right: Point2::origin(),
            bottom_left: Point2::origin(),
        }
    }

    /// [`BoundingBox`] center at the given point
    pub fn at(p: Point2<f32>) -> Self {
        BoundingBox {
            _ty: PhantomData,
            top_left: p,
            top_right: p,
            bottom_right: p,
            bottom_left: p,
        }
    }

    /// Convert the [`BoundingBox`] to [`UnParticular`].
    pub fn as_unparticular(self) -> BoundingBox<UnParticular> {
        BoundingBox {
            _ty: PhantomData,
            top_left: self.top_left,
            top_right: self.top_right,
            bottom_right: self.bottom_right,
            bottom_left: self.bottom_left,
        }
    }

    /// A u B
    ///
    /// Creates a bigger [`BoundingBox`] from the union of the two.
    pub fn join(mut self, other: BoundingBox<Straight>) -> BoundingBox<Straight> {
        self.top_left.x = self.top_left.x.max(other.top_left.x);
        self.top_left.y = self.top_left.y.min(other.top_left.y);
        self.top_right.x = self.top_right.x.min(other.top_right.x);
        self.top_right.y = self.top_right.y.min(other.top_right.y);
        self.bottom_right.x = self.bottom_right.x.min(other.bottom_right.x);
        self.bottom_right.y = self.bottom_right.y.max(other.bottom_right.y);
        self.bottom_left.x = self.bottom_left.x.max(other.bottom_left.x);
        self.bottom_left.y = self.bottom_left.y.max(other.bottom_left.y);

        self
    }

    /// A n B
    ///
    /// Creates a smaller [`BoundingBox`] from the intersection of the two.
    pub fn intersect(mut self, other: BoundingBox<Straight>) -> BoundingBox<Straight> {
        self.top_left.x = self.top_left.x.min(other.top_left.x);
        self.top_left.y = self.top_left.y.max(other.top_left.y);
        self.top_right.x = self.top_right.x.max(other.top_right.x);
        self.top_right.y = self.top_right.y.max(other.top_right.y);
        self.bottom_right.x = self.bottom_right.x.max(other.bottom_right.x);
        self.bottom_right.y = self.bottom_right.y.min(other.bottom_right.y);
        self.bottom_left.x = self.bottom_left.x.min(other.bottom_left.x);
        self.bottom_left.y = self.bottom_left.y.min(other.bottom_left.y);

        self
    }
}

/// Traits that defined whether a [`Shape`] can be bound by a [`BoundingBox`]
pub trait ShapeBoundingBox: ShapeOp {
    /// [`BoundingBox`] of a [`Shape`]
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>>;
    /// Absolute [`BoundingBox`] from a transform
    fn global_bounding_box(
        &self,
        parent_transform: &Transform2<f32>,
    ) -> Option<BoundingBox<UnParticular>> {
        self.local_bounding_box()
            .map(|v| v.transform(&self.global_transform(parent_transform)))
    }
}

/// Building block of a dessin
///
/// Every complex shape should boil down to these.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// A group of [`Shape`], locally positionned by a transform
    Group {
        /// Transform of the whole group
        local_transform: Transform2<f32>,
        /// List of shapes
        shapes: Vec<Shape>,
    },
    /// Block of style
    Style {
        /// Fill
        fill: Option<crate::style::Fill>,
        /// Stroke
        stroke: Option<crate::style::Stroke>,
        /// Styled shape. (Or Shapes if it is a [`Groupe`][Shape::Group])
        shape: Box<Shape>,
    },
    /// Ellipse
    Ellipse(Ellipse),
    /// Image
    Image(Image),
    /// Text
    Text(Text),
    /// Curve
    Curve(Curve),
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Group {
            local_transform: Transform2::default(),
            shapes: vec![],
        }
    }
}

impl ShapeOp for Shape {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        match self {
            Shape::Group {
                local_transform, ..
            } => {
                *local_transform = transform_matrix * *local_transform;
            }
            Shape::Style { shape, .. } => {
                shape.transform(transform_matrix);
            }
            Shape::Ellipse(v) => {
                v.transform(transform_matrix);
            }
            Shape::Image(v) => {
                v.transform(transform_matrix);
            }
            Shape::Text(v) => {
                v.transform(transform_matrix);
            }
            Shape::Curve(v) => {
                v.transform(transform_matrix);
            }
        };

        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        match self {
            Shape::Group {
                local_transform, ..
            } => local_transform,
            Shape::Style { shape, .. } => shape.local_transform(),
            Shape::Ellipse(v) => v.local_transform(),
            Shape::Image(v) => v.local_transform(),
            Shape::Text(v) => v.local_transform(),
            Shape::Curve(v) => v.local_transform(),
        }
    }
}

impl ShapeBoundingBox for Shape {
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>> {
        match self {
            Shape::Group {
                local_transform,
                shapes,
            } => shapes
                .iter()
                .filter_map(|v| v.global_bounding_box(local_transform))
                .map(|v| v.straigthen())
                .reduce(|acc, curr| BoundingBox::intersect(acc, curr))
                .map(|v| v.transform(local_transform)),
            Shape::Style { shape, .. } => shape.local_bounding_box(),
            Shape::Ellipse(e) => e.local_bounding_box(),
            Shape::Image(i) => i.local_bounding_box(),
            Shape::Text(t) => t.local_bounding_box(),
            Shape::Curve(c) => c.local_bounding_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use nalgebra::{Point2, Rotation2, Transform2};
    use std::f32::consts::FRAC_PI_2;

    const EPS: f32 = 10e-6;

    #[test]
    fn parent_rotate_child_scale() {
        let base = dessin!(Image: (
            scale={[2., 4.]}
            translate={[1., 2.]}
        ));

        let base_position = base.position(&Transform2::default());
        assert!(
            (base_position.bottom_left - Point2::new(0., 0.)).magnitude() < EPS,
            "left = {}, right = [0., 0.]",
            base_position.bottom_left,
        );
        assert!(
            (base_position.top_left - Point2::new(0., 4.)).magnitude() < EPS,
            "left = {}, right = [0., 4.]",
            base_position.top_left,
        );
        assert!(
            (base_position.top_right - Point2::new(2., 4.)).magnitude() < EPS,
            "left = {}, right = [2., 4.]",
            base_position.top_right,
        );

        let transform = nalgebra::convert(Rotation2::new(FRAC_PI_2));
        let transform_position = base.position(&transform);
        assert!(
            (transform_position.bottom_left - Point2::new(0., 0.)).magnitude() < EPS,
            "left = {}, right = [0., 0.]",
            transform_position.bottom_left,
        );
        assert!(
            (transform_position.top_left - Point2::new(-4., 0.)).magnitude() < EPS,
            "left = {}, right = [-4., 0.]",
            transform_position.top_left,
        );
        assert!(
            (transform_position.top_right - Point2::new(-4., 2.)).magnitude() < EPS,
            "left = {}, right = [-4., 2.]",
            transform_position.top_right,
        );
    }
}
