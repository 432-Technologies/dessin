use std::f32::consts::PI;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Angle {
    Radians(f32),
    Degrees(f32),
}
impl Angle {
    pub const fn rad(rad: f32) -> Angle {
        Angle::Radians(rad)
    }

    pub const fn deg(deg: f32) -> Angle {
        Angle::Degrees(deg)
    }

    pub const fn radians(radians: f32) -> Angle {
        Angle::Radians(radians)
    }

    pub const fn degrees(degrees: f32) -> Angle {
        Angle::Degrees(degrees)
    }

    pub fn to_rad(&self) -> f32 {
        self.to_radians()
    }

    pub fn to_deg(&self) -> f32 {
        self.to_degrees()
    }

    pub fn to_radians(&self) -> f32 {
        match self {
            Angle::Radians(radians) => *radians,
            Angle::Degrees(degrees) => PI * degrees / 180.0,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        match self {
            Angle::Radians(radians) => 180.0 * radians / PI,
            Angle::Degrees(degrees) => *degrees,
        }
    }
}
