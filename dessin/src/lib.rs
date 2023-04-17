pub mod contrib;
mod macros;
pub mod shapes;
pub mod style;

pub use ::image;
pub use ::nalgebra;

pub mod prelude {
    pub use crate::contrib::*;
    pub use crate::dessin;
    pub use crate::shapes::*;
    pub use crate::style::*;
}

pub mod font {
    pub use crate::shapes::text::font::*;
}
