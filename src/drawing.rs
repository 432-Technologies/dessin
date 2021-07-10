use crate::shapes::Shape;
use algebra::Vec2;

pub type Size = Vec2;

#[derive(Debug)]
pub struct Drawing {
    pub canvas_size: Size,
    pub canvas_anchor: Size,
    shapes: Vec<Shape>,
}
impl Drawing {
    pub const fn empty() -> Self {
        Drawing {
            canvas_size: Vec2::from_cartesian_tuple((0., 0.)),
            canvas_anchor: Vec2::from_cartesian_tuple((0., 0.)),
            shapes: vec![],
        }
    }
    pub const fn with_canvas_size(mut self, canvas_size: (f32, f32)) -> Self {
        self.canvas_size = Vec2::from_cartesian_tuple(canvas_size);
        self
    }
    pub const fn with_canvas_anchor(mut self, canvas_anchor: (f32, f32)) -> Self {
        self.canvas_anchor = Vec2::from_cartesian_tuple(canvas_anchor);
        self
    }
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }
}
