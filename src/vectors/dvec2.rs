use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::matrices::Mat2;

use super::{dvec3::{dvec3, DVec3}, vec2::Vec2};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a vector made from a x and y coordinate.
pub struct DVec2{
    pub x: f64,
    pub y: f64,
}
impl DVec2{
    ///a zero vector
    pub const ZERO: Self = dvec2(0.0, 0.0);
    ///a vector full of ones
    pub const ONE: Self = dvec2(1.0, 1.0);
    ///the x axis
    pub const X: Self = dvec2(1.0, 0.0);
    ///the y axis
    pub const Y: Self = dvec2(0.0, 1.0);

    pub fn new(x: f64, y: f64) -> Self{
        Self { x, y }
    }
    pub fn extend(self, z: f64) -> DVec3{
       dvec3(self.x, self.y, z)
    }
    pub fn truncate(self) -> f64{
        self.x
    }
    ///create a vector where x and y equals `value`.
    pub fn splat(value: f64) -> Self{
        Self::new(value, value)
    }
    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> f64{
        self.x*self.x + self.y*self.y
    }
    ///length of the vector.
    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DVec2) -> f64{
        (self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(self, other: DVec2) -> f64{
        (self - other).length()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DVec2) -> f64{
        self.x * other.x + self.y * other.y
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: f64) -> DVec2{
        Self::new(self.x * scalar, self.y * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns dvec2 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return dvec2(0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(self, matrix: &Mat2) -> DVec2{
        let a: DVec2 = matrix.row(0).into();
        let b: DVec2 = matrix.row(1).into();
        dvec2(a.dot(self), b.dot(self))
    }
}
impl AsUniformValue for DVec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleVec2([self.x, self.y])
    }
}
///create a double vector with an x and y coordinate.
pub const fn dvec2(x: f64, y: f64) -> DVec2{
    DVec2 { x, y }
}
impl From<Vec2> for DVec2 {
    fn from (value: Vec2) -> Self {
        dvec2(value.x as f64, value.y as f64)
    }
}
impl From<[f64; 2]> for DVec2 {
    fn from(value: [f64; 2]) -> Self {
        dvec2(value[0], value[1])
    }
}
impl From<(f64, f64)> for DVec2 {
    fn from(value: (f64, f64)) -> Self {
        dvec2(value.0, value.1)
    }
}
impl From<[f32; 2]> for DVec2 {
    fn from(value: [f32; 2]) -> Self {
        dvec2(value[0] as f64, value[1] as f64)
    }
}
