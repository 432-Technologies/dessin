mod keypoint;

use super::{BoundingBox, ShapeBoundingBox, UnParticular};
use crate::shapes::{Shape, ShapeOp};
pub use keypoint::*;
use nalgebra::{Point2, Transform2};

#[derive(Debug, Clone, PartialEq)]
pub struct CurvePosition {
    pub keypoints: Vec<KeypointPosition>,
    pub closed: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Curve {
    pub local_transform: Transform2<f32>,
    pub keypoints: Vec<Keypoint>,
    pub closed: bool,
}
impl Curve {
    #[inline]
    pub fn extend<T: IntoIterator<Item = Keypoint>>(&mut self, shapes: T) -> &mut Self {
        self.keypoints.extend(shapes);
        self
    }

    #[inline]
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

    #[inline]
    pub fn reverse(&mut self) -> &mut Self {
        *self = self.reversed();
        self
    }

    pub fn start_point(&self) -> Option<Point2<f32>> {
        match self.keypoints.first() {
            Some(Keypoint::Point(p)) => Some(*p),
            Some(Keypoint::Bezier(b)) => b.start,
            Some(Keypoint::Curve(c)) => c.start_point(),
            None => None,
        }
    }

    pub fn reversed(&self) -> Self {
        let (c, b) = self._reversed();
        if b.is_some() {
            panic!("")
        }
        c
    }

    fn _reversed<'a>(&'a self) -> (Self, Option<&'a Bezier>) {
        let mut keypoints = Vec::with_capacity(self.keypoints.len());

        let mut tmp: Option<&'a Bezier> = None;

        for k in self.keypoints.iter().rev() {
            match (k, tmp) {
                (Keypoint::Point(p), None) => {
                    keypoints.push(Keypoint::Point(*p));
                }
                (Keypoint::Point(p), Some(t)) => {
                    keypoints.push(Keypoint::Bezier(Bezier {
                        start: Some(t.end),
                        start_control: t.end_control,
                        end_control: t.start_control,
                        end: *p,
                    }));
                    tmp = None;
                }
                (Keypoint::Bezier(b), None) => {
                    if let Some(start) = b.start {
                        keypoints.push(Keypoint::Bezier(Bezier {
                            start: Some(b.end),
                            start_control: b.end_control,
                            end_control: b.start_control,
                            end: start,
                        }));
                    } else {
                        tmp = Some(b);
                    }
                }
                (Keypoint::Bezier(b), Some(t)) => {
                    keypoints.push(Keypoint::Bezier(Bezier {
                        start: Some(t.end),
                        start_control: t.end_control,
                        end_control: t.start_control,
                        end: b.end,
                    }));
                    tmp = None;

                    if let Some(start) = b.start {
                        keypoints.push(Keypoint::Bezier(Bezier {
                            start: Some(b.end),
                            start_control: b.end_control,
                            end_control: b.start_control,
                            end: start,
                        }));
                    } else {
                        tmp = Some(b);
                    }
                }
                (Keypoint::Curve(c), None) => {
                    let (curve, rest) = c._reversed();
                    keypoints.push(Keypoint::Curve(curve));
                    tmp = rest;
                }
                (Keypoint::Curve(c), Some(t)) => {
                    let (curve, rest) = c._reversed();
                    if let Some(start) = curve.start_point() {
                        keypoints.push(Keypoint::Bezier(Bezier {
                            start: Some(t.end),
                            start_control: t.end_control,
                            end_control: t.start_control,
                            end: start,
                        }));
                        tmp = rest;
                    } else if let None = rest {
                        // Everithing's good here
                    } else {
                        panic!("")
                    }
                }
            }
        }

        (
            Curve {
                local_transform: self.local_transform,
                closed: self.closed,
                keypoints,
            },
            tmp,
        )
    }

    pub fn position(&self, parent_transform: &Transform2<f32>) -> CurvePosition {
        fn flatten_curve(
            curve: &Curve,
            parent_transform: &Transform2<f32>,
        ) -> Vec<KeypointPosition> {
            let parent_transform = curve.global_transform(parent_transform);

            let res = curve
                .keypoints
                .iter()
                .flat_map(|keypoint| match keypoint {
                    Keypoint::Point(p) => vec![KeypointPosition::Point(parent_transform * p)],
                    Keypoint::Bezier(b) => {
                        vec![KeypointPosition::Bezier(b.transform(&parent_transform))]
                    }
                    Keypoint::Curve(c) => flatten_curve(c, &parent_transform),
                })
                .collect::<Vec<_>>();

            res
        }

        CurvePosition {
            keypoints: flatten_curve(self, parent_transform),
            closed: self.closed,
        }
    }
}

impl ShapeOp for Curve {
    fn transform(&mut self, transform_matrix: nalgebra::Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &nalgebra::Transform2<f32> {
        &self.local_transform
    }
}

impl From<Curve> for Shape {
    fn from(v: Curve) -> Self {
        Shape::Curve(v)
    }
}

impl ShapeBoundingBox for Curve {
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        let bb = self
            .keypoints
            .iter()
            .map(|v| v.bounding_box().straigthen())
            .reduce(|acc, curr| acc.join(curr))
            .unwrap_or_else(|| BoundingBox::zero());

        bb.as_unparticular().transform(&self.local_transform)
    }
}

impl<T> From<T> for Keypoint
where
    T: Into<Curve>,
{
    fn from(value: T) -> Self {
        Keypoint::Curve(value.into())
    }
}

pub trait CurveOp {
    fn as_curve(&self) -> Curve;
}

impl<T> CurveOp for T
where
    T: Into<Curve> + Clone,
{
    fn as_curve(&self) -> Curve {
        self.clone().into()
    }
}
