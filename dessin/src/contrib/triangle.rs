use crate::prelude::*;
use nalgebra::{Point2, Transform2};

// create a struct which will be composed by 3 vectors (3 points of the vertex of the triangle)
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Triangle_test {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,

    ///size of the side following the x axis
    pub width_x_axis : f32,

    ///size of the side following the angle axis
    pub size_axis_angle : f32,

    ///angle between the 2 side created before
    pub angle : f32,
}


impl From<Triangle_test> for Curve {
    fn from(Triangle_test { local_transform ,width_x_axis, size_axis_angle, angle}: Triangle_test) -> Self {
        let origin = Point2::new(0., 0.);
        let top = Point2::new(size_axis_angle*angle.cos(), size_axis_angle*angle.sin());
        let base = Point2::new(width_x_axis, 0.);

        dessin! {
            Curve: (
                transform={local_transform}
                then={origin}
                then={top}
                then={base}
                closed
            )
        }
    }
}

impl From<Triangle_test> for Shape {
    fn from(v: Triangle_test) -> Self {
        Curve::from(v).into()
    }
}