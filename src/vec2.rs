use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    /// Vector of zero.
    pub const fn zero() -> Self {
        Vec2::from_cartesian(0., 0.)
    }

    /// Vector of one.
    pub const fn ones() -> Self {
        Vec2::from_cartesian(1., 1.)
    }

    pub const fn from_cartesian_tuple((x, y): (f32, f32)) -> Self {
        Self::from_cartesian(x, y)
    }

    pub const fn from_cartesian(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn from_polar_deg(mag: f32, angle_deg: f32) -> Self {
        Self::from_polar_rad(mag, angle_deg.to_radians())
    }

    pub fn from_polar_rad(mag: f32, angle_rad: f32) -> Self {
        Vec2 {
            x: mag * angle_rad.cos(),
            y: mag * angle_rad.sin(),
        }
    }

    pub fn rot_deg(&self, deg: f32) -> Self {
        self.rot_rad(deg.to_radians())
    }

    pub fn rot_rad(&self, rad: f32) -> Self {
        Vec2 {
            x: rad.cos() * self.x + rad.sin() * self.y,
            y: rad.sin() * self.x - rad.cos() * self.y,
        }
    }

    /// Absolute value.
    pub fn abs(&self) -> Self {
        Vec2 {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::from_cartesian_tuple((x, y))
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(mut self, rhs: Vec2) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
