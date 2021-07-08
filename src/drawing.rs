use crate::shapes::{text::Text, Shape};

pub trait Add<T> {
    fn add(&mut self, shape: T);
}
impl Add<Text> for Drawing {
    fn add(&mut self, shape: Text) {
        self.texts.push(shape);
    }
}
impl Add<Box<dyn Shape>> for Drawing {
    fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
}

pub struct Drawing {
    texts: Vec<Text>,
    shapes: Vec<Box<dyn Shape>>,
}
