use algebr::Vec2;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rect {
    pub(crate) pos: Vec2,
    pub(crate) anchor: Vec2,
    pub(crate) size: Option<Vec2>,
}
impl Rect {
    pub const fn new() -> Self {
        Rect {
            pos: Vec2::zero(),
            anchor: Vec2::zero(),
            size: None,
        }
    }

    pub const fn at(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }

    pub const fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }

    pub const fn with_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn position_from_center(&self) -> Vec2 {
        self.position_from_anchor(Vec2::zero())
    }

    pub fn position_from_anchor(&self, new_anchor: Vec2) -> Vec2 {
        self.pos + (new_anchor - self.anchor) * self.size() / 2.
    }

    pub fn size(&self) -> Vec2 {
        self.size.unwrap_or(Vec2::ones())
    }
}
