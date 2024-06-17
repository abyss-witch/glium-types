use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::prelude::Mat3;
use super::{vec3::Vec3, dvec2::{dvec2, DVec2}, dvec4::{dvec4, DVec4}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a vector made from a x, y and z coordinate.
pub struct DVec3{
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl DVec3{
    ///a zero vector
    pub const ZERO: Self = dvec3(0.0, 0.0, 0.0);
    ///a vector full of ones
    pub const ONE: Self = dvec3(1.0, 1.0, 1.0);
    ///the x axis
    pub const X: Self = dvec3(1.0, 0.0, 0.0);
    ///the y axis
    pub const Y: Self = dvec3(0.0, 1.0, 0.0);
    ///the z axis
    pub const Z: Self = dvec3(0.0, 0.0, 1.0);

    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self { x, y, z }
    }
    pub fn extend(self, w: f64) -> DVec4{
        dvec4(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> DVec2{
        dvec2(self.x, self.y)
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: f64) -> Self{
        Self::new(value, value, value)
    }

    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> f64{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    ///length of the vector.
    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DVec3) -> f64{
        (self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(self, other: DVec3) -> f64{
        (self - other).length()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DVec3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors. the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{DVec3, dvec3};
    /// let x = dvec3(1.0, 0.0, 0.0);
    /// let y = dvec3(0.0, 1.0, 0.0);
    /// let z = dvec3(0.0, 0.0, 1.0);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(&self, other: DVec3) -> DVec3{
        dvec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: f64) -> DVec3{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns dvec3 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return dvec3(0.0, 0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(self, matrix: &Mat3) -> Self{
        let a: DVec3 = matrix.row(0).into();
        let b: DVec3 = matrix.row(1).into();
        let c: DVec3 = matrix.row(2).into();
        dvec3(a.dot(self), b.dot(self), c.dot(self))
    }
}
impl AsUniformValue for DVec3{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleVec3([self.x, self.y, self.z])
    }
}
impl From<Vec3> for DVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x as f64, y: value.y as f64, z: value.z as f64 }
    }
}
impl From<(f64, f64, f64)> for DVec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
impl From<[f64; 3]> for DVec3 {
    fn from(value: [f64; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
impl From<[f32; 3]> for DVec3 {
    fn from(value: [f32; 3]) -> Self {
        Self { x: value[0] as f64, y: value[1] as f64, z: value[2] as f64 }
    }
}
///create a double vector with an x, y and z coordinate.
pub const fn dvec3(x: f64, y: f64, z: f64) -> DVec3{
    DVec3 { x, y, z }
}
