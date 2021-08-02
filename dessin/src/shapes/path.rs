use algebr::Vec2;

use crate::{position::Rect, style::Style, Shape, ShapeType};

pub struct Bezier(Vec2);

#[derive(Debug, Clone, PartialEq)]
pub enum Keypoint {
    Point(Vec2),
    Bezier(Vec2),
}

impl Into<Keypoints> for Vec2 {
    fn into(self) -> Keypoints {
        Keypoints(vec![Keypoint::Point(self)])
    }
}

impl Into<Keypoints> for Bezier {
    fn into(self) -> Keypoints {
        Keypoints(vec![Keypoint::Bezier(self.0)])
    }
}

pub struct Keypoints(pub Vec<Keypoint>);

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
    pub(crate) path: Vec<Keypoint>,
    pub(crate) closed: bool,
}
impl Path {
    pub fn new() -> Path {
        Path {
            pos: Rect::new(),
            style: None,
            path: vec![],
            closed: false,
        }
    }

    pub fn from(start: Vec2) -> Path {
        Path {
            pos: Rect::new().at(start),
            style: None,
            path: vec![Keypoint::Point(start)],
            closed: false,
        }
    }

    pub fn then<T>(mut self, keypoints: T) -> Self
    where
        T: Into<Keypoints>,
    {
        self.path.extend(keypoints.into().0);
        self.pos = self
            .path
            .iter()
            .map(|v| match v {
                Keypoint::Point(p) | Keypoint::Bezier(p) => p,
            })
            .fold(Rect::new(), |acc, &curr| acc.union(Rect::new().at(curr)));
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
