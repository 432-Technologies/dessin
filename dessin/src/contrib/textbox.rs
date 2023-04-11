// use crate::{
//     shape::Style,
//     shapes::text::{FontWeight, Text, TextAlign},
//     Drawing, Rect, Shape,
// };
// use algebr::{vec2, Vec2};
// use rusttype::{Font, Scale};

use crate::prelude::{FontWeight, Shape, ShapeOp, Text, TextAlign};
use fontdue::{Font, FontSettings};
use image::EncodableLayout;
use nalgebra::Transform2;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextBox {
    pub local_transform: Transform2<f32>,
    pub font_size: f32,
    pub line_spacing: f32,
    pub text: String,
    pub width: f32,
    pub height: Option<f32>,
}
impl TextBox {
    #[inline]
    fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size *= font_size;
        self
    }
    #[inline]
    fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size(font_size);
        self
    }
}

impl ShapeOp for TextBox {
    #[inline]
    fn transform(&mut self, transform_matrix: nalgebra::Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &nalgebra::Transform2<f32> {
        &self.local_transform
    }
}

impl From<TextBox> for Shape {
    fn from(
        TextBox {
            local_transform,
            font_size,
            line_spacing,
            text,
            width,
            height,
        }: TextBox,
    ) -> Self {
        let fonts = crate::font::get(0);
        let crate::font::Font::Bytes(raw_font) = fonts.get(FontWeight::Regular) else {
			todo!()
		};

        let font = Font::from_bytes(raw_font.as_bytes(), FontSettings::default()).unwrap();

        fn size_of(font: &Font, s: &str, font_size: f32) -> Option<f32> {
            s.chars()
                .scan(None, |last, curr| {
                    let l = last.unwrap_or(' ');
                    let r = font.horizontal_kern(l, curr, font_size);

                    *last = Some(curr);

                    Some(r)
                })
                .fold(Some(0.), |acc, curr| match (acc, curr) {
                    (None, _) | (_, None) => None,
                    (Some(acc), Some(curr)) => Some(acc + curr),
                })
        }

        let mut lines = vec![];
        let mut height = height.unwrap_or(f32::MAX);

        for line in text.lines() {
            let mut len = 0.;
            let mut acc = String::new();

            if height - font_size < 0. {
                break;
            }

            for word in line.split_whitespace() {
                let word_size = size_of(&font, word, font_size).unwrap();

                if len + word_size > width {
                    lines.push(std::mem::take(&mut acc));

                    acc = word.to_owned();
                    len = word_size;

                    height -= font_size + line_spacing;
                } else {
                    len += word_size;

                    if !acc.is_empty() {
                        acc.push(' ');
                    }

                    acc.push_str(word)
                }
            }
        }

        let shapes = lines
            .into_iter()
            .map(|v| {
                Shape::Text(Text {
                    text: v,
                    local_transform,
                    align: TextAlign::Left,
                    font_weight: FontWeight::Regular,
                    on_curve: None,
                    font_size,
                    font: None,
                })
            })
            .collect();

        Shape::Group {
            local_transform,
            shapes,
        }
    }
}

//         fn length_of(word: &str, font: Font, scale: f32) -> f32 {
//             let scale = Scale::uniform(scale);
//             font.glyphs_for(word.chars())
//                 .scan(None, |last, g| {
//                     let pos = if let Some(last) = last {
//                         font.pair_kerning(scale, *last, g.clone().id())
//                     } else {
//                         0.0
//                     } + g.clone().scaled(scale).h_metrics().advance_width;
//                     *last = Some(g.id());
//                     Some(pos)
//                 })
//                 .sum()
//         }

//         text.push(' ');

//         let mut pos = Vec2::zero();
//         let mut box_idx = 0;
//         let mut acc = String::new();
//         let mut canvas = Rect::new();
//         let mut drawing = Drawing::empty();

//         for line in text.split("\n") {
//             for word in line.split(" ").chain(Some("\n")) {
//                 if let Some(b) = boxes.get(box_idx) {
//                     let font = match b.font_weight {
//                         FontWeight::Regular => Font::try_from_bytes(ARIAL_REGULAR),
//                         FontWeight::Bold => Font::try_from_bytes(ARIAL_BOLD),
//                         FontWeight::Italic => Font::try_from_bytes(ARIAL_ITALIC),
//                         FontWeight::BoldItalic => Font::try_from_bytes(ARIAL_BOLD_ITALIC),
//                     }
//                     .unwrap();

//                     let hard_new_line = if word == "\n" {
//                         true
//                     } else {
//                         let len = length_of(word, font, b.font_size);
//                         pos.x += len;

//                         if acc.len() > 0 {
//                             acc.push(' ');
//                         }
//                         acc.push_str(word);

//                         false
//                     };

//                     if hard_new_line || pos.x > b.pos.size().x {
//                         let text = std::mem::take(&mut acc);
//                         let res = Text::new(text)
//                             .at(global_pos.pos + b.pos.pos - vec2(0., pos.y))
//                             .with_anchor(b.pos.anchor)
//                             .with_size(global_pos.size() * b.pos.size())
//                             .with_align(b.align)
//                             .with_font_size(global_pos.size().x * b.font_size) // TODO: Non-uniform size
//                             .with_font_weight(b.font_weight)
//                             .with_style(b.style.clone().unwrap_or_default());

//                         canvas = canvas.union(res.pos);

//                         drawing.add(res);

//                         pos.y += (b.font_size + b.spacing) * global_pos.size().x; // TODO: Non-uniform size
//                         pos.x = 0.;

//                         if pos.y >= b.pos.size().y {
//                             box_idx += 1;
//                             pos.y = 0.;
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//         }

//         drawing.into()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Drawing;
//     use algebr::vec2;

//     #[test]
//     fn new() {
//         let mut drawing = Drawing::empty().with_canvas_size(vec2(300., 300.));
//         drawing.add(
//             TextLayout::new("Hello world".to_owned())
//                 .add_box(TextBox::new().at(vec2(10., 10.)).with_size(vec2(10., 10.))),
//         );
//     }
// }
