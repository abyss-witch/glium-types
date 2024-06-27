use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::prelude::Mat4;
use super::{vec4::Vec4, dvec3::{dvec3, DVec3}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a double vector made from a x, y, z and w coordinate.
pub struct DVec4{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl DVec4{
    ///a zero vector
    pub const ZERO: Self = dvec4(0.0, 0.0, 0.0, 0.0);
    ///a vector full of ones
    pub const ONE: Self = dvec4(1.0, 1.0, 1.0, 1.0);
    ///the x axis
    pub const X: Self = dvec4(1.0, 0.0, 0.0, 0.0);
    ///the y axis
    pub const Y: Self = dvec4(0.0, 1.0, 0.0, 0.0);
    ///the z axis
    pub const Z: Self = dvec4(0.0, 0.0, 1.0, 0.0);
    ///the w axis
    pub const W: Self = dvec4(0.0, 0.0, 0.0, 1.0);

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self{
        Self { x, y, z, w }
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: f64) -> Self{
        Self::new(value, value, value, value)
    }
    pub fn truncate(&self) -> DVec3{
        dvec3(self.x, self.y, self.z)
    }
    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> f64{
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///length of the vector.
    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DVec4) -> f64{
        (self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(self, other: DVec4) -> f64{
        (self - other).length()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DVec4) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: f64) -> DVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
    ///makes the length of the vector equal to 1.0 on fail returns dvec4 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return dvec4(0.0, 0.0, 0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(self, matrix: &Mat4) -> DVec4{
        let a: DVec4 = matrix.row(0).into();
        let b: DVec4 = matrix.row(1).into();
        let c: DVec4 = matrix.row(2).into();
        let d: DVec4 = matrix.row(3).into();
        dvec4(a.dot(self), b.dot(self), c.dot(self), d.dot(self))
    }
}
impl AsUniformValue for DVec4{
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
///create a double vector with an x, y, z and w coordinate.
pub const fn dvec4(x: f64, y: f64, z: f64, w: f64) -> DVec4{
    DVec4 { x, y, z, w }
}
