use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, uvec3::*, bvec4::*};
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign
)]
/// an unsigned interger vector made from a x, y, z and w coordinate.
pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32
}
impl UVec4 {
    /// a zero vector
    pub const ZERO: Self = uvec4(0, 0, 0, 0);
    /// a vector full of ones
    pub const ONE: Self = uvec4(1, 1, 1, 1);
    /// the x axis
    pub const X: Self = uvec4(1, 0, 0, 0);
    /// the y axis
    pub const Y: Self = uvec4(0, 1, 0, 0);
    /// the z axis
    pub const Z: Self = uvec4(0, 0, 1, 0);
    /// the w axis
    pub const W: Self = uvec4(0, 0, 0, 1);

    pub const fn new(x: u32, y: u32, z: u32, w: u32) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> UVec3 { uvec3(self.x, self.y, self.z) }
    /// create a vector where x, y and z equals `value`
    pub const fn splat(value: u32) -> Self { Self::new(value, value, value, value) }

    /// the length of the vector without being square rooted
    pub fn length_squared(self) -> u32 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    /// distance between two vectors before being square rooted
    pub fn distance_squared(self, other: UVec4) -> u32{
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors
    pub fn dot(self, other: UVec4) -> u32{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    /// multiplies each value by the scalar
    pub fn scale(self, scalar: u32) -> UVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
    /// returns whether the 2 components are equal
    pub fn eq(self, rhs: Self) -> BVec4 { bvec4(self.x == rhs.x, self.y == rhs.y, self.z == rhs.z, self.w == rhs.w) }
    /// returns whether the 1st components are less than the 2nd
    pub fn less(self, rhs: Self) -> BVec4 { bvec4(self.x < rhs.x, self.y < rhs.y, self.z < rhs.z, self.w < rhs.w) }
    /// returns whether the 1st components are more than the 2nd
    pub fn more(self, rhs: Self) -> BVec4 { bvec4(self.x > rhs.x, self.y > rhs.y, self.z > rhs.z, self.w > rhs.w) }
    /// returns whether the 1st components are less than or equal to the 2nd
    pub fn less_or_eq(self, rhs: Self) -> BVec4 {
        bvec4(self.x <= rhs.x, self.y <= rhs.y, self.z <= rhs.z, self.w <= rhs.w)
    }
    /// returns whether the 1st components are more than or equal to the 2nd
    pub fn more_or_eq(self, rhs: Self) -> BVec4 {
        bvec4(self.x >= rhs.x, self.y >= rhs.y, self.z >= rhs.z, self.w >= rhs.w)
    }
}
impl std::ops::Mul<UVec4> for u32 {
    fn mul(self, rhs: UVec4) -> Self::Output { rhs * self }
    type Output = UVec4;
}
impl std::ops::Mul<u32> for UVec4 {
    fn mul(self, rhs: u32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<u32> for UVec4 { fn mul_assign(&mut self, rhs: u32) { *self = *self * rhs } }
impl std::ops::Div<UVec4> for u32 {
    fn div(self, rhs: UVec4) -> Self::Output { UVec4::splat(self) / rhs }
    type Output = UVec4;
}
impl std::ops::Div<u32> for UVec4 {
    fn div(self, rhs: u32) -> Self::Output { self / UVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<u32> for UVec4 { fn div_assign(&mut self, rhs: u32) { *self = *self / rhs } }
impl std::ops::Rem<UVec4> for u32 {
    fn rem(self, rhs: UVec4) -> Self::Output { UVec4::splat(self) % rhs }
    type Output = UVec4;
}
impl std::ops::Rem<u32> for UVec4 {
    fn rem(self, rhs: u32) -> Self::Output { self % UVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<u32> for UVec4 { fn rem_assign(&mut self, rhs: u32) { *self = *self % rhs } }

impl AsUniformValue for UVec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::UnsignedIntVec4([self.x, self.y, self.z, self.w])
    }
}
impl From<Vec4> for UVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x as u32, y: value.y as u32, z: value.z as u32, w: value.w as u32 }
    }
}
impl From<(u32, u32, u32, u32)> for UVec4 {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<[u32; 4]> for UVec4 {
    fn from(value: [u32; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
impl From<UVec4> for [u32; 4] {
    fn from(value: UVec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<UVec4> for (u32, u32, u32, u32) {
    fn from(value: UVec4) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
/// create an unsigned interger vector with an x, y, z and w coordinate.
pub const fn uvec4(x: u32, y: u32, z: u32, w: u32) -> UVec4{
    UVec4 { x, y, z, w }
}
