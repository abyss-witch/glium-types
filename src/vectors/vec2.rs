use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::matrices::Mat2;

use super::{vec3::{vec3, Vec3}, bvec2::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
/// a vector made from a x and y coordinate.
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2{
    /// a zero vector
    pub const ZERO: Self = vec2(0.0, 0.0);
    /// a vector full of ones
    pub const ONE: Self = vec2(1.0, 1.0);
    /// the x axis
    pub const X: Self = vec2(1.0, 0.0);
    /// the y axis
    pub const Y: Self = vec2(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self{ Self { x, y } }
    pub const fn extend(self, z: f32) -> Vec3{ vec3(self.x, self.y, z) }
    pub const fn truncate(self) -> f32 { self.x }
    /// create a vector where x and y equals `value`.
    pub const fn splat(value: f32) -> Self { Self::new(value, value) }

    /// the length of the vector before being square rooted.
    pub fn length_squared(self) -> f32{
        self.x*self.x + self.y*self.y
    }
    /// length of the vector.
    pub fn length(self) -> f32{
        self.length_squared().sqrt()
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: Vec2) -> f32{
        (self - other).length_squared()
    }
    /// distance between two vectors.
    pub fn distance(self, other: Vec2) -> f32{
        (self - other).length()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: Vec2) -> f32{
        self.x * other.x + self.y * other.y
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: f32) -> Vec2{
        Self::new(self.x * scalar, self.y * scalar)
    }
    /// makes the length of the vector equal to 1. on fail returns vec2 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return vec2(0.0, 0.0); }
        self.scale(1.0 / length)
    }
    /// transforms vector by the matrix
    pub fn transform(self, matrix: Mat2) -> Vec2{
        let a: Vec2 = matrix.row(0).into();
        let b: Vec2 = matrix.row(1).into();
        vec2(a.dot(self), b.dot(self))
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
impl std::ops::Mul<Vec2> for f32 {
    fn mul(self, rhs: Vec2) -> Self::Output { rhs * self }
    type Output = Vec2;
}
impl std::ops::Mul<f32> for Vec2 {
    fn mul(self, rhs: f32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<f32> for Vec2 { fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs } }
impl std::ops::Div<Vec2> for f32 {
    fn div(self, rhs: Vec2) -> Self::Output { Vec2::splat(self) / rhs }
    type Output = Vec2;
}
impl std::ops::Div<f32> for Vec2 {
    fn div(self, rhs: f32) -> Self::Output { self.scale(1.0 / rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<f32> for Vec2 { fn div_assign(&mut self, rhs: f32) { *self = *self / rhs } }
impl std::ops::Rem<Vec2> for f32 {
    fn rem(self, rhs: Vec2) -> Self::Output { Vec2::splat(self) % rhs }
    type Output = Vec2;
}
impl std::ops::Rem<f32> for Vec2 {
    fn rem(self, rhs: f32) -> Self::Output { self % Vec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<f32> for Vec2 { fn rem_assign(&mut self, rhs: f32) { *self = *self % rhs } }

impl AsUniformValue for Vec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec2([self.x, self.y])
    }
}
/// create a vector with an x and y coordinate.
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}
impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        vec2(value[0], value[1])
    }
}
impl From<(f32, f32)> for Vec2 {
    fn from(value: (f32, f32)) -> Self {
        vec2(value.0, value.1)
    }
}
impl From<Vec2> for [f32; 2] {
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}
impl From<Vec2> for (f32, f32) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}
