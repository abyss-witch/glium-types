use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;

use super::{divec3::{divec3, DIVec3}, vec2::Vec2, ivec2::IVec2};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
///an double interger vector made from a x and y coordinate.
pub struct DIVec2{
    pub x: i64,
    pub y: i64,
}
impl DIVec2{
    ///a zero vector
    pub const ZERO: Self = divec2(0, 0);
    ///a vector full of ones
    pub const ONE: Self = divec2(1, 1);
    ///the x axis
    pub const X: Self = divec2(1, 0);
    ///the y axis
    pub const Y: Self = divec2(0, 1);

    pub fn new(x: i64, y: i64) -> Self{
        Self { x, y }
    }
    pub fn extend(self, z: i64) -> DIVec3{
       divec3(self.x, self.y, z)
    }
    pub fn truncate(self) -> i64 {
        self.x
    }
    ///create a vector where x and y equals `value`.
    pub fn splat(value: i64) -> Self{
        Self::new(value, value)
    }
    ///the length of the vector without being square rooted.
    pub fn length_squared(self) -> i64 {
        self.x*self.x + self.y*self.y
    }
    ///distance between two vectors without being square rooted.
    pub fn distance_squared(self, other: DIVec2) -> i64 {
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DIVec2) -> i64 {
        self.x * other.x + self.y * other.y
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: i64) -> DIVec2 {
        Self::new(self.x * scalar, self.y * scalar)
    }
}
impl AsUniformValue for DIVec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Int64Vec2([self.x, self.y])
    }
}
///create an double interger vector with an x and y coordinate.
pub const fn divec2(x: i64, y: i64) -> DIVec2{
    DIVec2 { x, y }
}
impl From<Vec2> for DIVec2 {
    fn from (value: Vec2) -> Self {
        divec2(value.x as i64, value.y as i64)
    }
}
impl From<IVec2> for DIVec2 {
    fn from (value: IVec2) -> Self {
        divec2(value.x as i64, value.y as i64)
    }
}
impl From<[i64; 2]> for DIVec2 {
    fn from(value: [i64; 2]) -> Self {
        divec2(value[0], value[1])
    }
}
impl From<(i64, i64)> for DIVec2 {
    fn from(value: (i64, i64)) -> Self {
        divec2(value.0, value.1)
    }
}
