use crate::ToSVG;
use dessin::{shape::*, style::*, Shape, ShapeType, Vec2};
use std::error::Error;

impl ToSVG for Shape {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        let pos = self.pos.position_from_center();
        let size = self.pos.size();
        match &self.shape_type {
            ShapeType::Text {
                text,
                align,
                font_size,
                font_weight,
            } => Ok(format!(
                r#"<text x="{x}" y="{y}" {anchor} font-size="{size}" {weight} {style}>{text}</text>"#,
                x = pos.x,
                y = pos.y,
                anchor = align.to_svg()?,
                size = font_size,
                weight = font_weight.to_svg()?,
                style = self.style.to_svg()?,
                text = text,
            )),
            ShapeType::Line { from, to } => Ok(format!(
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" {style}/>"#,
                x1 = from.x,
                y1 = from.y,
                x2 = to.x,
                y2 = to.y,
                style = self.style.to_svg()?,
            )),
            ShapeType::Circle { radius } => Ok(format!(
                r#"<circle cx="{x}" cy="{y}" r="{r}" {style}/>"#,
                x = pos.x,
                y = pos.y,
                r = radius,
                style = self.style.to_svg()?,
            )),
            ShapeType::Arc {
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
            } => {
                let pos = self.pos.position_from_center();

                let start = if start_angle.to_deg() >= 0. {
                    start_angle.to_deg()
                } else {
                    start_angle.to_deg() + 360.
                } % 360.;

                let end = if end_angle.to_deg() >= 0. {
                    end_angle.to_deg()
                } else {
                    end_angle.to_deg() + 360.
                } % 360.;

                let lower_left = Vec2::from_polar_deg(*inner_radius, start) + pos;
                let upper_left = Vec2::from_polar_deg(*outer_radius, start) + pos;
                let lower_right = Vec2::from_polar_deg(*inner_radius, end) + pos;
                let upper_right = Vec2::from_polar_deg(*outer_radius, end) + pos;

                let mut span = end - start;
                span = if span >= 0. { span } else { span + 360. } % 360.;
                let large_arc_flag = if span >= 180. { 1 } else { 0 };

                let res = format!(r#"<path d="M {x} {y} "#, x = lower_left.x, y = lower_left.y);
                let res = format!("{}L {x}, {y} ", res, x = upper_left.x, y = upper_left.y);

                let res = format!(
                    "{}A {r} {r} {s} {f} 1 {x}, {y} ",
                    res,
                    r = outer_radius,
                    s = span,
                    f = large_arc_flag,
                    x = upper_right.x,
                    y = upper_right.y
                );

                let res = format!("{}L {x}, {y} ", res, x = lower_right.x, y = lower_right.y);
                let res = format!(
                    "{}A {r} {r} {s} {f} 0 {x}, {y} ",
                    res,
                    r = inner_radius,
                    s = span,
                    f = large_arc_flag,
                    x = lower_left.x,
                    y = lower_left.y
                );

                let res = format!(r#"{}Z" {style}></path>"#, res, style = self.style.to_svg()?,);
                Ok(res)
            }
            ShapeType::Image { data } => Ok(format!(
                r#"<image x="{x}" y="{y}" width="{width}" height="{height}" xlink:href="{href}"/>"#,
                x = pos.x,
                y = pos.y,
                width = size.x,
                height = size.y,
                href = match data {
                    ImageFormat::PNG(ref d) => {
                        format!("data:image/png;base64,{}", base64::encode(d))
                    }
                    ImageFormat::JPEG(ref d) => {
                        format!("data:image/jpeg;base64,{}", base64::encode(d))
                    }
                }
            )),
            ShapeType::Drawing(shapes) => shapes.to_svg(),
            ShapeType::Path { keypoints, closed } => {
                let start = keypoints.first().ok_or("No start")?;
                let rest = &keypoints[1..];

                Ok(format!(
                    r#"<path d="{start} {rest} {close}" {style}/>"#,
                    style = self.style.to_svg()?,
                    start = if let Keypoint::Point(start) = start {
                        format!("M {} {} ", start.x, start.y)
                    } else {
                        unreachable!();
                    },
                    rest = rest
                        .iter()
                        .map(|v| match v {
                            Keypoint::Point(p) => format!("L {} {} ", p.x, p.y),
                            Keypoint::Bezier {
                                destination,
                                start_prop,
                                dest_prop,
                            } => format!(
                                "C {} {} {} {} {} {} ",
                                start_prop.x,
                                start_prop.y,
                                dest_prop.x,
                                dest_prop.y,
                                destination.x,
                                destination.y,
                            ),
                        })
                        .collect::<String>(),
                    close = if *closed { "Z" } else { "" }
                ))
            }
        }
    }
}

impl ToSVG for Option<Style> {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.as_ref()
            .map(|v| {
                Ok(format!(
                    r#"{fill} {stroke}"#,
                    fill = v.fill.to_svg()?,
                    stroke = v.stroke.to_svg()?,
                ))
            })
            .unwrap_or_else(|| Ok(String::new()))
    }
}

