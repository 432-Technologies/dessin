use crate::{position::Rect, style::Style, Shape, ShapeType};

#[derive(Debug, Clone)]
pub enum ImageFormat {
    PNG(Vec<u8>),
    JPEG(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Image {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
    pub(crate) data: ImageFormat,
}
crate::impl_pos!(Image);
crate::impl_style!(Image);
impl Image {
    /// Create a new image from a raw data.
    pub fn new(data: ImageFormat) -> Image {
        Image {
            pos: Rect::new(),
            style: None,
            data,
        }
    }
}

impl Into<Shape> for Image {
    fn into(self) -> Shape {
        Shape {
            pos: self.pos,
            style: self.style,
            shape_type: ShapeType::Image { data: self.data },
        }
    }
}
