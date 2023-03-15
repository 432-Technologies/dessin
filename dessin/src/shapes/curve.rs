mod keypoint;

use crate::shapes::{Shape, ShapeOp};
pub use keypoint::*;
use nalgebra::Transform2;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Curve {
    pub local_transform: Transform2<f32>,
    pub keypoints: Vec<Keypoint>,
    pub closed: bool,
}
impl Curve {
    pub fn then<K: Into<Keypoint>>(&mut self, keypoint: K) -> &mut Self {
        self.keypoints.push(keypoint.into());
        self
    }
    #[inline]
    pub fn with_then<K: Into<Keypoint>>(mut self, keypoint: K) -> Self {
        self.then(keypoint);
        self
    }

    #[inline]
    pub fn close(&mut self, is_closed: bool) -> &mut Self {
        self.closed = is_closed;
        self
    }
    #[inline]
    pub fn with_close(mut self, is_closed: bool) -> Self {
        self.close(is_closed);
        self
    }

    #[inline]
    pub fn closed(&mut self) -> &mut Self {
        self.close(true)
    }
    #[inline]
    pub fn opened(&mut self) -> &mut Self {
        self.close(false)
    }
}

impl ShapeOp for Curve {
    fn transform(&mut self, transform_matrix: nalgebra::Transform2<f32>) -> &mut Self {
        self.local_transform *= transform_matrix;
        self
    }

    #[inline]
    fn local_transform(&self) -> &nalgebra::Transform2<f32> {
        &self.local_transform
    }
}

impl From<Curve> for Shape {
    fn from(v: Curve) -> Self {
        todo!()
    }
}
