use super::{BoundingBox, ShapeBoundingBox, UnParticular};
use crate::shapes::{Shape, ShapeOp};
use image::DynamicImage;
use nalgebra::{Point2, Scale2, Transform2, Unit, Vector2};

#[derive(Debug, Clone, PartialEq)]
pub struct ImagePosition {
    pub top_left: Point2<f32>,
    pub top_right: Point2<f32>,
    pub bottom_right: Point2<f32>,
    pub bottom_left: Point2<f32>,

    pub width: f32,
    pub height: f32,

    pub rotation: f32,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Image {
    pub image: DynamicImage,
    pub local_transform: Transform2<f32>,
}
impl Image {
    #[inline]
    pub fn image_size_pixel(&self) -> (u32, u32) {
        (self.image.width(), self.image.height())
    }

    #[inline]
    pub fn aspect_ration(&self) -> f32 {
        let (w, h) = self.image_size_pixel();
        w as f32 / h as f32
    }

    pub fn image(&mut self, image: DynamicImage) -> &mut Self {
        self.image = image;
        self
    }
    #[inline]
    pub fn with_image(mut self, image: DynamicImage) -> Self {
        self.image(image);
        self
    }

    pub fn keep_aspect_ratio(&mut self) -> &mut Self {
        self.scale(Scale2::new(self.aspect_ration(), 1.));
        self
    }
    #[inline]
    pub fn with_keep_aspect_ratio(mut self) -> Self {
        self.keep_aspect_ratio();
        self
    }

    pub fn position(&self, parent_transform: &Transform2<f32>) -> ImagePosition {
        let transform = self.global_transform(parent_transform);

        let top_left = transform * Point2::new(-0.5, 0.5);
        let top_right = transform * Point2::new(0.5, 0.5);
        let bottom_right = transform * Point2::new(0.5, -0.5);
        let bottom_left = transform * Point2::new(-0.5, -0.5);

        let rot_dir = Unit::new_normalize(transform * Vector2::x());
        let rotation = rot_dir.angle(&Vector2::x());

        ImagePosition {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            width: (top_right - top_left).magnitude(),
            height: (top_right - bottom_right).magnitude(),
            rotation,
        }
    }
}

impl From<Image> for Shape {
    #[inline]
    fn from(v: Image) -> Self {
        Shape::Image(v)
    }
}

impl ShapeOp for Image {
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

impl ShapeBoundingBox for Image {
    fn local_bounding_box(&self) -> Option<BoundingBox<UnParticular>> {
        let ImagePosition {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            width: _,
            height: _,
            rotation: _,
        } = self.position(&Transform2::default());
        Some(BoundingBox::new(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use crate::prelude::*;
    use nalgebra::{ComplexField, Point2, Rotation2, Scale2, Transform2, Translation2};

    const EPS: f32 = 0.000001;

    #[test]
    fn base() {
        let img = dessin!(Image: ());

        assert_eq!(
            img.position(&Transform2::default()),
            ImagePosition {
                top_left: Point2::new(-0.5, 0.5),
                top_right: Point2::new(0.5, 0.5),
                bottom_right: Point2::new(0.5, -0.5),
                bottom_left: Point2::new(-0.5, -0.5),
                width: 1.,
                height: 1.,
                rotation: 0.,
            }
        );
    }

    #[test]
    fn local_transform() {
        let img = dessin!(Image: (
            rotate={Rotation2::new(-45_f32.to_radians())}
        ));
        let img_pos = img.position(&Transform2::default());
        assert!(
            (img_pos.rotation - 45_f32.to_radians()).abs() < EPS,
            "left = {}, right = {}",
            img_pos.rotation,
            -45_f32.to_radians(),
        );
        assert!(
            (img_pos.width - 1.).abs() < EPS,
            "left = {}, right = {}",
            img_pos.width,
            1.,
        );
        assert!(
            (img_pos.top_left - Point2::new(0., SQRT_2 / 2.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_left,
            Point2::new(0., SQRT_2 / 2.),
        );
    }

    #[test]
    fn global_transform() {
        let img = dessin!(Image: ());
        let parent_transform = Transform2::default() * Rotation2::new(-45_f32.to_radians());
        let img_pos = img.position(&parent_transform);

        assert!(
            (img_pos.rotation - 45_f32.to_radians()).abs() < EPS,
            "left = {}, right = {}",
            img_pos.rotation,
            -45_f32.to_radians(),
        );
        assert!(
            (img_pos.width - 1.).abs() < EPS,
            "left = {}, right = {}",
            img_pos.width,
            1.,
        );
        assert!(
            (img_pos.top_left - Point2::new(0., SQRT_2 / 2.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_left,
            Point2::new(0., SQRT_2 / 2.),
        );
    }

    #[test]
    fn combined_transform() {
        let img = dessin!(Image: ());
        let img_pos = img.position(&Transform2::default());
        println!("Base = {img_pos:?}\n");
        assert_eq!(
            img_pos,
            ImagePosition {
                top_left: Point2::new(-0.5, 0.5),
                top_right: Point2::new(0.5, 0.5),
                bottom_right: Point2::new(0.5, -0.5),
                bottom_left: Point2::new(-0.5, -0.5),
                width: 1.,
                height: 1.,
                rotation: 0.,
            }
        );

        let img = dessin!(var |img|: (rotate={ Rotation2::new(-45_f32.to_radians()) }));
        let img_pos = img.position(&Transform2::default());
        println!("Rot(-45deg) = {img_pos:?}\n");
        assert!(
            (img_pos.rotation - 45_f32.to_radians()).abs() < EPS,
            "left = {}, right = {}",
            img_pos.rotation,
            -45_f32.to_radians(),
        );
        assert!(
            (img_pos.width - 1.).abs() < EPS,
            "left = {}, right = {}",
            img_pos.width,
            1.,
        );
        assert!(
            (img_pos.top_left - Point2::new(0., SQRT_2 / 2.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_left,
            Point2::new(0., SQRT_2 / 2.),
        );

        let img = dessin!(var |img|: (translate = { Translation2::new(1., 0.) }));
        let img_pos = img.position(&Transform2::default());
        println!("Translate_x(1) = {img_pos:?}\n");
        assert!(
            (img_pos.rotation - 45_f32.to_radians()).abs() < EPS,
            "left = {}, right = {}",
            img_pos.rotation,
            -45_f32.to_radians(),
        );
        assert!(
            (img_pos.width - 1.).abs() < EPS,
            "left = {}, right = {}",
            img_pos.width,
            1.,
        );
        assert!(
            (img_pos.top_left - Point2::new(1., SQRT_2 / 2.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_left,
            Point2::new(1., SQRT_2 / 2.),
        );
        assert!(
            (img_pos.top_right - Point2::new(SQRT_2 / 2. + 1., 0.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_right,
            Point2::new(SQRT_2 / 2. + 1., 0.),
        );

        let img = dessin!(var |img|: (scale = { Scale2::new(3., 2.) }));
        let img_pos = img.position(&Transform2::default());
        println!("Scale(3, 2) = {img_pos:?}\n");
        assert!(
            (img_pos.top_left - Point2::new(3. * 1., 2. * SQRT_2 / 2.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_left,
            Point2::new(3. * 1., 2. * SQRT_2 / 2.),
        );
        assert!(
            (img_pos.top_right - Point2::new(3. * (SQRT_2 / 2. + 1.), 2. * 0.)).magnitude() < EPS,
            "left = {}, right = {}",
            img_pos.top_right,
            Point2::new(3. * (SQRT_2 / 2. + 1.), 2. * 0.),
        );
    }
}
