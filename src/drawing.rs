use crate::shapes::Shape;
use algebra::Vec2;

pub type Size = Vec2;

#[derive(Debug)]
pub struct Drawing {
    canvas_size: Size,
    shapes: Vec<Shape>,
}
impl Drawing {
    pub const fn empty() -> Self {
        Drawing {
            canvas_size: Vec2::from_cartesian_tuple((0., 0.)),
            shapes: vec![],
        }
    }
    pub const fn with_canvas_size(mut self, canvas_size: (f32, f32)) -> Self {
        self.canvas_size = Vec2::from_cartesian_tuple(canvas_size);
        self
    }
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }
}
