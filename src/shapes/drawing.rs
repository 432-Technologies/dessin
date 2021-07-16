use super::Shape;
use crate::{drawing::Drawing, position::Rect};
use algebra::Vec2;

pub struct EmbeddedDrawing {
    pub(crate) shapes: Vec<Shape>,
    pub pos: Rect<false>,
}
impl EmbeddedDrawing {
    pub fn from_drawing(drawing: Drawing) -> Self {
        EmbeddedDrawing {
            shapes: drawing.shapes,
            pos: Rect::new(),
        }
    }
}
