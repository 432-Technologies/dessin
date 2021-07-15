use algebra::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct ImageStyle {}
impl ImageStyle {
    pub const fn new() -> Self {
        ImageStyle {}
    }
}

#[derive(Debug, Clone)]
pub enum ImageFormat {
    PNG(Vec<u8>),
    JPEG(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Image {
    pub pos: Vec2,
    pub size: Vec2,
    pub style: ImageStyle,
    pub data: ImageFormat,
}
