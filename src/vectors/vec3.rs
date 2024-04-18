use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use crate::prelude::Mat3;
use super::{vec2::{vec2, Vec2}, vec4::{vec4, Vec4}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a vector made from a x, y and z coordinate.
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vec3{
    ///a zero vector
    pub const ZERO: Self = vec3(0.0, 0.0, 0.0);
    ///a vector full of ones
    pub const ONE: Self = vec3(1.0, 1.0, 1.0);
    ///the x axis
    pub const X: Self = vec3(1.0, 0.0, 0.0);
    ///the y axis
    pub const Y: Self = vec3(0.0, 1.0, 0.0);
    ///the z axis
    pub const Z: Self = vec3(0.0, 0.0, 1.0);

    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self { x, y, z }
    }
    pub fn extend(self, w: f32) -> Vec4{
        vec4(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> Vec2{
        vec2(self.x, self.y)
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: f32) -> Self{
        Self::new(value, value, value)
    }

    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    ///length of the vector.
    pub fn length(self) -> f32{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: Vec3) -> f32{
        (self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(self, other: Vec3) -> f32{
        (self - other).length()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: Vec3) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors. the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{Vec3, vec3};
    /// let x = vec3(1.0, 0.0, 0.0);
    /// let y = vec3(0.0, 1.0, 0.0);
    /// let z = vec3(0.0, 0.0, 1.0);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(&self, other: Vec3) -> Vec3{
        vec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: f32) -> Vec3{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns vec3 of zeros
    pub fn normalise(self) -> Self{
        let length = self.length();
        if length == 0.0 { return vec3(0.0, 0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(self, matrix: &Mat3) -> Self{
        let a: Vec3 = matrix.row(0).into();
        let b: Vec3 = matrix.row(1).into();
        let c: Vec3 = matrix.row(2).into();
        vec3(a.dot(self), b.dot(self), c.dot(self))
    }
}
impl AsUniformValue for Vec3{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec3([self.x, self.y, self.z])
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
///create a vector with an x, y and z coordinate.
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3{
    Vec3 { x, y, z }
}