pub mod arc;
pub mod circle;
pub mod embedded;
pub mod image;
pub mod line;
pub mod text;

use algebr::{Angle, Vec2};

use crate::{position::Rect, style::Style};

use self::{
    image::ImageFormat,
    text::{FontWeight, TextAlign},
};

/// Base shape.
///
/// The [`Shape::pos`][Shape::pos] member must *at any time* reflect the bounding box of the shape.
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
impl Shape {
    /// Update the position of the shape.
    pub(crate) fn update_pos(&mut self, pos: Vec2) {
        self.pos.pos = pos;
        match &mut self.shape_type {
            ShapeType::Drawing(s) => s.iter_mut().for_each(|v| v.update_pos(pos)),
            _ => {}
        }
    }

    /// Update the scale of the shape.
    pub(crate) fn update_scale(&mut self, scale: f32) {
        match &mut self.shape_type {
            ShapeType::Drawing(s) => s.iter_mut().for_each(|v| v.update_scale(scale)),
            ShapeType::Text {
                text: _,
                align: _,
                font_size,
                font_weight: _,
            } => {
                *font_size *= scale;
            }
            ShapeType::Line { from, to } => {
                *from *= scale;
                *to *= scale;
            }
            ShapeType::Circle { radius } => {
                *radius *= scale;
            }
            ShapeType::Arc {
                inner_radius,
                outer_radius,
                start_angle: _,
                end_angle: _,
            } => {
                *inner_radius *= scale;
                *outer_radius *= scale;
            }
            ShapeType::Image { data: _ } => {}
        }

        self.pos.size = self.pos.size.map(|v| v * scale);
    }
}
