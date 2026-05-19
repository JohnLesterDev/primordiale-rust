use std::ops::{Add, AddAssign, Sub, Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    pub fn length(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
    pub fn distance(&self, other: &Vec2) -> f32 { (*self - *other).length() }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 { Self::new(self.x / len, self.y / len) } else { Self::zero() }
    }
    pub fn lerp(&self, target: Vec2, t: f32) -> Self {
        Self::new(self.x + (target.x - self.x) * t, self.y + (target.y - self.y) * t)
    }
}

impl Add for Vec2 { type Output = Self; fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y) } }
impl AddAssign for Vec2 { fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; } }
impl Sub for Vec2 { type Output = Self; fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y) } }
impl Mul<f32> for Vec2 { type Output = Self; fn mul(self, rhs: f32) -> Self { Self::new(self.x * rhs, self.y * rhs) } }
impl MulAssign<f32> for Vec2 { fn mul_assign(&mut self, rhs: f32) { self.x *= rhs; self.y *= rhs; } }