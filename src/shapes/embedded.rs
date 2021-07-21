use crate::{position::Rect, style::Style, Drawing, Shape};
use algebr::Vec2;

#[derive(Debug, Clone)]
pub struct EmbeddedDrawing {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
    pub(crate) shapes: Vec<Shape>,
}
crate::impl_style!(EmbeddedDrawing);
impl EmbeddedDrawing {
    /// Creates a new [`EmbeddedDrawing`][EmbeddedDrawing] based on the given [`Drawing`][Drawing].
    pub fn new(drawing: Drawing) -> Self {
        EmbeddedDrawing {
            pos: Rect::new().with_size(drawing.canvas_size),
            style: None,
            shapes: drawing.shapes,
        }
    }

    pub fn at(mut self, pos: Vec2) -> Self {
        self.shapes.iter_mut().for_each(|v| v.update_pos(pos));
        self.pos.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        let self_size = self.pos.size.unwrap_or(Vec2::ones());
        if size.x / size.y != self_size.x / self_size.y {
            unimplemented!();
        }

        let scale = size.x / self_size.x;

        self.shapes.iter_mut().for_each(|v| v.update_scale(scale));
        self.pos.size = Some(size);
        self
    }
}

#[cfg(test)]
mod tests {
    use algebr::vec2;

    use super::*;
    use crate::{
        shapes::{
            image::{Image, ImageFormat},
            ShapeType,
        },
        AddShape, Drawing,
    };

    #[test]
    fn test_embedded_drawing() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Image::new(ImageFormat::PNG(vec![]))
                .at(vec2(50., 50.))
                .with_size(vec2(20., 10.)),
        );

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(
            EmbeddedDrawing::new(drawing)
                .at(vec2(75., 75.))
                .with_size(vec2(50., 50.)),
        );

        assert_eq!(
            parent.shapes()[0].pos,
            Rect {
                pos: vec2(75., 75.),
                size: Some(vec2(50., 50.)),
                anchor: Vec2::zero(),
            }
        );

        if let ShapeType::Drawing(shapes) = &parent.shapes()[0].shape_type {
            assert_eq!(
                shapes[0].pos,
                Rect {
                    pos: vec2(75., 75.),
                    anchor: Vec2::zero(),
                    size: Some(vec2(10., 5.)),
                }
            );
        } else {
            panic!("Wrong shape type");
        }
    }
}