impl ToSVG for Option<Fill> {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Some(Fill::Color(c)) => Ok(format!("fill='{}'", c.to_svg()?)),
            None => Ok("fill='none'".to_string()),
        }
    }
}

impl ToSVG for Option<Stroke> {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Some(Stroke::Full { color, width }) => Ok(format!(
                "stroke='{}' stroke-width='{}'",
                color.to_svg()?,
                width
            )),
            Some(Stroke::Dashed {
                color,
                width,
                on,
                off,
            }) => Ok(format!(
                "stroke='{}' stroke-width='{}' stroke-dasharray='{},{}'",
                color.to_svg()?,
                width,
                on,
                off
            )),
            None => Ok(String::new()),
        }
    }
}

impl ToSVG for TextAlign {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            r#"text-anchor="{}""#,
            match self {
                TextAlign::Left => "left",
                TextAlign::Center => "middle",
                TextAlign::Right => "right",
            }
        ))
    }
}

impl ToSVG for FontWeight {
    fn to_svg(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(match &self {
            FontWeight::Regular => "".to_owned(),
            FontWeight::Bold => r#"font-weight="bold""#.to_owned(),
            FontWeight::Italic => r#"font-weight="italic""#.to_owned(),
            FontWeight::BoldItalic => r#"font-weight="bold italic""#.to_owned(),
        })
    }
}

impl ToSVG for Color {
    fn to_svg(&self) -> Result<String, Box<dyn Error>> {
        let c = self.rgba();
        if let Color::RGBA { r, g, b, a } = c {
            Ok(format!("rgba({},{},{},{})", r, g, b, a))
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dessin::{vec2, Drawing};

    struct MyStruct;
    impl ToSVG for MyStruct {
        fn to_svg(&self) -> Result<String, Box<dyn Error>> {
            Ok("MyStruct".to_owned())
        }
    }

    #[test]
    fn test_text() {
        let mut drawing = Drawing::empty();
        drawing.add(
            Text::new("hello world".to_owned())
                .at(vec2(10., 10.))
                .with_font_weight(FontWeight::Bold)
                .with_font_size(10.)
                .with_fill(Fill::Color(Color::U32(0xFF0000)))
                .with_align(TextAlign::Center),
        );

        let text_svg = r#"<text x="10" y="10" text-anchor="middle" font-size="10" font-weight="bold" fill='#FF0000FF' >hello world</text>"#;
        let drawing_svg = format!(
            r#"<svg width="0px" height="0px" viewBox="-0 -0 0 0">{}</svg>"#,
            text_svg
        );

        assert_eq!(drawing.shapes()[0].to_svg().unwrap(), text_svg);
        assert_eq!(drawing.to_svg().unwrap(), drawing_svg);
    }

    #[test]
    fn test_line() {
        let mut drawing = Drawing::empty();
        drawing.add(
            Line::from(vec2(10., 10.))
                .to(vec2(20., 20.))
                .with_fill(Fill::Color(Color::U32(0xFF0000))),
        );
        let line_svg = r#"<line x1="10" y1="10" x2="20" y2="20" fill='#FF0000' />"#;
        let drawing_svg = format!(
            r#"<svg width="0px" height="0px" viewBox="-0 -0 0 0">{}</svg>"#,
            line_svg
        );
        assert_eq!(drawing.shapes()[0].to_svg().unwrap(), line_svg);
        assert_eq!(drawing.to_svg().unwrap(), drawing_svg);
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Regular.to_svg().unwrap(), "".to_owned());
        assert_eq!(
            FontWeight::Bold.to_svg().unwrap(),
            r#"font-weight="bold""#.to_owned()
        );
        assert_eq!(
            FontWeight::Italic.to_svg().unwrap(),
            r#"font-weight="italic""#.to_owned()
        );
        assert_eq!(
            FontWeight::BoldItalic.to_svg().unwrap(),
            r#"font-weight="bold italic""#.to_owned()
        );
    }

    #[test]
    fn test_color() {
        let rgb = Color::RGB {
            r: 0x12,
            g: 0x34,
            b: 0x56,
        };
        assert_eq!(rgb.to_svg().unwrap(), "#123456");

        let u = Color::U32(0x12345678);
        assert_eq!(u.to_svg().unwrap(), "#12345678");

        let red = Color::U32(0xFF0000);
        assert_eq!(red.to_svg().unwrap(), "#FF0000");

        let blue = Color::U32(0x0000FF);
        assert_eq!(blue.to_svg().unwrap(), "#0000FF");
    }
}
