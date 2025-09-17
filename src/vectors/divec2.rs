use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;

use super::{divec3::*, vec2::Vec2, ivec2::IVec2, bvec2::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// a double interger vector made from an x and y coordinate.
pub struct DIVec2 {
    pub x: i64,
    pub y: i64,
}
impl DIVec2{
    /// a zero vector
    pub const ZERO: Self = divec2(0, 0);
    /// a vector full of ones
    pub const ONE: Self = divec2(1, 1);
    /// the x axis
    pub const X: Self = divec2(1, 0);
    /// the y axis
    pub const Y: Self = divec2(0, 1);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub const fn extend(self, z: i64) -> DIVec3 { divec3(self.x, self.y, z) }
    pub const fn truncate(self) -> i64 { self.x }
    ///create a vector where x and y equals `value`.
    pub const fn splat(value: i64) -> Self { Self::new(value, value) }

    /// the length of the vector without being square rooted.
    pub fn length_squared(self) -> i64 {
        self.x*self.x + self.y*self.y
    }
    /// distance between two vectors without being square rooted.
    pub fn distance_squared(self, other: DIVec2) -> i64 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DIVec2) -> i64 {
        self.x * other.x + self.y * other.y
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: i64) -> DIVec2 {
        Self::new(self.x * scalar, self.y * scalar)
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
impl std::ops::Mul<DIVec2> for i64 {
    fn mul(self, rhs: DIVec2) -> Self::Output { rhs * self }
    type Output = DIVec2;
}
impl std::ops::Mul<i64> for DIVec2 {
    fn mul(self, rhs: i64) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i64> for DIVec2 { fn mul_assign(&mut self, rhs: i64) { *self = *self * rhs } }
impl std::ops::Div<DIVec2> for i64 {
    fn div(self, rhs: DIVec2) -> Self::Output { DIVec2::splat(self) / rhs }
    type Output = DIVec2;
}
impl std::ops::Div<i64> for DIVec2 {
    fn div(self, rhs: i64) -> Self::Output { self / DIVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i64> for DIVec2 { fn div_assign(&mut self, rhs: i64) { *self = *self / rhs } }

impl std::ops::Rem<DIVec2> for i64 {
    fn rem(self, rhs: DIVec2) -> Self::Output { DIVec2::splat(self) % rhs }
    type Output = DIVec2;
}
impl std::ops::Rem<i64> for DIVec2 {
    fn rem(self, rhs: i64) -> Self::Output { self % DIVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i64> for DIVec2 { fn rem_assign(&mut self, rhs: i64) { *self = *self % rhs } }
impl AsUniformValue for DIVec2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Int64Vec2([self.x, self.y])
    }
}
/// create an double interger vector with an x and y coordinate.
pub const fn divec2(x: i64, y: i64) -> DIVec2 {
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
impl From<DIVec2> for [i64; 2] {
    fn from(value: DIVec2) -> Self {
        [value.x, value.y]
    }
}
impl From<DIVec2> for (i64, i64) {
    fn from(value: DIVec2) -> Self {
        (value.x, value.y)
    }
}
