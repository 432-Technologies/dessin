use dessin::prelude::*;
use nalgebra::Transform2;
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub enum SvgError {
    WriteError(io::Error),
    CurveHasNoStartingPoint,
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
        parent_transform: &Transform2<f32>,
    ) -> Result<(), SvgError>;

    fn write_svg<W: Write>(&self, w: &mut W) -> Result<(), SvgError> {
        let max_x = 150.;
        let max_y = 150.;

        write!(
            w,
            r#"<svg viewBox="{offset_x} {offset_y} {max_x} {max_y}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
            offset_x = -max_x / 2.,
            offset_y = -max_y / 2.,
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

impl ToSVG for Shape {
    fn write_raw_svg<W: Write>(
        &self,
        w: &mut W,
        parent_transform: &Transform2<f32>,
    ) -> Result<(), SvgError> {
        match self {
            Shape::Group { shapes, .. } => {
                let transform = self.global_transform(parent_transform);
                for v in shapes {
                    v.write_raw_svg(w, &transform)?
                }
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
                shape.write_raw_svg(w, parent_transform)?;
                write!(w, "</g>")?;
            }
            Shape::Ellipse(e) => {
                let EllipsePosition {
                    center,
                    semi_major_axis,
                    semi_minor_axis,
                    rotation,
                } = e.position(parent_transform);
                write!(
                    w,
                    r#"<ellipse cx="{cx}" cy="{cy}" rx="{semi_major_axis}" ry="{semi_minor_axis}" transform="rotate({rot}rad)"/>"#,
                    cx = center.x,
                    cy = center.y,
                    rot = -rotation.to_degrees()
                )?;
            }
            Shape::Text(t) => {
                todo!()
            }
            Shape::Curve(c) => {
                write!(w, r#"<path d=""#)?;

                fn write_curve<W: Write>(
                    w: &mut W,
                    c: &Curve,
                    parent_transform: &Transform2<f32>,
                    has_start: &mut bool,
                ) -> Result<(), SvgError> {
                    let CurvePosition { keypoints } = c.position(parent_transform);
                    for k in keypoints {
                        match k {
                            Keypoint::Point(p) => {
                                if *has_start {
                                    write!(w, "L ")?;
                                } else {
                                    write!(w, "M ")?;
                                    *has_start = true;
                                }
                                write!(w, "{} {} ", p.x, p.y)?;
                            }
                            Keypoint::Bezier(b) => {
                                if *has_start {
                                    if let Some(v) = b.start {
                                        write!(w, "L {} {} ", v.x, v.y)?;
                                    }
                                } else {
                                    if let Some(v) = b.start {
                                        write!(w, "M {} {} ", v.x, v.y)?;
                                    } else {
                                        return Err(SvgError::CurveHasNoStartingPoint);
                                    }
                                }

                                write!(w, "C {start_ctrl_x} {start_ctrl_y} {end_ctrl_x} {end_ctrl_y} {end_x} {end_y} ", 
								start_ctrl_x = b.start_control.x,
								start_ctrl_y = b.start_control.y,
								end_ctrl_x = b.end_control.x,
								end_ctrl_y = b.end_control.y,
								end_x = b.end.x,
								end_y = b.end.y,
							)?;
                            }
                            Keypoint::Curve(c) => {
                                write_curve(w, &c, parent_transform, has_start)?;
                            }
                        }
                    }

                    Ok(())
                }

                write_curve(w, c, parent_transform, &mut false)?;

                if c.closed {
                    write!(w, " Z")?;
                }
                write!(w, r#""/>"#)?;
            }
            x => {
                todo!("{x:?}")
            }
        }

        Ok(())
    }
}
