//! Export a [`Drawing`] in [`SVG`].
//!
//! [`Drawing`]: https://github.com/daedalus-aero-space/drawing
//! [`SVG`]: https://www.w3.org/Graphics/SVG/
//!
//! After importing [`ToSVG`][ToSVG] in scope, one can call [`ToSVG::to_svg`][ToSVG::to_svg] on any [`Drawing`][Drawing].
//!
//! See the [`Dessin`] crate for more details on how to build a drawing.
//!
//! [`Dessin`]: https://github.com/daedalus-aero-space/drawing
//! ```
//! use dessin::{shape::Text, style::{FontWeight, Fill, Color}, vec2, Drawing};
//! use dessin_svg::ToSVG;
//!
//! let mut drawing = Drawing::empty().with_canvas_size(vec2(50., 50.));
//!
//! drawing.add(
//!    Text::new("Hello, world!".to_owned())
//!        .at(vec2(10., 10.))
//!        .with_font_weight(FontWeight::Bold)
//!        .with_fill(Fill::Color(Color::U32(0xFF0000)))
//!     );
//!
//! let svg = drawing.to_svg().unwrap();
//!
//! assert_eq!(svg, r#"<svg viewBox="-25 -25 50 50"><text x="10" y="-10" text-anchor="start" font-family="Arial" font-size="16" font-weight="bold" fill='rgba(255,0,0,1)' >Hello, world!</text></svg>"#);
//! ```

mod shapes;

use dessin::Drawing;
use std::error::Error;

pub trait ToSVG {
    fn to_svg(&self) -> Result<String, Box<dyn Error>>;
}

/// Implementation of ToSVG for Drawing.
/// ```
/// # use dessin::{shape::*, style::*, *};
/// # use dessin_svg::ToSVG;
///
/// let mut drawing = Drawing::empty().with_canvas_size(vec2(50., 50.));
///
/// drawing.add(
///    Text::new("Hello, world!".to_owned())
///        .at(vec2(10., 10.))
///        .with_font_weight(FontWeight::Bold)
///        .with_fill(Fill::Color(Color::U32(0xFF0000)))
///     );
///
/// let svg = drawing.to_svg().unwrap();
///
/// assert_eq!(svg, r#"<svg viewBox="-25 -25 50 50"><text x="10" y="-10" text-anchor="start" font-family="Arial" font-size="16" font-weight="bold" fill='rgba(255,0,0,1)' >Hello, world!</text></svg>"#);
/// ```
impl ToSVG for Drawing {
    fn to_svg(&self) -> Result<String, Box<dyn Error>> {
        let offset = -self.canvas_size() / 2.;
        Ok(format!(
            r#"<svg viewBox="{offset_x} {offset_y} {max_x} {max_y}">{}</svg>"#,
            self.shapes().to_svg()?,
            // self.shapes()[0],
            offset_x = offset.x,
            offset_y = offset.y,
            max_x = self.canvas_size().x,
            max_y = self.canvas_size().y,
        ))
    }
}

impl<T: ToSVG> ToSVG for Vec<T> {
    fn to_svg(&self) -> Result<String, Box<dyn Error>> {
        self.iter()
            .map(|v| v.to_svg())
            .collect::<Result<String, Box<dyn Error>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyStruct;
    impl ToSVG for MyStruct {
        fn to_svg(&self) -> Result<String, Box<dyn Error>> {
            Ok("MyStruct".to_owned())
        }
    }

    #[test]
    fn test_vec() {
        let mut v = Vec::<MyStruct>::new();
        v.push(MyStruct);
        assert_eq!(v.to_svg().unwrap(), "MyStruct".to_owned());
        v.push(MyStruct);
        assert_eq!(v.to_svg().unwrap(), "MyStructMyStruct".to_owned());
    }
}
