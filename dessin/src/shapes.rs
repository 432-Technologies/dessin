pub mod circle;
pub mod embedded;
pub mod image;
pub mod line;
pub mod path;
pub mod text;

use algebr::{Angle, Vec2};

use crate::{position::Rect, style::Style};

use self::{
    image::ImageFormat,
    path::Keypoint,
    text::{FontWeight, TextAlign},
};

use crate::style::Stroke;

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
        radius: f32,
        start_angle: Angle,
        end_angle: Angle,
    },
    Image {
        data: ImageFormat,
    },
    Path {
        keypoints: Vec<Keypoint>,
        closed: bool,
    },
}
impl Shape {
    /// Update the position of the shape.
    pub(crate) fn update_pos(&mut self, pos: Vec2) {
        let prev_pos = self.pos.pos;
        self.pos.pos = pos;
        match &mut self.shape_type {
            ShapeType::Drawing(s) => {
                let self_pos = self.pos.pos;
                s.iter_mut().for_each(|v| {
                    v.update_pos(self_pos + pos);
                });
            }
            ShapeType::Line { from, to } => {
                let delta = pos - prev_pos;
                *from += delta;
                *to += delta;
            }
            ShapeType::Path {
                keypoints,
                closed: _,
            } => {
                keypoints.iter_mut().for_each(|v| match v {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => *p += pos,
                });
            }
            ShapeType::Circle { .. } => {}
            ShapeType::Arc { .. } => {}
            ShapeType::Image { .. } => {}
            ShapeType::Text { .. } => {}
        }
    }

    /// Update the scale of the shape.
    pub(crate) fn update_scale(&mut self, scale: f32) {
        match &mut self.shape_type {
            ShapeType::Drawing(s) => {
                let self_pos = self.pos.pos;
                s.iter_mut().for_each(|v| {
                    v.update_scale(scale);
                    v.update_pos(self_pos + v.pos.pos * scale);
                })
            }
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
                radius,
                start_angle: _,
                end_angle: _,
            } => {
                *radius *= scale;
            }
            ShapeType::Image { data: _ } => {}
            ShapeType::Path {
                keypoints,
                closed: _,
            } => {
                keypoints.iter_mut().for_each(|v| match v {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => *p *= scale,
                });
            }
        }

        self.pos.pos = self.pos.position_from_center() * scale;
        self.pos.size = self.pos.size.map(|v| v * scale);

        match self.style.as_mut().map(|v| &mut v.stroke) {
            Some(Some(Stroke::Full { color: _, width })) => {
                *width *= scale;
            }
            Some(Some(Stroke::Dashed {
                color: _,
                width,
                on,
                off,
            })) => {
                *width *= scale;
                *on *= scale;
                *off *= scale;
            }
            _ => {}
        }
    }
}
