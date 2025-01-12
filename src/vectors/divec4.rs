use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, divec3::{divec3, DIVec3}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
///an interger vector made from a x, y, z and w coordinate.
pub struct DIVec4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64
}
impl DIVec4 {
    ///a zero vector
    pub const ZERO: Self = divec4(0, 0, 0, 0);
    ///a vector full of ones
    pub const ONE: Self = divec4(1, 1, 1, 1);
    ///the x axis
    pub const X: Self = divec4(1, 0, 0, 0);
    ///the y axis
    pub const Y: Self = divec4(0, 1, 0, 0);
    ///the z axis
    pub const Z: Self = divec4(0, 0, 1, 0);
    ///the w axis
    pub const W: Self = divec4(0, 0, 0, 1);

    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self{
        Self { x, y, z, w }
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: i64) -> Self{
        Self::new(value, value, value, value)
    }
    pub fn truncate(&self) -> DIVec3{
        divec3(self.x, self.y, self.z)
    }
    ///the length of the vector without being square rooted.
    pub fn length_squared(self) -> i64 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DIVec4) -> i64{
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DIVec4) -> i64{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: i64) -> DIVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}
impl AsUniformValue for DIVec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Int64Vec4([self.x, self.y, self.z, self.w])
    }
}
impl From<Vec4> for DIVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x as i64, y: value.y as i64, z: value.z as i64, w: value.w as i64 }
    }
}
impl From<(i64, i64, i64, i64)> for DIVec4 {
    fn from(value: (i64, i64, i64, i64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<[i64; 4]> for DIVec4 {
    fn from(value: [i64; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
impl From<DIVec4> for [i64; 4] {
    fn from(value: DIVec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<DIVec4> for (i64, i64, i64, i64) {
    fn from(value: DIVec4) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
///create an interger vector with an x, y, z and w coordinate.
pub const fn divec4(x: i64, y: i64, z: i64, w: i64) -> DIVec4{
    DIVec4 { x, y, z, w }
}
