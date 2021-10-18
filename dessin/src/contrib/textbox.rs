use crate::{
    shape::Style,
    shapes::text::{FontWeight, Text, TextAlign},
    Drawing, Rect, Shape,
};
use algebr::{vec2, Vec2};
use rusttype::{Font, Scale};

#[derive(Debug, Clone, PartialEq)]
pub struct TextLayout {
    pub(crate) text: String,
    pub(crate) pos: Rect,
    pub(crate) boxes: Vec<TextBox>,
}
crate::impl_pos_at!(TextLayout);
crate::impl_pos_anchor!(TextLayout);
impl TextLayout {
    pub const fn new(text: String) -> Self {
        TextLayout {
            text,
            pos: Rect::new(),
            boxes: vec![],
        }
    }

    pub fn add_box(mut self, b: TextBox) -> Self {
        self.boxes.push(b);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.pos = self.pos.with_size(vec2(scale, scale));
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBox {
    pub(crate) pos: Rect,
    pub(crate) style: Option<Style>,
    pub(crate) align: TextAlign,
    pub(crate) font_size: f32,
    pub(crate) font_weight: FontWeight,
    pub(crate) spacing: f32,
}
crate::impl_pos!(TextBox);
crate::impl_style!(TextBox);
impl TextBox {
    pub const fn new() -> Self {
        TextBox {
            pos: Rect::new(),
            style: None,
            align: TextAlign::Left,
            font_size: 16.,
            font_weight: FontWeight::Regular,
            spacing: 0.,
        }
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.pos = self.pos.with_size(vec2(scale, scale));
        self
    }

    pub const fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub const fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub const fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub const fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }
}
impl From<TextLayout> for Shape {
    fn from(
        TextLayout {
            mut text,
            pos: global_pos,
            boxes,
        }: TextLayout,
    ) -> Self {
        const ARIAL_REGULAR: &[u8] = include_bytes!("Arial.ttf");
        const ARIAL_BOLD: &[u8] = include_bytes!("Arial Bold.ttf");
        const ARIAL_ITALIC: &[u8] = include_bytes!("Arial Italic.ttf");
        const ARIAL_BOLD_ITALIC: &[u8] = include_bytes!("Arial Bold Italic.ttf");

        fn length_of(word: &str, font: Font, scale: f32) -> f32 {
            let scale = Scale::uniform(scale);
            font.glyphs_for(word.chars())
                .scan(None, |last, g| {
                    let pos = if let Some(last) = last {
                        font.pair_kerning(scale, *last, g.clone().id())
                    } else {
                        0.0
                    } + g.clone().scaled(scale).h_metrics().advance_width;
                    *last = Some(g.id());
                    Some(pos)
                })
                .sum()
        }

        text.push(' ');

        let mut pos = Vec2::zero();
        let mut box_idx = 0;
        let mut acc = String::new();
        let mut canvas = Rect::new();
        let mut drawing = Drawing::empty();
        for word in text.split(" ") {
            if let Some(b) = boxes.get(box_idx) {
                let font = match b.font_weight {
                    FontWeight::Regular => Font::try_from_bytes(ARIAL_REGULAR),
                    FontWeight::Bold => Font::try_from_bytes(ARIAL_BOLD),
                    FontWeight::Italic => Font::try_from_bytes(ARIAL_ITALIC),
                    FontWeight::BoldItalic => Font::try_from_bytes(ARIAL_BOLD_ITALIC),
                }
                .unwrap();

                let len = length_of(word, font, b.font_size);
                pos.x += len;
                if acc.len() > 0 {
                    acc.push(' ')
                }
                acc.push_str(word);
                if pos.x > b.pos.size().x {
                    let text = std::mem::take(&mut acc);
                    let res = Text::new(text)
                        .at(global_pos.pos + b.pos.pos - vec2(0., pos.y))
                        .with_anchor(b.pos.anchor)
                        .with_size(global_pos.size() * b.pos.size())
                        .with_align(b.align)
                        .with_font_size(global_pos.size().x * b.font_size) // TODO: Non-uniform size
                        .with_font_weight(b.font_weight)
                        .with_style(b.style.clone().unwrap_or_default());
                    canvas = canvas.union(res.pos);
                    drawing.add(res);

                    pos.y += (b.font_size + b.spacing) * global_pos.size().x; // TODO: Non-uniform size
                    pos.x = 0.;

                    if pos.y >= b.pos.size().y {
                        box_idx += 1;
                        pos.y = 0.;
                    }
                }
            } else {
                break;
            }
        }

        drawing.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Drawing;
    use algebr::vec2;

    #[test]
    fn new() {
        let mut drawing = Drawing::empty().with_canvas_size(vec2(300., 300.));
        drawing.add(
            TextLayout::new("Hello world".to_owned())
                .add_box(TextBox::new().at(vec2(10., 10.)).with_size(vec2(10., 10.))),
        );
    }
}
