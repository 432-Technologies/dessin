mod contrib;
mod macros;
mod shapes;
mod style;

pub mod prelude {
    pub use crate::dessin;
    pub use crate::shapes::*;
    pub use crate::style::*;
}

pub use shapes::{Shape, ShapeOp, ShapeOpWith};
pub use style::*;
