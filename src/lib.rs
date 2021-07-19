mod drawing;
mod position;
mod shapes;
mod style;

pub type Size = Vec2;

pub use crate::drawing::{AddShape, Drawing};
pub use algebra::{vec2, Vec2};
pub mod shape {
    pub use crate::shapes::arc::Arc;
    pub use crate::shapes::circle::Circle;
    pub use crate::shapes::image::{Image, ImageFormat};
    pub use crate::shapes::line::Line;
    pub use crate::shapes::text::Text;
}
