use algebr::Vec2;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rect {
    pos: Vec2,
    anchor: Vec2,
    size: Option<Vec2>,
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

    pub fn position_from_center(&self) -> Option<Vec2> {
        self.position_from_anchor(Vec2::zero())
    }

    pub fn position_from_anchor(&self, new_anchor: Vec2) -> Option<Vec2> {
        self.size
            .map(|size| self.pos + (new_anchor - self.anchor) * size / 2.)
    }

    pub const fn size(&self) -> Option<Vec2> {
        self.size
    }
}
