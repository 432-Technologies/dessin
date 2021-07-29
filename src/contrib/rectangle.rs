use crate::{
    shape::{EmbeddedDrawing, Line, Style},
    vec2, Drawing, Rect, Shape,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
}
crate::impl_pos!(Rectangle);
crate::impl_style!(Rectangle);
impl Rectangle {
    pub const fn new() -> Self {
        Rectangle {
            pos: Rect::new(),
            style: None,
        }
    }
}

impl Into<Shape> for Rectangle {
    fn into(self) -> Shape {
        let min = self.pos.position_from_anchor(vec2(-1., -1.));
        let max = self.pos.position_from_anchor(vec2(1., 1.));

        let mut rect = Drawing::empty().with_canvas_size(self.pos.size());

        rect.add(
            Line::from(vec2(min.x, min.y))
                .to(vec2(min.x, max.y))
                .with_style(self.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
        )
        .add(
            Line::from(vec2(min.x, min.y))
                .to(vec2(max.x, min.y))
                .with_style(self.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
        )
        .add(
            Line::from(vec2(max.x, max.y))
                .to(vec2(min.x, max.y))
                .with_style(self.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
        )
        .add(
            Line::from(vec2(max.x, max.y))
                .to(vec2(max.x, min.y))
                .with_style(self.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
        );

        EmbeddedDrawing::new(rect).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ShapeType;

    #[test]
    fn rect() {
        let rect: Shape = Rectangle::new()
            .at(vec2(20., 20.))
            .with_size(vec2(20., 40.))
            .into();

        if let ShapeType::Drawing(shapes) = &rect.shape_type {
            if let ShapeType::Line { from, to } = &shapes[0].shape_type {
                assert_eq!(*from, vec2(10., 0.));
                assert_eq!(*to, vec2(10., 40.));
            } else {
                panic!("Expected a Line");
            }

            if let ShapeType::Line { from, to } = &shapes[1].shape_type {
                assert_eq!(*from, vec2(10., 0.));
                assert_eq!(*to, vec2(30., 0.));
            } else {
                panic!("Expected a Line");
            }

            if let ShapeType::Line { from, to } = &shapes[2].shape_type {
                assert_eq!(*from, vec2(30., 40.));
                assert_eq!(*to, vec2(10., 40.));
            } else {
                panic!("Expected a Line");
            }

            if let ShapeType::Line { from, to } = &shapes[3].shape_type {
                assert_eq!(*from, vec2(30., 40.));
                assert_eq!(*to, vec2(30., 0.));
            } else {
                panic!("Expected a Line");
            }
        } else {
            panic!("Expected a Drawing");
        }
    }
}
