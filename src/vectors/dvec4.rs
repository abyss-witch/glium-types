use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::matrices::DMat4;
use super::{vec4::Vec4, dvec3::{dvec3, DVec3}, bvec4::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
/// a double vector made from a x, y, z and w coordinate.
pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl DVec4 {
    /// a zero vector
    pub const ZERO: Self = dvec4(0.0, 0.0, 0.0, 0.0);
    /// a vector full of ones
    pub const ONE: Self = dvec4(1.0, 1.0, 1.0, 1.0);
    /// the x axis
    pub const X: Self = dvec4(1.0, 0.0, 0.0, 0.0);
    /// the y axis
    pub const Y: Self = dvec4(0.0, 1.0, 0.0, 0.0);
    /// the z axis
    pub const Z: Self = dvec4(0.0, 0.0, 1.0, 0.0);
    /// the w axis
    pub const W: Self = dvec4(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> DVec3 { dvec3(self.x, self.y, self.z) }
    /// create a vector where x, y and z equal `value`.
    pub const fn splat(value: f64) -> Self { Self::new(value, value, value, value) }

    /// the length of the vector before being square rooted.
    pub fn length_squared(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    /// length of the vector.
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DVec4) -> f64 {
        (self - other).length_squared()
    }
    /// distance between two vectors.
    pub fn distance(self, other: DVec4) -> f64 {
        (self - other).length()
    }
    /// get the dot product of 2 vectors. when vectors are normalised its equal to the
    /// cosign of the angle between the vectors
    pub fn dot(self, other: DVec4) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    /// multiplies each value by the scalar
    pub fn scale(self, scalar: f64) -> DVec4 {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
    /// makes the length of the vector equal to 1.0 on fail returns dvec4 of zeros
    pub fn normalise(self) -> Self {
        let length = self.length();
        if length == 0.0 { return DVec4::ZERO; }
        self.scale(1.0 / length)
    }
    /// transforms vector by the matrix. same as multiplying
    pub fn transform(self, matrix: DMat4) -> DVec4 {
        let a: DVec4 = matrix.row(0).into();
        let b: DVec4 = matrix.row(1).into();
        let c: DVec4 = matrix.row(2).into();
        let d: DVec4 = matrix.row(3).into();
        dvec4(a.dot(self), b.dot(self), c.dot(self), d.dot(self))
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
// const math
impl std::ops::Mul<DVec4> for f64 {
    fn mul(self, rhs: DVec4) -> Self::Output { rhs * self }
    type Output = DVec4;
}
impl std::ops::Mul<f64> for DVec4 {
    fn mul(self, rhs: f64) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<f64> for DVec4 { fn mul_assign(&mut self, rhs: f64) { *self = *self * rhs } }
impl std::ops::Div<DVec4> for f64 {
    fn div(self, rhs: DVec4) -> Self::Output { DVec4::splat(self) / rhs }
    type Output = DVec4;
}
impl std::ops::Div<f64> for DVec4 {
    fn div(self, rhs: f64) -> Self::Output { self.scale(1.0/rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<f64> for DVec4 { fn div_assign(&mut self, rhs: f64) { *self = *self / rhs } }
impl std::ops::Rem<DVec4> for f64 {
    fn rem(self, rhs: DVec4) -> Self::Output { DVec4::splat(self) % rhs }
    type Output = DVec4;
}
impl std::ops::Rem<f64> for DVec4 {
    fn rem(self, rhs: f64) -> Self::Output { self % DVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<f64> for DVec4 { fn rem_assign(&mut self, rhs: f64) { *self = *self % rhs } }

impl AsUniformValue for DVec4 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleVec4([self.x, self.y, self.z, self.w])
    }
}
impl From<Vec4> for DVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x as f64, y: value.y as f64, z: value.z as f64, w: value.w as f64 }
    }
}
impl From<(f64, f64, f64, f64)> for DVec4 {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<[f64; 4]> for DVec4 {
    fn from(value: [f64; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
impl From<[f32; 4]> for DVec4 {
    fn from(value: [f32; 4]) -> Self {
        Self { x: value[0] as f64, y: value[1] as f64, z: value[2] as f64, w: value[3] as f64 }
    }
}
impl From<DVec4> for [f64; 4] {
    fn from(value: DVec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<DVec4> for (f64, f64, f64, f64) {
    fn from(value: DVec4) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
/// creates a double vector with an x, y, z and w coordinate.
pub const fn dvec4(x: f64, y: f64, z: f64, w: f64) -> DVec4 { DVec4 { x, y, z, w } }
