use algebra::Vec2;

use crate::style::Style;

#[derive(Debug, Clone)]
pub struct Line<const IS_INIT: bool> {
    pub from: Vec2,
    pub to: Vec2,
    pub style: Option<Style>,
}
macros::impl_style!(Line<true>);
impl Line<false> {
    pub const fn from(from: Vec2) -> Self {
        Line {
            from,
            to: from,
            style: None,
        }
    }

    pub const fn to(self, to: Vec2) -> Line<true> {
        Line {
            from: self.from,
            to,
            style: self.style,
        }
    }
}
