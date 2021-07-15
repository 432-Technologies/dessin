use crate::shapes::{
    arc::Arc, circle::Circle, drawing::EmbeddedDrawing, image::Image, line::Line, text::Text, Shape,
};
use algebra::Vec2;

pub type Size = Vec2;

pub trait AddShape<T> {
    fn add(&mut self, shape: T);
}

#[derive(Debug, Clone)]
pub struct Drawing {
    pub canvas_size: Size,
    pub canvas_anchor: Size,
    pub(crate) shapes: Vec<Shape>,
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
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }
    pub fn into_embedded(self) -> EmbeddedDrawing {
        EmbeddedDrawing::from_drawing(self)
    }
}

impl AddShape<Text> for Drawing {
    fn add(&mut self, shape: Text) {
        self.shapes.push(Shape::Text(shape));
    }
}
impl AddShape<Line> for Drawing {
    fn add(&mut self, shape: Line) {
        self.shapes.push(Shape::Line(shape));
    }
}
impl AddShape<Circle> for Drawing {
    fn add(&mut self, shape: Circle) {
        self.shapes.push(Shape::Circle(shape));
    }
}
impl AddShape<Arc> for Drawing {
    fn add(&mut self, shape: Arc) {
        self.shapes.push(Shape::Arc(shape));
    }
}
impl AddShape<Image> for Drawing {
    fn add(&mut self, shape: Image) {
        self.shapes.push(Shape::Image(shape));
    }
}
impl AddShape<EmbeddedDrawing> for Drawing {
    fn add(
        &mut self,
        EmbeddedDrawing {
            mut shapes,
            pos,
            canvas_anchor,
            scale,
        }: EmbeddedDrawing,
    ) {
        if canvas_anchor != Vec2::from_cartesian(0., 0.) {
            unimplemented!()
        }

        shapes
            .iter_mut()
            .for_each(|s| s.apply_transform(pos, scale));
        self.shapes.push(Shape::Drawing(shapes));
    }
}
