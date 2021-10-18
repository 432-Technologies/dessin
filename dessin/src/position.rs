use algebr::{vec2, Vec2};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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

    pub fn union(&self, other: Rect) -> Rect {
        let mins_self = self.position_from_anchor(vec2(-1., -1.));
        let mins_other = other.position_from_anchor(vec2(-1., -1.));

        let maxs_self = self.position_from_anchor(vec2(1., 1.));
        let maxs_other = other.position_from_anchor(vec2(1., 1.));

        let mins = vec2(mins_self.x.min(mins_other.x), mins_self.y.min(mins_other.y));
        let maxs = vec2(maxs_self.x.max(maxs_other.x), maxs_self.y.max(maxs_other.y));

        Rect {
            size: Some(maxs - mins),
            pos: (maxs + mins) / 2.,
            anchor: Vec2::zero(),
        }
    }
}

pub trait RectOp {
    fn union(&self) -> Rect;
}
impl RectOp for &[Rect] {
    fn union(&self) -> Rect {
        self.iter().fold(Rect::new(), |acc, curr| acc.union(*curr))
    }
}
