use algebr::Vec2;

use crate::{shapes::path::Keypoint, style::Style, Rect, Shape, ShapeType};

use super::path::Keypoints;

pub type Line = LineBuilder<true>;

#[derive(Debug, Clone)]
pub struct LineBuilder<const IS_INIT: bool> {
    pub(crate) from: Vec2,
    pub(crate) to: Vec2,
    pub(crate) style: Option<Style>,
}
crate::impl_style!(LineBuilder<true>);
impl<const IS_INIT: bool> LineBuilder<IS_INIT> {
    pub const fn from(from: Vec2) -> LineBuilder<false> {
        LineBuilder {
            from,
            to: from,
            style: None,
        }
    }
}

impl LineBuilder<false> {
    pub const fn to(self, to: Vec2) -> LineBuilder<true> {
        LineBuilder {
            from: self.from,
            to,
            style: self.style,
        }
    }
}

impl Into<Shape> for LineBuilder<true> {
    fn into(self) -> Shape {
        let pos = Rect::new()
            .at((self.from + self.to) / 2.)
            .with_size((self.from - self.to).abs());

        Shape {
            pos,
            style: self.style,
            shape_type: ShapeType::Line {
                from: self.from,
                to: self.to,
            },
        }
    }
}

impl Into<Keypoints> for LineBuilder<true> {
    fn into(self) -> Keypoints {
        Keypoints(vec![Keypoint::Point(self.from), Keypoint::Point(self.to)])
    }
}
