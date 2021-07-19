#[macro_export]
macro_rules! impl_pos {
    ($t:ty) => {
        impl $t {
            pub const fn with_pos(mut self, pos: crate::position::Rect) -> Self {
                self.pos = pos;
                self
            }

            pub const fn at(mut self, pos: ::algebra::Vec2) -> Self {
                self.pos = self.pos.at(pos);
                self
            }

            pub const fn with_anchor(mut self, anchor: ::algebra::Vec2) -> Self {
                self.pos = self.pos.with_anchor(anchor);
                self
            }

            pub const fn with_size(mut self, size: ::algebra::Vec2) -> Self {
                self.pos = self.pos.with_size(size);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_style {
    ($t:ty) => {
        impl $t {
            pub const fn with_style(mut self, style: crate::style::Style) -> Self {
                self.style = Some(style);
                self
            }

            pub fn with_stroke(mut self, stroke: crate::style::Stroke) -> Self {
                self.style = {
                    let mut style = self.style.unwrap_or_default();
                    style.stroke = Some(stroke);
                    Some(style)
                };
                self
            }

            pub fn with_fill(mut self, fill: crate::style::Fill) -> Self {
                self.style = {
                    let mut style = self.style.unwrap_or_default();
                    style.fill = Some(fill);
                    Some(style)
                };
                self
            }
        }
    };
}
