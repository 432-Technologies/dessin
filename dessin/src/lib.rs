pub mod contrib;
mod macros;
pub mod shapes;
pub mod style;

pub mod prelude {
    pub use crate::contrib::*;
    pub use crate::dessin;
    pub use crate::shapes::*;
    pub use crate::style::*;
}

pub use shapes::{Shape, ShapeOp, ShapeOpWith};
pub use style::*;
