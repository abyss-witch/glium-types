use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::prelude::Mat4;
use super::{vec3::{vec3, Vec3}, bvec4::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a vector made from a x, y, z and w coordinate.
pub struct Vec4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl Vec4{
    ///a zero vector
    pub const ZERO: Self = vec4(0.0, 0.0, 0.0, 0.0);
    ///a vector full of ones
    pub const ONE: Self = vec4(1.0, 1.0, 1.0, 1.0);
    ///the x axis
    pub const X: Self = vec4(1.0, 0.0, 0.0, 0.0);
    ///the y axis
    pub const Y: Self = vec4(0.0, 1.0, 0.0, 0.0);
    ///the z axis
    pub const Z: Self = vec4(0.0, 0.0, 1.0, 0.0);
    ///the w axis
    pub const W: Self = vec4(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> Vec3 { vec3(self.x, self.y, self.z) }
    ///create a vector where x, y and z equals `value`.
    pub const fn splat(value: f32) -> Self{ Self::new(value, value, value, value) }

    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///length of the vector.
    pub fn length(self) -> f32{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: Vec4) -> f32{
        (self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(self, other: Vec4) -> f32{
        (self - other).length()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: Vec4) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: f32) -> Vec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns vec4 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return vec4(0.0, 0.0, 0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(self, matrix: Mat4) -> Vec4{
        let a: Vec4 = matrix.row(0).into();
        let b: Vec4 = matrix.row(1).into();
        let c: Vec4 = matrix.row(2).into();
        let d: Vec4 = matrix.row(3).into();
        vec4(a.dot(self), b.dot(self), c.dot(self), d.dot(self))
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
impl std::ops::Mul<Vec4> for f32 {
    fn mul(self, rhs: Vec4) -> Self::Output { rhs * self }
    type Output = Vec4;
}
impl std::ops::Mul<f32> for Vec4 {
    fn mul(self, rhs: f32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<f32> for Vec4 { fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs } }
impl std::ops::Div<Vec4> for f32 {
    fn div(self, rhs: Vec4) -> Self::Output { Vec4::splat(self) / rhs }
    type Output = Vec4;
}
impl std::ops::Div<f32> for Vec4 {
    fn div(self, rhs: f32) -> Self::Output { self.scale(1.0/rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<f32> for Vec4 { fn div_assign(&mut self, rhs: f32) { *self = *self / rhs } }
impl std::ops::Rem<Vec4> for f32 {
    fn rem(self, rhs: Vec4) -> Self::Output { Vec4::splat(self) % rhs }
    type Output = Vec4;
}
impl std::ops::Rem<f32> for Vec4 {
    fn rem(self, rhs: f32) -> Self::Output { self % Vec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<f32> for Vec4 { fn rem_assign(&mut self, rhs: f32) { *self = *self % rhs } }
impl AsUniformValue for Vec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec4([self.x, self.y, self.z, self.w])
    }
}
impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<Vec4> for [f32; 4] {
    fn from(value: Vec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<Vec4> for (f32, f32, f32, f32) {
    fn from(value: Vec4) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
///create a vector with an x, y, z and w coordinate.
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4{
    Vec4 { x, y, z, w }
}
