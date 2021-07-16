use algebra::Vec2;

pub struct Rect<const LockedSize: bool> {
    pub pos: Vec2,
    pub anchor: Vec2,
    pub size: Vec2,
}
impl<const LockedSize: bool> Rect<LockedSize> {
    pub const fn new() -> Self {
        Rect {
            pos: Vec2::zero(),
            anchor: Vec2::zero(),
            size: Vec2::ones(),
        }
    }
    pub const fn with_pos(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }
    pub const fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }
    pub fn position_from_center(&self) -> Vec2 {
        self.position_from_anchor(Vec2::zero())
    }
    pub fn position_from_anchor(&self, new_anchor: Vec2) -> Vec2 {
        self.pos + (new_anchor - self.anchor) * self.size / 2.
    }
}
impl Rect<false> {
    pub const fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}
impl Rect<true> {
    pub const fn with_size(mut self, size: f32) -> Self {
        let scale = self.size.y / self.size.x;
        self.size.x = size;
        self.size.y = scale * size;
        self
    }
}
