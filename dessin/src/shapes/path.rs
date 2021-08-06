use crate::{position::Rect, style::Style, Shape, ShapeType};
use algebr::{vec2, Vec2};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keypoint {
    /// Straigth line to point.
    Point(Vec2),
    /// Quadratic bezier curve.
    BezierQuad { to: Vec2, control: Vec2 },
    /// Cubic bezier curve.
    BezierCubic {
        to: Vec2,
        control_from: Vec2,
        control_to: Vec2,
    },
    // /// N points bezier curve.
    // Bezier(Vec<Vec2>),
}

impl Keypoint {
    pub fn rot_deg(&self, deg: f32) -> Self {
        self.rot_rad(deg.to_radians())
    }

    pub fn rot_rad(&self, rad: f32) -> Self {
        match self {
            Keypoint::Point(p) => Keypoint::Point(p.rot_rad(rad)),
            Keypoint::BezierQuad { to, control } => Keypoint::BezierQuad {
                to: to.rot_rad(rad),
                control: control.rot_rad(rad),
            },
            Keypoint::BezierCubic {
                to,
                control_from,
                control_to,
            } => Keypoint::BezierCubic {
                to: to.rot_rad(rad),
                control_from: control_from.rot_rad(rad),
                control_to: control_to.rot_rad(rad),
            },
            // Keypoint::Bezier(points) => {
            //     Keypoint::Bezier(points.into_iter().map(|v| v.rot_rad(rad)).collect())
            // }
        }
    }
}

impl Add<Vec2> for Keypoint {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        match self {
            Keypoint::Point(p) => Keypoint::Point(p + rhs),
            Keypoint::BezierQuad { to, control } => Keypoint::BezierQuad {
                to: to + rhs,
                control: control + rhs,
            },
            Keypoint::BezierCubic {
                to,
                control_from,
                control_to,
            } => Keypoint::BezierCubic {
                to: to + rhs,
                control_from: control_from + rhs,
                control_to: control_to + rhs,
            },
            // Keypoint::Bezier(points) => {
            //     Keypoint::Bezier(points.into_iter().map(|v| v + rhs).collect())
            // }
        }
    }
}

impl Add<f32> for Keypoint {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        self + vec2(rhs, rhs)
    }
}

impl Sub<Vec2> for Keypoint {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        match self {
            Keypoint::Point(p) => Keypoint::Point(p - rhs),
            Keypoint::BezierQuad { to, control } => Keypoint::BezierQuad {
                to: to - rhs,
                control: control - rhs,
            },
            Keypoint::BezierCubic {
                to,
                control_from,
                control_to,
            } => Keypoint::BezierCubic {
                to: to - rhs,
                control_from: control_from - rhs,
                control_to: control_to - rhs,
            },
            // Keypoint::Bezier(points) => {
            //     Keypoint::Bezier(points.into_iter().map(|v| v - rhs).collect())
            // }
        }
    }
}

impl Sub<f32> for Keypoint {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        self - vec2(rhs, rhs)
    }
}

impl Mul<Vec2> for Keypoint {
    type Output = Self;
    fn mul(self, rhs: Vec2) -> Self::Output {
        match self {
            Keypoint::Point(p) => Keypoint::Point(p * rhs),
            Keypoint::BezierQuad { to, control } => Keypoint::BezierQuad {
                to: to * rhs,
                control: control * rhs,
            },
            Keypoint::BezierCubic {
                to,
                control_from,
                control_to,
            } => Keypoint::BezierCubic {
                to: to * rhs,
                control_from: control_from * rhs,
                control_to: control_to * rhs,
            },
            // Keypoint::Bezier(points) => {
            //     Keypoint::Bezier(points.into_iter().map(|v| v * rhs).collect())
            // }
        }
    }
}

impl Mul<f32> for Keypoint {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        self * vec2(rhs, rhs)
    }
}

impl Div<f32> for Keypoint {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Keypoint::Point(p) => Keypoint::Point(p / rhs),
            Keypoint::BezierQuad { to, control } => Keypoint::BezierQuad {
                to: to / rhs,
                control: control / rhs,
            },
            Keypoint::BezierCubic {
                to,
                control_from,
                control_to,
            } => Keypoint::BezierCubic {
                to: to / rhs,
                control_from: control_from / rhs,
                control_to: control_to / rhs,
            },
            // Keypoint::Bezier(points) => {
            //     Keypoint::Bezier(points.into_iter().map(|v| v / rhs).collect())
            // }
        }
    }
}

impl AddAssign<Vec2> for Keypoint {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Keypoint {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl SubAssign<Vec2> for Keypoint {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Keypoint {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl MulAssign<Vec2> for Keypoint {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Keypoint {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Keypoint {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Into<Keypoint> for Vec2 {
    fn into(self) -> Keypoint {
        Keypoint::Point(self)
    }
}

pub struct Keypoints(pub Vec<Keypoint>);

impl Into<Keypoints> for Keypoint {
    fn into(self) -> Keypoints {
        Keypoints(vec![self])
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

    pub fn then<T>(mut self, keypoint: T) -> Self
    where
        T: Into<Keypoint>,
    {
        self.path.push(keypoint.into());
        self.update_bounding_box();
        self
    }

    pub fn then_do<T>(mut self, keypoints: T) -> Self
    where
        T: Into<Keypoints>,
    {
        self.path.extend(keypoints.into().0);
        self.update_bounding_box();
        self
    }

    fn update_bounding_box(&mut self) {
        self.pos = self
            .path
            .iter()
            .map(|v| match v {
                Keypoint::Point(p)
                | Keypoint::BezierQuad { to: p, .. }
                | Keypoint::BezierCubic { to: p, .. } => p,
                // Keypoint::Bezier(pts) => {
                //     todo!()
                // }
            })
            .fold(Rect::new(), |acc, &curr| acc.union(Rect::new().at(curr)));
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
