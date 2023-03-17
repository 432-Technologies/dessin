use crate::{
    dessin,
    shapes::{Curve, Shape, ShapeOp},
};
use nalgebra::{Point2, Transform2};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub local_transform: Transform2<f32>,
    pub width: f32,
    pub height: f32,
}
impl Rectangle {
    #[inline]
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }
    #[inline]
    pub fn with_width(mut self, width: f32) -> Self {
        self.width(width);
        self
    }

    #[inline]
    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }
    #[inline]
    pub fn with_height(mut self, height: f32) -> Self {
        self.height(height);
        self
    }

    pub fn as_curve(self) -> Curve {
        use crate::prelude::*;

        let Rectangle {
            local_transform,
            width,
            height,
        } = self;

        let width = width / 2.;
        let height = height / 2.;

        let top_left = Point2::new(-width, height);
        let top_right = Point2::new(width, height);
        let bottom_right = Point2::new(width, -height);
        let bottom_left = Point2::new(-width, -height);

        dessin! {
            Curve: (
                transform={local_transform}
                then={top_left}
                then={bottom_left}
                then={bottom_right}
                then={top_right}
                closed
            )
        }
    }
}

impl ShapeOp for Rectangle {
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}

impl From<Rectangle> for Shape {
    fn from(v: Rectangle) -> Self {
        v.as_curve().into()
    }
}
// impl From<Rectangle> for Shape {
//     fn from(value: Rectangle) -> Self {
//         let min = value.pos.position_from_anchor(vec2(-1., -1.));
//         let max = value.pos.position_from_anchor(vec2(1., 1.));

//         let mut rect = Drawing::empty().with_canvas_size(value.pos.size());

//         rect.add(
//             Line::from(vec2(min.x, min.y))
//                 .to(vec2(min.x, max.y))
//                 .with_style(value.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
//         )
//         .add(
//             Line::from(vec2(min.x, min.y))
//                 .to(vec2(max.x, min.y))
//                 .with_style(value.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
//         )
//         .add(
//             Line::from(vec2(max.x, max.y))
//                 .to(vec2(min.x, max.y))
//                 .with_style(value.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
//         )
//         .add(
//             Line::from(vec2(max.x, max.y))
//                 .to(vec2(max.x, min.y))
//                 .with_style(value.style.as_ref().map(|v| v.clone()).unwrap_or_default()),
//         );

//         rect.into()
//     }
// }
