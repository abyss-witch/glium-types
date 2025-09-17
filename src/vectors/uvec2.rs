use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;

use super::{uvec3::*, vec2::Vec2, bvec2::*};
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign
)]
/// an unsigned interger vector made from a x and y coordinate.
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
impl UVec2 {
    ///a zero vector
    pub const ZERO: Self = uvec2(0, 0);
    ///a vector full of ones
    pub const ONE: Self = uvec2(1, 1);
    ///the x axis
    pub const X: Self = uvec2(1, 0);
    ///the y axis
    pub const Y: Self = uvec2(0, 1);

    pub const fn new(x: u32, y: u32) -> Self { Self { x, y } }
    pub const fn extend(self, z: u32) -> UVec3 { uvec3(self.x, self.y, z) }
    pub const fn truncate(self) -> u32 { self.x }
    /// create a vector where x and y equals `value`.
    pub const fn splat(value: u32) -> Self { Self::new(value, value) }

    /// the length of the vector without being square rooted.
    pub fn length_squared(self) -> u32 {
        self.x*self.x + self.y*self.y
    }
    /// distance between two vectors without being square rooted.
    pub fn distance_squared(self, other: UVec2) -> u32 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: UVec2) -> u32 {
        self.x * other.x + self.y * other.y
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: u32) -> UVec2 {
        Self::new(self.x * scalar, self.y * scalar)
    }
    /// returns whether the 2 components are equal
    pub fn eq(self, rhs: Self) -> BVec2 { bvec2(self.x == rhs.x, self.y == rhs.y) }
    /// returns whether the 1st components are less than the 2nd
    pub fn less(self, rhs: Self) -> BVec2 { bvec2(self.x < rhs.x, self.y < rhs.y) }
    /// returns whether the 1st components are more than the 2nd
    pub fn more(self, rhs: Self) -> BVec2 { bvec2(self.x > rhs.x, self.y > rhs.y) }
    /// returns whether the 1st components are less than or equal to the 2nd
    pub fn less_or_eq(self, rhs: Self) -> BVec2 { bvec2(self.x <= rhs.x, self.y <= rhs.y) }
    /// returns whether the 1st components are more than or equal to the 2nd
    pub fn more_or_eq(self, rhs: Self) -> BVec2 { bvec2(self.x >= rhs.x, self.y >= rhs.y) } 
}
impl std::ops::Mul<UVec2> for u32 {
    fn mul(self, rhs: UVec2) -> Self::Output { rhs * self }
    type Output = UVec2;
}
impl std::ops::Mul<u32> for UVec2 {
    fn mul(self, rhs: u32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<u32> for UVec2 { fn mul_assign(&mut self, rhs: u32) { *self = *self * rhs } }
impl std::ops::Div<UVec2> for u32 {
    fn div(self, rhs: UVec2) -> Self::Output { UVec2::splat(self) / rhs }
    type Output = UVec2;
}
impl std::ops::Div<u32> for UVec2 {
    fn div(self, rhs: u32) -> Self::Output { self / UVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<u32> for UVec2 { fn div_assign(&mut self, rhs: u32) { *self = *self / rhs } }
impl std::ops::Rem<UVec2> for u32 {
    fn rem(self, rhs: UVec2) -> Self::Output { UVec2::splat(self) % rhs }
    type Output = UVec2;
}
impl std::ops::Rem<u32> for UVec2 {
    fn rem(self, rhs: u32) -> Self::Output { self % UVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<u32> for UVec2 { fn rem_assign(&mut self, rhs: u32) { *self = *self % rhs } }

impl AsUniformValue for UVec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::UnsignedIntVec2([self.x, self.y])
    }
}
///create an unsigned interger vector with an x and y coordinate.
pub const fn uvec2(x: u32, y: u32) -> UVec2{
    UVec2 { x, y }
}
impl From<Vec2> for UVec2 {
    fn from (value: Vec2) -> Self {
        uvec2(value.x as u32, value.y as u32)
    }
}
impl From<[u32; 2]> for UVec2 {
    fn from(value: [u32; 2]) -> Self {
        uvec2(value[0], value[1])
    }
}
impl From<(u32, u32)> for UVec2 {
    fn from(value: (u32, u32)) -> Self {
        uvec2(value.0, value.1)
    }
}
impl From<UVec2> for [u32; 2] {
    fn from(value: UVec2) -> Self {
        [value.x, value.y]
    }
}
impl From<UVec2> for (u32, u32) {
    fn from(value: UVec2) -> Self {
        (value.x, value.y)
    }
}
