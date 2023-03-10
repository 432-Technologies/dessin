use image::DynamicImage;
use nalgebra::Transform2;

use crate::{Shape, ShapeOp};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Image {
    pub image: DynamicImage,
    pub transform: Transform2<f32>,
}
impl Image {
    #[inline]
    pub fn image(&mut self, image: DynamicImage) -> &mut Self {
        self.image = image;
        self
    }
    #[inline]
    pub fn with_image(mut self, image: DynamicImage) -> Self {
        self.image(image);
        self
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
        self.transform *= transform_matrix;
        self
    }
}
