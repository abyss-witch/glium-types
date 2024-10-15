use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, uvec4::UVec4, duvec3::{duvec3, DUVec3}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign)]
///an unsigned double interger vector made from a x, y, z and w coordinate.
pub struct DUVec4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64
}
impl DUVec4 {
    ///a zero vector
    pub const ZERO: Self = duvec4(0, 0, 0, 0);
    ///a vector full of ones
    pub const ONE: Self = duvec4(1, 1, 1, 1);
    ///the x axis
    pub const X: Self = duvec4(1, 0, 0, 0);
    ///the y axis
    pub const Y: Self = duvec4(0, 1, 0, 0);
    ///the z axis
    pub const Z: Self = duvec4(0, 0, 1, 0);
    ///the w axis
    pub const W: Self = duvec4(0, 0, 0, 1);

    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self{
        Self { x, y, z, w }
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: u64) -> Self{
        Self::new(value, value, value, value)
    }
    pub fn truncate(&self) -> DUVec3{
        duvec3(self.x, self.y, self.z)
    }
    ///the length of the vector without being square rooted.
    pub fn length_squared(self) -> u64 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DUVec4) -> u64{
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DUVec4) -> u64{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: u64) -> DUVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}
impl AsUniformValue for DUVec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::UnsignedInt64Vec4([self.x, self.y, self.z, self.w])
    }
}
impl From<Vec4> for DUVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x as u64, y: value.y as u64, z: value.z as u64, w: value.w as u64 }
    }
}
impl From<UVec4> for DUVec4 {
    fn from(value: UVec4) -> Self {
        Self { x: value.x as u64, y: value.y as u64, z: value.z as u64, w: value.w as u64 }
    }
}
impl From<(u64, u64, u64, u64)> for DUVec4 {
    fn from(value: (u64, u64, u64, u64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<[u64; 4]> for DUVec4 {
    fn from(value: [u64; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
///create an unsigned double interger vector with an x, y, z and w coordinate.
pub const fn duvec4(x: u64, y: u64, z: u64, w: u64) -> DUVec4{
    DUVec4 { x, y, z, w }
}
