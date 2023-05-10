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

// We need this in order for the proc_macro to work in this library.
// See https://github.com/rust-lang/rust/issues/56409 for more details
extern crate self as dessin;

/// Shapes made of basic [shapes][crate::shapes::Shape]
pub mod contrib;
/// Declarations to create an export format.
pub mod export;
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
    pub use crate::shapes::*;
    pub use crate::style::*;
    pub use ::dessin_macros::dessin;
}

/// Everything related to fonts.
pub mod font {
    pub use crate::shapes::text::font::*;
}

fn test() {
    use ::dessin::prelude::*;

    let then = dessin_macros::dessin!(Line: (
        from={[0., 0.]}
        to={[10., 10.]}
    ));

    let text = dessin_macros::dessin!(Text: (
        text={"Hi"}
    ));

    let x = 2;
    let oupsy = dessin_macros::dessin!(if x > 2 {
        Circle: ()
    });
    let oupsy2 = dessin_macros::dessin!(if x < 2 { Circle: () } else { Circle: () });

    let res = dessin_macros::dessin!([
        Circle: (
            translate={[1., 1.]}
        ),
        for x in 0..10 {
            let y = x as f32 * 2.;
            dessin_macros::dessin!(Circle: (
                radius={y}
            ))
        },
        cloned(text): (),
        var(text): (),
        Curve: #(
            {then}
            closed
        ),
    ] -> #(
        translate={[1., 1.]}
    ));
}
