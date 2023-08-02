use crate::{font::FontRef, prelude::*};
use fontdue::{Font, FontSettings};
use nalgebra::Transform2;

#[derive(Debug, Clone, PartialEq, Shape)]
pub struct TextBox {
    #[local_transform]
    pub local_transform: Transform2<f32>,
    pub font_size: f32,
    pub line_spacing: f32,
    pub align: TextAlign,
    pub vertical_align: TextVerticalAlign,
    #[shape(into)]
    pub text: String,
    pub font_weight: FontWeight,
    /// Dimension on the x-axis
    pub width: f32,
    /// Dimension on the y-axis
    #[shape(skip)]
    pub height: Option<f32>,
    #[shape(skip)]
    pub font: Option<FontRef>,
}
impl Default for TextBox {
    fn default() -> Self {
        TextBox {
            local_transform: Default::default(),
            font_size: Default::default(),
            line_spacing: Default::default(),
            align: Default::default(),
            vertical_align: TextVerticalAlign::Top,
            text: Default::default(),
            font_weight: Default::default(),
            width: Default::default(),
            height: Default::default(),
            font: Default::default(),
        }
    }
}
impl TextBox {
    #[inline]
    pub fn font<F: Into<FontRef>>(&mut self, font: F) -> &mut Self {
        self.font = Some(font.into());
        self
    }
    #[inline]
    pub fn with_font<F: Into<FontRef>>(mut self, font: F) -> Self {
        self.font(font);
        self
    }

    #[inline]
    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = Some(height);
        self
    }
    #[inline]
    pub fn with_height(mut self, height: f32) -> Self {
        self.height(height);
        self
    }

    #[inline]
    pub fn no_height(&mut self) -> &mut Self {
        self.height = None;
        self
    }
    #[inline]
    pub fn without_weight(mut self) -> Self {
        self.no_height();
        self
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
            align,
            vertical_align,
            font_weight,
            font,
        }: TextBox,
    ) -> Self {
        let font_ref = font.clone();
        let fonts = crate::font::get(font.unwrap_or_default());
        let raw_font = match fonts.get(FontWeight::Regular) {
            crate::font::Font::OTF(bytes) => bytes,
            crate::font::Font::TTF(bytes) => bytes,
        };

        let font = Font::from_bytes(raw_font.as_slice(), FontSettings::default()).unwrap();

        let mut lines = vec![];
        let mut height = height.unwrap_or(f32::MAX);

        for line in text.lines() {
            let mut len = 0.;
            let mut acc = String::new();

            if height - font_size < 0. {
                break;
            }

            for word in line.split_whitespace() {
                if word.is_empty() {
                    continue;
                }

                let word_size = size_of(&font, word, font_size);

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

            if !acc.is_empty() {
                lines.push(acc)
            }
        }

        let (vertical_align, _) = match vertical_align {
            TextVerticalAlign::Bottom => (TextVerticalAlign::Top, 1.),
            TextVerticalAlign::Center => (TextVerticalAlign::Center, -1.),
            TextVerticalAlign::Top => (TextVerticalAlign::Bottom, -1.),
        };

        dessin!(VerticalLayout: (
            extend={lines.into_iter().map(|text| dessin!(Text: (
                text={text}
                align={align}
                vertical_align={vertical_align}
                font_weight={font_weight}
                font_size={font_size}
                maybe_font={font_ref.clone()}
            )).into())}
            gap={line_spacing}
            transform={local_transform}
        ))
        .into()
    }
}

impl ShapeBoundingBox for TextBox {
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        // self.font_size
        todo!()
    }
}

#[test]
fn one_line() {
    use assert_float_eq::*;

    let text = "it should work pass famous last word";

    let shape: Shape = dessin!(TextBox: #(
        {text}
        fill={Fill::Color(Color::BLACK)}
        font_size={5.}
        align={TextAlign::Left}
        line_spacing={2.}
    ))
    .into();

    let bb = shape.local_bounding_box();
    assert_float_absolute_eq!(bb.height(), 5., 0.001);
}

#[test]
fn two_lines() {
    use assert_float_eq::*;

    let text = "it should work pass\nfamous last word";

    let shape: Shape = dessin!(TextBox: #(
        {text}
        fill={Fill::Color(Color::BLACK)}
        font_size={5.}
        align={TextAlign::Left}
        line_spacing={2.}
    ))
    .into();

    let bb = shape.local_bounding_box();

    assert_float_absolute_eq!(bb.height(), 12., 0.0001);
}
