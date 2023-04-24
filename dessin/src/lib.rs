#![warn(missing_docs)]

//! **dessin is library aimed at building complex drawings, combine them, move them and export them as PDF or SVG.**
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

pub mod contrib;
pub mod export;
mod macros;
pub mod shapes;
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
