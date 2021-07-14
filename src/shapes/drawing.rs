use super::Shape;
use crate::drawing::Drawing;
use algebra::Vec2;

pub struct EmbeddedDrawing {
    pub(crate) shapes: Vec<Shape>,
    pub pos: Vec2,
    pub canvas_anchor: Vec2,
    pub scale: f32,
}
impl EmbeddedDrawing {
    pub fn from_drawing(drawing: Drawing) -> Self {
        EmbeddedDrawing {
            shapes: drawing.shapes,
            pos: Vec2::from_cartesian(0., 0.),
            canvas_anchor: Vec2::from_cartesian(0., 0.),
            scale: 1.,
        }
    }
    pub fn with_canvas_anchor(mut self, canvas_anchor: Vec2) -> Self {
        self.canvas_anchor = canvas_anchor;
        self
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn with_pos(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }
}
