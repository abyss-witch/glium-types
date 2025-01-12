use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;
use super::{vec3::Vec3, uvec2::{uvec2, UVec2}, uvec4::{uvec4, UVec4}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign)]
///an unsigned interger vector made from a x, y and z coordinate.
pub struct UVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32
}
impl UVec3 {
    ///a zero vector
    pub const ZERO: Self = uvec3(0, 0, 0);
    ///a vector full of ones
    pub const ONE: Self = uvec3(1, 1, 1);
    ///the x axis
    pub const X: Self = uvec3(1, 0, 0);
    ///the y axis
    pub const Y: Self = uvec3(0, 1, 0);
    ///the z axis
    pub const Z: Self = uvec3(0, 0, 1);

    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
    pub fn extend(self, w: u32) -> UVec4{
        uvec4(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> UVec2{
        uvec2(self.x, self.y)
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: u32) -> Self{
        Self::new(value, value, value)
    }

    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> u32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: UVec3) -> u32 {
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: UVec3) -> u32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors. the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{UVec3, uvec3};
    /// let x = uvec3(1, 0, 0);
    /// let y = uvec3(0, 1, 0);
    /// let z = uvec3(0, 0, 1);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(&self, other: UVec3) -> UVec3{
        uvec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: u32) -> UVec3{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl AsUniformValue for UVec3{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
       glium::uniforms::UniformValue::UnsignedIntVec3([self.x, self.y, self.z])
    }
}
impl From<Vec3> for UVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x as u32, y: value.y as u32, z: value.z as u32 }
    }
}
impl From<(u32, u32, u32)> for UVec3 {
    fn from(value: (u32, u32, u32)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
impl From<[u32; 3]> for UVec3 {
    fn from(value: [u32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
impl From<UVec3> for [u32; 3] {
    fn from(value: UVec3) -> Self {
        [value.x, value.y, value.z]
    }
}
impl From<UVec3> for (u32, u32, u32) {
    fn from(value: UVec3) -> Self {
        (value.x, value.y, value.z)
    }
}
///create an unsigned interger vector with an x, y and z coordinate.
pub const fn uvec3(x: u32, y: u32, z: u32) -> UVec3{
    UVec3 { x, y, z }
}
