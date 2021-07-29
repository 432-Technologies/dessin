use crate::{position::Rect, style::Style, Drawing, Shape, ShapeType};
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
        self.shapes
            .iter_mut()
            .for_each(|v| v.update_pos(pos + v.pos.pos));
        self.pos.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        let self_size = self.pos.size();
        if size.x / size.y != self_size.x / self_size.y {
            unimplemented!();
        }

        let scale = size.x / self_size.x;

        let curr_pos = self.pos.pos;

        self.shapes
            .iter_mut()
            .for_each(|v| v.update_pos(v.pos.pos - curr_pos));
        self.shapes.iter_mut().for_each(|v| v.update_scale(scale));
        self.shapes
            .iter_mut()
            .for_each(|v| v.update_pos(v.pos.pos + curr_pos));
        self.pos.size = Some(size);
        self
    }
}

impl Into<Shape> for EmbeddedDrawing {
    fn into(self) -> Shape {
        Shape {
            pos: self.pos,
            style: self.style,
            shape_type: ShapeType::Drawing(self.shapes),
        }
    }
}

#[cfg(test)]
mod tests {
    use algebr::vec2;

    use super::*;
    use crate::{
        shape::{Color, Stroke},
        shapes::{
            circle::Circle,
            image::{Image, ImageFormat},
            line::Line,
            ShapeType,
        },
        Drawing,
    };

