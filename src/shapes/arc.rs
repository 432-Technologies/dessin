use crate::{position::Rect, style::Style};

#[derive(Debug, Clone)]
pub struct Arc {
    pub pos: Rect,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub from_deg: f32,
    pub to_deg: f32,
    pub style: Option<Style>,
}
macros::impl_pos!(Arc);
macros::impl_style!(Arc);
