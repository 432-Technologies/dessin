use crate::{position::Rect, style::Style};

#[derive(Debug, Clone)]
pub enum ImageFormat {
    PNG(Vec<u8>),
    JPEG(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Image {
    pub pos: Rect,
    pub style: Option<Style>,
    pub data: ImageFormat,
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
