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
//! assert_eq!(svg, r#"<svg viewBox="-25 -25 50 50" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><text x="10" y="-10" text-anchor="start" font-family="Arial" font-size="16" font-weight="bold" fill='rgba(255,0,0,1)' >Hello, world!</text></svg>"#);
//! ```

use dessin::{
    shapes::{Ellipse, EllipsePosition},
    Fill, Shape, ShapeOp, Stroke,
};
use nalgebra::Transform2;
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub enum SvgError {
    WriteError(io::Error),
}
impl From<io::Error> for SvgError {
    fn from(e: io::Error) -> Self {
        SvgError::WriteError(e)
    }
}

pub trait ToSVG {
    fn write_raw_svg<W: Write>(
        &self,
        w: &mut W,
        transform: &Transform2<f32>,
    ) -> Result<(), SvgError>;

    fn write_svg<W: Write>(&self, w: &mut W) -> Result<(), SvgError> {
        write!(
            w,
            r#"<svg viewBox="{offset_x} {offset_y} {max_x} {max_y}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
            offset_x = -100.,
            offset_y = -100.,
            max_x = 200.,
            max_y = 200.,
        )?;

        self.write_raw_svg(w, &Transform2::default())?;

        write!(w, "</svg>")?;

        Ok(())
    }

    fn to_svg(&self) -> Result<String, SvgError> {
        let mut res = Cursor::new(vec![]);
        self.write_svg(&mut res)?;
        Ok(unsafe { String::from_utf8_unchecked(res.into_inner()) })
    }
}

impl<T: ToSVG> ToSVG for [T] {
    fn write_raw_svg<W: Write>(
        &self,
        w: &mut W,
        transform: &Transform2<f32>,
    ) -> Result<(), SvgError> {
        for v in self {
            v.write_raw_svg(w, transform)?
        }

        Ok(())
    }
}

impl ToSVG for Shape {
    fn write_raw_svg<W: Write>(
        &self,
        w: &mut W,
        transform: &Transform2<f32>,
    ) -> Result<(), SvgError> {
        match self {
            Shape::Group {
                local_transform,
                shapes,
            } => {
                let transform = transform * local_transform;
                shapes.write_raw_svg(w, &transform)?;
            }
            Shape::Style {
                fill,
                stroke,
                shape,
            } => {
                fn write_stroke<W: Write>(w: &mut W, stroke: &Stroke) -> io::Result<()> {
                    match stroke {
                        Stroke::Dashed {
                            color,
                            width,
                            on,
                            off,
                        } => write!(
                            w,
                            "stroke='{color}' stroke-width='{width}' stroke-dasharray='{on},{off}'"
                        ),
                        Stroke::Full { color, width } => {
                            write!(w, "stroke='{color}' stroke-width='{width}'")
                        }
                    }
                }

                fn write_fill<W: Write>(w: &mut W, fill: &Option<Fill>) -> io::Result<()> {
                    match fill {
                        Some(Fill::Color(color)) => write!(w, "fill='{color}'"),
                        None => write!(w, "fill='none'"),
                    }
                }

                write!(w, "<g ")?;
                write_fill(w, fill)?;
                if let Some(stroke) = stroke {
                    write!(w, " ")?;
                    write_stroke(w, stroke)?;
                }
                write!(w, ">")?;
                shape.write_raw_svg(w, transform)?;
                write!(w, "</g>")?;
            }
            Shape::Ellipse(e) => {
                let EllipsePosition {
                    center,
                    semi_major_axis,
                    semi_minor_axis,
                    rotation,
                } = e.position(transform);
                write!(
                    w,
                    r#"<ellipse cx="{cx}" cy="{cy}" rx="{semi_major_axis}" ry="{semi_minor_axis}" transform="rotate({rot}rad)"/>"#,
                    cx = center.x,
                    cy = center.y,
                    rot = -rotation.to_degrees()
                )?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}

// impl ToSVG for Drawing {
//     fn to_svg(&self) -> Result<String, Box<dyn Error>> {
//         let offset = -self.canvas_size() / 2.;
//         Ok(format!(
//             r#"<svg viewBox="{offset_x} {offset_y} {max_x} {max_y}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">{}</svg>"#,
//             self.shapes().to_svg()?,
//             // self.shapes()[0],
//             offset_x = offset.x,
//             offset_y = offset.y,
//             max_x = self.canvas_size().x,
//             max_y = self.canvas_size().y,
//         ))
//     }
// }

// impl<T: ToSVG> ToSVG for Vec<T> {
//     fn to_svg(&self) -> Result<String, Box<dyn Error>> {
//         self.iter()
//             .map(|v| v.to_svg())
//             .collect::<Result<String, Box<dyn Error>>>()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MyStruct;
//     impl ToSVG for MyStruct {
//         fn to_svg(&self) -> Result<String, Box<dyn Error>> {
//             Ok("MyStruct".to_owned())
//         }
//     }

//     #[test]
//     fn test_vec() {
//         let mut v = Vec::<MyStruct>::new();
//         v.push(MyStruct);
//         assert_eq!(v.to_svg().unwrap(), "MyStruct".to_owned());
//         v.push(MyStruct);
//         assert_eq!(v.to_svg().unwrap(), "MyStructMyStruct".to_owned());
//     }
// }
