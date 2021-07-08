use crate::shapes::Shape;

#[derive(Debug)]
pub struct Drawing {
    shapes: Vec<Shape>,
}
impl Drawing {
    pub fn empty() -> Self {
        Drawing { shapes: vec![] }
    }
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }
}