    #[test]
    fn scale() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Image::new(ImageFormat::PNG(vec![]))
                .at(vec2(40., 60.))
                .with_size(vec2(20., 10.)),
        );

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(EmbeddedDrawing::new(drawing).with_size(vec2(50., 50.)));

        let drawing = &parent.shapes()[0];
        if let ShapeType::Drawing(shapes) = &drawing.shape_type {
            assert_eq!(
                parent.shapes()[0].pos,
                Rect {
                    pos: vec2(0., 0.),
                    size: Some(vec2(50., 50.)),
                    anchor: Vec2::zero(),
                }
            );

            let image = &shapes[0];
            if let ShapeType::Image { data: _ } = image.shape_type {
                assert_eq!(
                    image.pos,
                    Rect {
                        pos: vec2(20., 30.),
                        anchor: Vec2::zero(),
                        size: Some(vec2(10., 5.)),
                    }
                );
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn scale_line() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(Line::from(vec2(10., 10.)).to(vec2(10., -10.)));

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(EmbeddedDrawing::new(drawing).with_size(vec2(50., 50.)));

        let drawing = &parent.shapes()[0];
        if let ShapeType::Drawing(shapes) = &drawing.shape_type {
            assert_eq!(
                parent.shapes()[0].pos,
                Rect {
                    pos: vec2(0., 0.),
                    size: Some(vec2(50., 50.)),
                    anchor: Vec2::zero(),
                }
            );

            let image = &shapes[0];
            if let ShapeType::Line { from, to } = image.shape_type {
                assert_eq!(
                    image.pos,
                    Rect {
                        pos: vec2(5., 0.),
                        anchor: Vec2::zero(),
                        size: Some(vec2(0., 10.)),
                    }
                );
                assert_eq!(from, vec2(5., 5.));
                assert_eq!(to, vec2(5., -5.));
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn pos_line() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(Line::from(vec2(10., 10.)).to(vec2(10., -10.)));

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(EmbeddedDrawing::new(drawing).at(vec2(50., 50.)));

        let drawing = &parent.shapes()[0];
        if let ShapeType::Drawing(shapes) = &drawing.shape_type {
            assert_eq!(
                parent.shapes()[0].pos,
                Rect {
                    pos: vec2(50., 50.),
                    size: Some(vec2(100., 100.)),
                    anchor: Vec2::zero(),
                }
            );

            let image = &shapes[0];
            if let ShapeType::Line { from, to } = image.shape_type {
                assert_eq!(
                    image.pos,
                    Rect {
                        pos: vec2(60., 50.),
                        anchor: Vec2::zero(),
                        size: Some(vec2(0., 20.)),
                    }
                );
                assert_eq!(from, vec2(10., 10.) + 50.);
                assert_eq!(to, vec2(10., -10.) + 50.);
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn pos() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Image::new(ImageFormat::PNG(vec![]))
                .at(vec2(40., 60.))
                .with_size(vec2(20., 10.)),
        );

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(
            EmbeddedDrawing::new(drawing)
                //
                .at(vec2(20., 20.)),
        );

        let drawing = &parent.shapes()[0];
        if let ShapeType::Drawing(shapes) = &drawing.shape_type {
            assert_eq!(
                drawing.pos,
                Rect {
                    pos: vec2(20., 20.),
                    size: Some(vec2(100., 100.)),
                    anchor: Vec2::zero(),
                }
            );

            let image = &shapes[0];
            if let ShapeType::Image { data: _ } = image.shape_type {
                assert_eq!(
                    image.pos,
                    Rect {
                        pos: vec2(60., 80.),
                        anchor: Vec2::zero(),
                        size: Some(vec2(20., 10.)),
                    }
                );
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn both_way_simple() {
        let a = Image::new(ImageFormat::PNG(vec![]))
            .at(vec2(40., 60.))
            .with_size(vec2(20., 10.));

        let b = Image::new(ImageFormat::PNG(vec![]))
            .with_size(vec2(20., 10.))
            .at(vec2(40., 60.));

        assert_eq!(a.pos, b.pos);
    }

    #[test]
    fn both_way_drawing() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing
            .add(
                Image::new(ImageFormat::PNG(vec![]))
                    .at(vec2(40., 60.))
                    .with_size(vec2(20., 10.)),
            )
            .add(
                Image::new(ImageFormat::PNG(vec![]))
                    .with_size(vec2(20., 10.))
                    .at(vec2(40., 60.)),
            );

        let a = &drawing.shapes()[0];
        let b = &drawing.shapes()[1];

        assert_eq!(a.pos, b.pos);
    }

    #[test]
    fn both_way_embedded() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing
            .add(
                Image::new(ImageFormat::PNG(vec![]))
                    .at(vec2(40., 60.))
                    .with_size(vec2(20., 10.)),
            )
            .add(
                Image::new(ImageFormat::PNG(vec![]))
                    .with_size(vec2(20., 10.))
                    .at(vec2(40., 60.)),
            );

        let mut parent1 = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent1.add(
            EmbeddedDrawing::new(drawing.clone())
                .at(vec2(20., 20.))
                .with_size(vec2(50., 50.)),
        );

        let mut parent2 = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent2.add(
            EmbeddedDrawing::new(drawing)
                .with_size(vec2(50., 50.))
                .at(vec2(20., 20.)),
        );

        let drawing1 = &parent1.shapes()[0];
        let drawing2 = &parent2.shapes()[0];
        if let (ShapeType::Drawing(shapes1), ShapeType::Drawing(shapes2)) =
            (&drawing1.shape_type, &drawing2.shape_type)
        {
            let a = &shapes1[0];
            let b = &shapes1[1];
            let c = &shapes2[0];
            let d = &shapes2[1];

            assert_eq!(a.pos, b.pos);
            assert_eq!(c.pos, d.pos);
            assert_eq!(a.pos, c.pos);
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn image() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Image::new(ImageFormat::PNG(vec![]))
                .at(vec2(40., 60.))
                .with_size(vec2(20., 10.)),
        );

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(
            EmbeddedDrawing::new(drawing)
                .at(vec2(10., 10.))
                .with_size(vec2(50., 50.)),
        );

        let drawing = &parent.shapes()[0];
        if let ShapeType::Drawing(shapes) = &drawing.shape_type {
            assert_eq!(
                parent.shapes()[0].pos,
                Rect {
                    pos: vec2(10., 10.),
                    size: Some(vec2(50., 50.)),
                    anchor: Vec2::zero(),
                }
            );

            let image = &shapes[0];
            if let ShapeType::Image { data: _ } = image.shape_type {
                assert_eq!(
                    image.pos,
                    Rect {
                        pos: vec2(30., 40.),
                        anchor: Vec2::zero(),
                        size: Some(vec2(10., 5.)),
                    }
                );
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn test_embedded_drawing_line_with_style() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Line::from(vec2(0., 0.))
                .to(vec2(100., 100.))
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000FF),
                    width: 4.,
                    on: 2.,
                    off: 6.,
                }),
        );

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(
            EmbeddedDrawing::new(drawing)
                .with_size(vec2(50., 50.))
                .at(vec2(50., 50.)),
        );

        assert_eq!(
            parent.shapes()[0].pos,
            Rect {
                pos: vec2(50., 50.),
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
                    size: Some(vec2(50., 50.)),
                }
            );
            assert_eq!(
                shapes[0].style,
                Some(Style {
                    stroke: Some(Stroke::Dashed {
                        color: Color::U32(0xFF0000FF),
                        width: 2.,
                        on: 1.,
                        off: 3.,
                    }),
                    ..Default::default()
                })
            );

            if let ShapeType::Line { from, to } = &shapes[0].shape_type {
                assert_eq!(*from, vec2(50., 50.));
                assert_eq!(*to, vec2(100., 100.));
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn test_embedded_drawing_line2() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(Line::from(vec2(30., 20.)).to(vec2(50., 50.)));
        if let ShapeType::Line { from, to } = &drawing.shapes()[0].shape_type {
            assert_eq!(
                drawing.shapes()[0].pos,
                Rect {
                    pos: vec2(40., 35.),
                    anchor: Vec2::zero(),
                    size: Some(vec2(20., 30.)),
                }
            );
            assert_eq!(*from, vec2(30., 20.));
            assert_eq!(*to, vec2(50., 50.));
        } else {
            panic!("Wrong shape type");
        }

        let mut parent = Drawing::empty().with_canvas_size(vec2(100., 100.));
        parent.add(
            EmbeddedDrawing::new(drawing)
                .at(vec2(40., 40.))
                .with_size(vec2(20., 20.)),
        );

        assert_eq!(
            parent.shapes()[0].pos,
            Rect {
                pos: vec2(40., 40.),
                size: Some(vec2(20., 20.)),
                anchor: Vec2::zero(),
            }
        );

        if let ShapeType::Drawing(shapes) = &parent.shapes()[0].shape_type {
            assert_eq!(
                shapes[0].pos,
                Rect {
                    pos: vec2(40., 35.) / 5. + 40.,
                    anchor: Vec2::zero(),
                    size: Some(vec2(20., 30.) / 5.),
                }
            );

            if let ShapeType::Line { from, to } = &shapes[0].shape_type {
                assert_eq!(*from, vec2(30., 20.) / 5. + 40.);
                assert_eq!(*to, vec2(50., 50.) / 5. + 40.);
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }

    #[test]
    fn test_embedded_drawing_circle_with_style() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
        drawing.add(
            Circle::new()
                .at(vec2(50., 50.))
                .with_radius(10.)
                .with_stroke(Stroke::Dashed {
                    color: Color::U32(0xFF0000FF),
                    width: 4.,
                    on: 2.,
                    off: 6.,
                }),
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
                    pos: vec2(100., 100.),
                    anchor: Vec2::zero(),
                    size: Some(vec2(10., 10.)),
                }
            );
            assert_eq!(
                shapes[0].style,
                Some(Style {
                    stroke: Some(Stroke::Dashed {
                        color: Color::U32(0xFF0000FF),
                        width: 2.,
                        on: 1.,
                        off: 3.,
                    }),
                    ..Default::default()
                })
            );

            if let ShapeType::Circle { radius } = &shapes[0].shape_type {
                assert_eq!(*radius, 5.);
            } else {
                panic!("Wrong shape type");
            }
        } else {
            panic!("Wrong shape type");
        }
    }
}
