pub mod contrib;
mod drawing;
mod macros;
mod position;
mod shapes;
pub mod style;

pub type Size = Vec2;

pub use crate::drawing::Drawing;
pub use crate::position::Rect;
pub use crate::shapes::{Shape, ShapeType};
pub use algebr::{vec2, Angle, Vec2};

pub mod shape {
    pub use crate::shapes::circle::Circle;
    pub use crate::shapes::embedded::EmbeddedDrawing;
    pub use crate::shapes::image::{Image, ImageFormat};
    pub use crate::shapes::line::{Line, LineBuilder};
    pub use crate::shapes::path::{Keypoint, Keypoints, Path};
    pub use crate::shapes::text::Text;
    pub use crate::style::{Color, Fill, Stroke, Style};
}

pub trait ShapeGrouping {
    fn group(self) -> Drawing;
}

impl<T> ShapeGrouping for T
where
    T: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(self)
    }
}

impl<T, U> ShapeGrouping for (T, U)
where
    T: Into<Shape>,
    U: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![self.0.into(), self.1.into()])
    }
}

impl<T, U, V> ShapeGrouping for (T, U, V)
where
    T: Into<Shape>,
    U: Into<Shape>,
    V: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![self.0.into(), self.1.into(), self.2.into()])
    }
}

impl<T, U, V, W> ShapeGrouping for (T, U, V, W)
where
    T: Into<Shape>,
    U: Into<Shape>,
    V: Into<Shape>,
    W: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
        ])
    }
}

impl<T, U, V, W, X> ShapeGrouping for (T, U, V, W, X)
where
    T: Into<Shape>,
    U: Into<Shape>,
    V: Into<Shape>,
    W: Into<Shape>,
    X: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ])
    }
}

impl<T, U, V, W, X, Y> ShapeGrouping for (T, U, V, W, X, Y)
where
    T: Into<Shape>,
    U: Into<Shape>,
    V: Into<Shape>,
    W: Into<Shape>,
    X: Into<Shape>,
    Y: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ])
    }
}

impl<T, U, V, W, X, Y, Z> ShapeGrouping for (T, U, V, W, X, Y, Z)
where
    T: Into<Shape>,
    U: Into<Shape>,
    V: Into<Shape>,
    W: Into<Shape>,
    X: Into<Shape>,
    Y: Into<Shape>,
    Z: Into<Shape>,
{
    fn group(self) -> Drawing {
        Drawing::new(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ])
    }
}

pub trait ShapeInteraction<T> {
    fn children_of(self, parent: T) -> Drawing;
    fn parent_of(self, children: T) -> Drawing;
}

impl<T, U> ShapeInteraction<U> for T
where
    T: Into<Shape>,
    U: Into<Shape>,
{
    fn children_of(self, parent: U) -> Drawing {
        let mut parent = Drawing::new(parent);
        parent.add(self);
        parent
    }

    fn parent_of(self, children: U) -> Drawing {
        let mut parent = Drawing::new(self);
        parent.add(children);
        parent
    }
}

impl<T> Into<Shape> for Vec<T>
where
    T: Into<Shape>,
{
    fn into(self) -> Shape {
        let shapes: Vec<Shape> = self.into_iter().map(Into::into).collect();

        let pos = shapes.iter().fold(Rect::new(), |r, s| r.union(s.pos));

        Shape {
            pos,
            style: None,
            shape_type: ShapeType::Drawing(shapes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shape::{Circle, Text};

    #[test]
    fn children() {
        let c = Circle::new();
        let t = Text::new("".to_owned());

        let drawing = c.parent_of(t);

        let ts = vec![Text::new("".to_owned()), Text::new("".to_owned())];
        drawing.parent_of(ts);
    }
}
