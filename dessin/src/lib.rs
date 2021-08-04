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
    pub use crate::shapes::path::{Bezier, Keypoint, Keypoints, Path};
    pub use crate::shapes::text::Text;
    pub use crate::style::{Color, Fill, Stroke, Style};
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
