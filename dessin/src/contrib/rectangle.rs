use crate::prelude::*;
use nalgebra::{Point2, Scale2, Transform2};

/// Rectangle
#[derive(Default, Debug, Clone, PartialEq, Shape)]
pub struct Rectangle {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,
}
impl Rectangle {
    /// Width (x axis)
    #[inline]
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.scale(Scale2::new(width, 1.));
        self
    }
    /// Width (x axis)
    #[inline]
    pub fn with_width(mut self, width: f32) -> Self {
        self.width(width);
        self
    }

    /// Height (y axis)
    #[inline]
    pub fn height(&mut self, height: f32) -> &mut Self {
        self.scale(Scale2::new(1., height));
        self
    }
    /// Height (y axis)
    #[inline]
    pub fn with_height(mut self, height: f32) -> Self {
        self.height(height);
        self
    }
}

impl From<Rectangle> for Curve {
    fn from(Rectangle { local_transform }: Rectangle) -> Self {
        let top_left = Point2::new(-0.5, 0.5);
        let top_right = Point2::new(0.5, 0.5);
        let bottom_right = Point2::new(0.5, -0.5);
        let bottom_left = Point2::new(-0.5, -0.5);

        dessin! {
            Curve: (
                transform={local_transform}
                then={top_left}
                then={bottom_left}
                then={bottom_right}
                then={top_right}
                closed
            )
        }
    }
}

impl From<Rectangle> for Shape {
    fn from(v: Rectangle) -> Self {
        Curve::from(v).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use nalgebra::{Point2, Rotation2, Scale2, Transform2};
    use std::f32::consts::FRAC_PI_2;

    const EPS: f32 = 10e-6;

    #[test]
    fn similar_op() {
        let base = dessin!(Rectangle: (
            width={2.}
            height={3.}
            translate={[1., 2.]}
        ));

        let base_2 = dessin!(Rectangle: (
            scale={[2., 3.]}
            translate={[1., 2.]}
        ));

        let base_3 = dessin!(Rectangle: (
            translate={[1. / 2., 2. / 3.]}
            scale={[2., 3.]}
        ));

        assert_eq!(base, base_2);
        assert_eq!(base, base_3);
    }

    #[test]
    fn parent_rotate_text_scale() {
        let base = dessin!(Rectangle: (
            width={2.}
            height={3.}
            translate={[1., 2.]}
        ));

        let base_position: Vec<Point2<f32>> = base
            .clone()
            .as_curve()
            .position(&Transform2::default())
            .keypoints
            .into_iter()
            .map(|key_point| match key_point {
                KeypointPosition::Point(point) => point,
                _ => unreachable!(),
            })
            .collect();

        assert!(
            (base_position[0] - Point2::new(0., 3.5)).magnitude() < EPS,
            "left = {}, right = [0., 3.5]",
            base_position[0],
        );
        assert!(
            (base_position[1] - Point2::new(0., 0.5)).magnitude() < EPS,
            "left = {}, right = [0., 0.5]",
            base_position[1],
        );
        assert!(
            (base_position[2] - Point2::new(2., 0.5)).magnitude() < EPS,
            "left = {}, right = [2., 0.5]",
            base_position[2],
        );
        assert!(
            (base_position[3] - Point2::new(2., 3.5)).magnitude() < EPS,
            "left = {}, right = [2., 3.5]",
            base_position[3],
        );

        let transform = nalgebra::convert(Rotation2::new(FRAC_PI_2));
        let transform_position: Vec<Point2<f32>> = base
            .clone()
            .as_curve()
            .position(&transform)
            .keypoints
            .into_iter()
            .map(|key_point| match key_point {
                KeypointPosition::Point(point) => point,
                _ => unreachable!(),
            })
            .collect();
        assert!(
            (transform_position[0] - Point2::new(-3.5, 0.)).magnitude() < EPS,
            "left = {}, right = [-3.5, 0.]",
            transform_position[0],
        );
        assert!(
            (transform_position[1] - Point2::new(-0.5, 0.)).magnitude() < EPS,
            "left = {}, right = [-0.5,0.]",
            transform_position[1],
        );
        assert!(
            (transform_position[2] - Point2::new(-0.5, 2.)).magnitude() < EPS,
            "left = {}, right = [-0.5,2.]",
            transform_position[2],
        );
        assert!(
            (transform_position[3] - Point2::new(-3.5, 2.)).magnitude() < EPS,
            "left = {}, right = [-3.5,2.]",
            transform_position[3],
        );

        let transform = nalgebra::convert::<_, Transform2<f32>>(Rotation2::new(FRAC_PI_2))
            * nalgebra::convert::<_, Transform2<f32>>(Scale2::new(2., 2.));
        let transform_position: Vec<Point2<f32>> = base
            .as_curve()
            .position(&transform)
            .keypoints
            .into_iter()
            .map(|key_point| match key_point {
                KeypointPosition::Point(point) => point,
                _ => unreachable!(),
            })
            .collect();
        assert!(
            (transform_position[0] - Point2::new(-7., 0.)).magnitude() < EPS,
            "left = {}, right = [-7., 0.]",
            transform_position[0],
        );
        assert!(
            (transform_position[1] - Point2::new(-1., 0.)).magnitude() < EPS,
            "left = {}, right = [-1., 0]",
            transform_position[1],
        );
        assert!(
            (transform_position[2] - Point2::new(-1., 4.)).magnitude() < EPS,
            "left = {}, right = [-1., 4]",
            transform_position[2],
        );
        assert!(
            (transform_position[3] - Point2::new(-7., 4.)).magnitude() < EPS,
            "left = {}, right = [-7., 4]",
            transform_position[3],
        );
    }
}
