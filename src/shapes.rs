pub mod arc;
pub mod circle;
pub mod drawing;
pub mod image;
pub mod line;
pub mod text;

use algebra::{Angle, Vec2};

use crate::{position::Rect, style::Style};

use self::{
    image::ImageFormat,
    text::{FontWeight, TextAlign},
};

/// Base shape.
#[derive(Debug, Clone)]
pub struct Shape {
    pub pos: Rect,
    pub style: Option<Style>,
    pub shape_type: ShapeType,
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Drawing(Vec<Shape>),
    Text {
        text: String,
        align: TextAlign,
        font_size: f32,
        font_weight: FontWeight,
    },
    Line {
        from: Vec2,
        to: Vec2,
    },
    Circle {
        radius: f32,
    },
    Arc {
        inner_radius: f32,
        outer_radius: f32,
        start_angle: Angle,
        end_angle: Angle,
    },
    Image {
        data: ImageFormat,
    },
}
