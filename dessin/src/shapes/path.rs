use algebr::Vec2;

use crate::{position::Rect, style::Style, Shape, ShapeType};

#[derive(Debug, Clone, PartialEq)]
pub enum Keypoint {
    Point(Vec2),
    Bezier {
        destination: Vec2,
        start_prop: Vec2,
        dest_prop: Vec2,
    },
}

impl Into<Keypoint> for Vec2 {
    fn into(self) -> Keypoint {
        Keypoint::Point(self)
    }
}

impl Into<Keypoint> for (Vec2, Vec2, Vec2) {
    fn into(self) -> Keypoint {
        Keypoint::Bezier {
            destination: self.0,
            start_prop: self.1,
            dest_prop: self.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
    pub(crate) path: Vec<Keypoint>,
    pub(crate) closed: bool,
}
impl Path {
    pub fn from(start: Vec2) -> Path {
        Path {
            pos: Rect::new().at(start),
            style: None,
            path: vec![Keypoint::Point(start)],
            closed: false,
        }
    }

    pub fn then<T>(mut self, keypoint: T) -> Self
    where
        T: Into<Keypoint>,
    {
        self.path.push(keypoint.into());
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }
}

impl Into<Shape> for Path {
    fn into(self) -> Shape {
        Shape {
            pos: self.pos,
            style: self.style,
            shape_type: ShapeType::Path {
                keypoints: self.path,
                closed: self.closed,
            },
        }
    }
}
