use crate::{font::FontRef, prelude::*};
use fontdue::{Font, FontSettings};
use nalgebra::{Transform2, Translation2};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextBox {
    pub local_transform: Transform2<f32>,
    pub font_size: f32,
    pub line_spacing: f32,
    pub align: TextAlign,
    pub vertical_align: TextVerticalAlign,
    pub text: String,
    pub font_weight: FontWeight,
    /// Dimension on the x-axis
    pub width: f32,
    /// Dimension on the y-axis
    pub height: Option<f32>,
    pub font: Option<FontRef>,
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
    pub fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size = font_size;
        self
    }
    #[inline]
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size(font_size);
        self
    }

    #[inline]
    pub fn text<T: ToString>(&mut self, text: T) -> &mut Self {
        self.text = text.to_string();
        self
    }
    #[inline]
    pub fn with_text<T: ToString>(mut self, text: T) -> Self {
        self.text(text);
        self
    }

    #[inline]
    pub fn line_spacing(&mut self, line_spacing: f32) -> &mut Self {
        self.line_spacing = line_spacing;
        self
    }
    #[inline]
    pub fn with_line_spacing(mut self, line_spacing: f32) -> Self {
        self.line_spacing(line_spacing);
        self
    }

    #[inline]
    pub fn align(&mut self, align: TextAlign) -> &mut Self {
        self.align = align;
        self
    }
    #[inline]
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align(align);
        self
    }

    #[inline]
    pub fn vertical_align(&mut self, vertical_align: TextVerticalAlign) -> &mut Self {
        self.vertical_align = vertical_align;
        self
    }
    #[inline]
    pub fn with_vertical_align(mut self, vertical_align: TextVerticalAlign) -> Self {
        self.vertical_align(vertical_align);
        self
    }

    #[inline]
    pub fn font_weight(&mut self, font_weight: FontWeight) -> &mut Self {
        self.font_weight = font_weight;
        self
    }
    #[inline]
    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight(font_weight);
        self
    }

    #[inline]
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }
    #[inline]
    pub fn with_width(mut self, width: f32) -> Self {
        self.width(width);
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
            crate::font::Font::ByName(_) => todo!(),
        };

        let font = Font::from_bytes(raw_font.as_slice(), FontSettings::default()).unwrap();

        fn size_of(font: &Font, s: &str, font_size: f32) -> f32 {
            s.chars()
                .scan(None, |last, curr| {
                    let l = last.unwrap_or(' ');
                    let r = if let Some(v) = font.horizontal_kern(l, curr, font_size) {
                        v
                    } else {
                        font.metrics(curr, font_size).advance_width
                    };

                    *last = Some(curr);

                    Some(r)
                })
                .sum()
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

        dessin!(for line in {lines.into_iter().enumerate()}: {
            let (line, text) = line;
            let line = line as f32;
            let (vertical_align, growing_direction) = match vertical_align {
                TextVerticalAlign::Bottom => (TextVerticalAlign::Top, 1.),
                TextVerticalAlign::Center => (TextVerticalAlign::Center, -1.),
                TextVerticalAlign::Top => (TextVerticalAlign::Bottom, 1.),
            };

            let translation = Translation2::new(0., growing_direction * (font_size + line_spacing) * line);

            let mut text = dessin!(Text: (
                transform={local_transform * translation}
                text={text}
                align={align}
                vertical_align={vertical_align}
                font_weight={font_weight}
                font_size={font_size}
            ));

            text.font = font_ref.clone();
            text
        })
    }
}
