use glium::uniforms::AsUniformValue;

use crate::prelude::Mat4;

use super::vec3::{vec3, Vec3};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///a vector made from and x y and z coordinate.
pub struct Vec4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl Vec4{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self{
        Self { x, y, z, w }
    }
    ///create a vector where x, y and z equals value.
    pub fn splat(value: f32) -> Self{
        Self::new(value, value, value, value)
    }
    pub fn truncate(&self) -> Vec3{
        vec3(self.x, self.y, self.z)
    }
    ///the length of the vector before being square rooted.
    pub fn length_squared(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///length of the vector.
    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(&self, other: Vec4) -> f32{
        (*self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(&self, other: Vec4) -> f32{
        (*self - other).length()
    }
    ///get the dot profuct of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(&self, other: Vec4) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///multiplies each value by the scalar.
    pub fn scale(&self, scalar: f32) -> Vec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns vec3 of zeros
    pub fn normalise(&self) -> Self{
        let length = self.length();
        if length == 0.0 { return vec4(0.0, 0.0, 0.0, 0.0); }
        self.scale(1.0 / length)
    }
    ///transforms vector by the matrix
    pub fn transform(&self, matrix: Mat4) -> Vec4{
        let a: Vec4 = matrix.row(0).into();
        let b: Vec4 = matrix.row(1).into();
        let c: Vec4 = matrix.row(2).into();
        let d: Vec4 = matrix.row(3).into();
        vec4(a.dot(*self), b.dot(*self), c.dot(*self), d.dot(*self))
    }
}
impl AsUniformValue for Vec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec4([self.x, self.y, self.z, self.w])
    }
}
impl std::ops::Add for Vec4{
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
    type Output = Self;
}
impl std::ops::Sub for Vec4{
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
    type Output = Self;
}
impl std::ops::Mul for Vec4{
    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
    type Output = Self;
}
impl std::ops::Div for Vec4{
    fn div(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
    type Output = Self;
}
impl std::ops::Neg for Vec4{
    fn neg(self) -> Self::Output {
        vec4(-self.x, -self.y, -self.z, -self.w)
    }
    type Output = Self;
}
impl std::ops::AddAssign for Vec4{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign for Vec4{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign for Vec4{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl std::ops::DivAssign for Vec4{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}

///create a vector with an x, y and z coordinate.
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4{
    Vec4 { x, y, z, w }
}