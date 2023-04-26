// #![warn(missing_docs)]

//! **dessin is library aimed at building complex drawings, combine them, move them and export them as PDF or SVG.**
//!
//! ## Example
//!
//! ```
//! # fn main () {
//! use dessin::prelude::*;
//!
//! let dessin = dessin!();
//! # }
//! ```
//!
//! Details about the [`dessin`] macro.
//!
//! ## Implement own export format.
//! Documentation can be found in the [`export`] module.

/// Shapes made of basic [shapes][crate::shapes::Shape]
pub mod contrib;
/// Declarations to create an export format.
pub mod export;
mod macros;
/// Building blocks of a dessin
pub mod shapes;
/// Styling of the building blocks
pub mod style;

pub use ::image;
pub use ::nalgebra;

/// Prelude module includes everyting you need to build a dessin.
/// You can of courses cherry pick what you need by importing directly from other modules.
pub mod prelude {
    pub use crate::contrib::*;
    pub use crate::dessin;
    pub use crate::shapes::*;
    pub use crate::style::*;
}

/// Everything related to fonts.
pub mod font {
    pub use crate::shapes::text::font::*;
}
