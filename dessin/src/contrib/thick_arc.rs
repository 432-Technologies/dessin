use crate::prelude::*;
use core::f32::consts::PI;
use nalgebra::{Point2, Transform2};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ThickArc {
    pub local_transform: Transform2<f32>,
    /// start angle in radian
    pub start_angle: f32,
    /// end angle in radian
    pub end_angle: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
}
impl ThickArc {
    #[inline]
    pub fn inner_radius(&mut self, inner_radius: f32) -> &mut Self {
        self.inner_radius = inner_radius;
        self
    }
    #[inline]
    pub fn with_inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius(inner_radius);
        self
    }

    #[inline]
    pub fn outer_radius(&mut self, outer_radius: f32) -> &mut Self {
        self.outer_radius = outer_radius;
        self
    }
    #[inline]
    pub fn outerinner_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius(outer_radius);
        self
    }

    #[inline]
    pub fn start_angle(&mut self, start_angle: f32) -> &mut Self {
        self.start_angle = start_angle;
        self
    }
    #[inline]
    pub fn with_start_angle(mut self, start_angle: f32) -> Self {
        self.start_angle(start_angle);
        self
    }

    pub fn span_angle(&mut self, span_angle: f32) -> &mut Self {
        self.end_angle = (self.start_angle + span_angle) % (2. * PI);
        self
    }
    pub fn with_span_angle(mut self, span_angle: f32) -> Self {
        self.end_angle((self.start_angle + span_angle) % (2. * PI));
        self
    }

    #[inline]
    pub fn end_angle(&mut self, end_angle: f32) -> &mut Self {
        self.end_angle = end_angle;
        self
    }
    #[inline]
    pub fn with_end_angle(mut self, end_angle: f32) -> Self {
        self.end_angle(end_angle);
        self
    }
}

impl From<ThickArc> for Shape {
    fn from(
        ThickArc {
            local_transform,
            start_angle,
            end_angle,
            inner_radius,
            outer_radius,
        }: ThickArc,
    ) -> Self {
        dessin!(Curve: (
            transform={local_transform}
            then={Curve::from(Arc {start_angle, end_angle, ..Default::default()}.with_radius(outer_radius))}
            then={Curve::from(Arc {start_angle, end_angle, ..Default::default()}.with_radius(inner_radius)).reversed()}
			closed
        ))
        .into()
    }
}

impl ShapeOp for ThickArc {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}

// impl Into<Shape> for ThickArc {
//     fn into(self) -> Shape {
//         let outer: Keypoints = Arc::new()
//             .at(self.pos.pos)
//             .with_anchor(self.pos.anchor)
//             .with_radius(self.outer_radius)
//             .with_start_angle(self.start_angle)
//             .with_end_angle(self.end_angle)
//             .into();

//         let inner: Keypoints = Arc::new()
//             .at(self.pos.pos)
//             .with_anchor(self.pos.anchor)
//             .with_radius(self.inner_radius)
//             .with_start_angle(self.start_angle)
//             .with_end_angle(self.end_angle)
//             .into();

//         let inner = inner.reversed();

//         let p = if let Keypoint::Point(p) = inner.0.first().unwrap() {
//             *p
//         } else {
//             unreachable!()
//         };

//         Path::new()
//             .then_do(outer)
//             .then(p)
//             .then_do(inner)
//             .close()
//             .with_maybe_style(self.style)
//             .into()
//     }
// }
