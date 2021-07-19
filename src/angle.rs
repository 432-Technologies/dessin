use std::{f32::consts::PI, ops::Deref};

/// Helper function to create Radians.
pub fn radians(rad: f32) -> Radians {
    Radians::new(rad)
}

/// Helper function to create Degrees.
pub fn degrees(rad: f32) -> Degrees {
    Degrees::new(rad)
}

/// Radians.
pub struct Radians(f32);
impl Radians {
    pub fn new(rad: f32) -> Radians {
        Radians (rad)
    }

    pub fn to_degrees(&self) -> Degrees {
        Degrees(**self * 180. / PI)
    }
}
impl Deref for Radians {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}


/// Degrees.
pub struct Degrees(f32);
impl Degrees {
    pub fn new(deg: f32) -> Degrees {
        Degrees(deg)
    }
}
impl Deref for Degrees {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}