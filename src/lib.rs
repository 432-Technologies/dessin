mod drawing;
mod position;
mod shapes;
pub mod style;

pub type Size = Vec2;

pub use crate::drawing::{AddShape, Drawing};
pub use crate::position::Rect;
pub use crate::shapes::{Shape, ShapeType};
pub use algebra::{vec2, Angle, Vec2};

pub mod shape {
    pub use crate::shapes::arc::Arc;
    pub use crate::shapes::circle::Circle;
    pub use crate::shapes::image::{Image, ImageFormat};
    pub use crate::shapes::line::{Line, LineBuilder};
    pub use crate::shapes::text::Text;
    pub use crate::style::{Color, Fill, Stroke, Style};
}
