use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;

use super::{duvec3::{duvec3, DUVec3}, vec2::Vec2, uvec2::UVec2};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign)]
///an unsigned double interger vector made from a x and y coordinate.
pub struct DUVec2 {
    pub x: u64,
    pub y: u64,
}
impl DUVec2 {
    ///a zero vector
    pub const ZERO: Self = duvec2(0, 0);
    ///a vector full of ones
    pub const ONE: Self = duvec2(1, 1);
    ///the x axis
    pub const X: Self = duvec2(1, 0);
    ///the y axis
    pub const Y: Self = duvec2(0, 1);

    pub fn new(x: u64, y: u64) -> Self{
        Self { x, y }
    }
    pub fn extend(self, z: u64) ->DUVec3{
       duvec3(self.x, self.y, z)
    }
    pub fn truncate(self) -> u64 {
        self.x
    }
    ///create a vector where x and y equals `value`.
    pub fn splat(value: u64) -> Self{
        Self::new(value, value)
    }
    ///the length of the vector without being square rooted.
    pub fn length_squared(self) -> u64 {
        self.x*self.x + self.y*self.y
    }
    ///distance between two vectors without being square rooted.
    pub fn distance_squared(self, other:DUVec2) -> u64 {
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other:DUVec2) -> u64 {
        self.x * other.x + self.y * other.y
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: u64) ->DUVec2 {
        Self::new(self.x * scalar, self.y * scalar)
    }
}
impl AsUniformValue for DUVec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::UnsignedInt64Vec2([self.x, self.y])
    }
}
///create an unsigned double interger vector with an x and y coordinate.
pub const fn duvec2(x: u64, y: u64) -> DUVec2 {
   DUVec2 { x, y }
}
impl From<Vec2> for DUVec2 {
    fn from (value: Vec2) -> Self {
        duvec2(value.x as u64, value.y as u64)
    }
}
impl From<UVec2> for DUVec2 {
    fn from (value: UVec2) -> Self {
        duvec2(value.x as u64, value.y as u64)
    }
}
impl From<[u64; 2]> for DUVec2 {
    fn from(value: [u64; 2]) -> Self {
        duvec2(value[0], value[1])
    }
}
impl From<(u64, u64)> for DUVec2 {
    fn from(value: (u64, u64)) -> Self {
        duvec2(value.0, value.1)
    }
}
impl From<DUVec2> for [u64; 2] {
    fn from(value: DUVec2) -> Self {
        [value.x, value.y]
    }
}
impl From<DUVec2> for (u64, u64) {
    fn from(value: DUVec2) -> Self {
        (value.x, value.y)
    }
}
