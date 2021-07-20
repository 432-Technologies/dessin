use super::Shape;
use crate::{drawing::Drawing, position::Rect};

pub struct EmbeddedDrawing {
    pub(crate) shapes: Vec<Shape>,
    pub pos: Rect,
}
impl EmbeddedDrawing {
    pub fn from_drawing(drawing: Drawing) -> Self {
        EmbeddedDrawing {
            shapes: drawing.shapes,
            pos: Rect::new(),
        }
    }
}
