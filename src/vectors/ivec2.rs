use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;

use super::{ivec3::*, vec2::Vec2, bvec2::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// an interger vector made from a x and y coordinate.
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    /// a zero vector
    pub const ZERO: Self = ivec2(0, 0);
    /// a vector full of ones
    pub const ONE: Self = ivec2(1, 1);
    /// the x axis
    pub const X: Self = ivec2(1, 0);
    /// the y axis
    pub const Y: Self = ivec2(0, 1);

    pub const fn new(x: i32, y: i32) -> Self { Self { x, y } }
    pub const fn extend(self, z: i32) -> IVec3 { ivec3(self.x, self.y, z) }
    pub const fn truncate(self) -> i32 { self.x }

    /// create a vector where x and y equals `value`
    pub fn splat(value: i32) -> Self {
        Self::new(value, value)
    }
    /// the length of the vector without being square rooted
    pub fn length_squared(self) -> i32 {
        self.x*self.x + self.y*self.y
    }
    /// distance between two vectors without being square rooted
    pub fn distance_squared(self, other: IVec2) -> i32 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors
    pub fn dot(self, other: IVec2) -> i32 {
        self.x * other.x + self.y * other.y
    }
    /// multiplies each value by the scalar
    pub fn scale(self, scalar: i32) -> IVec2 {
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
impl std::ops::Mul<IVec2> for i32 {
    fn mul(self, rhs: IVec2) -> Self::Output { rhs * self }
    type Output = IVec2;
}
impl std::ops::Mul<i32> for IVec2 {
    fn mul(self, rhs: i32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i32> for IVec2 { fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs } }
impl std::ops::Div<IVec2> for i32 {
    fn div(self, rhs: IVec2) -> Self::Output { IVec2::splat(self) / rhs }
    type Output = IVec2;
}
impl std::ops::Div<i32> for IVec2 {
    fn div(self, rhs: i32) -> Self::Output { self / IVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i32> for IVec2 { fn div_assign(&mut self, rhs: i32) { *self = *self / rhs } }
impl std::ops::Rem<IVec2> for i32 {
    fn rem(self, rhs: IVec2) -> Self::Output { IVec2::splat(self) % rhs }
    type Output = IVec2;
}
impl std::ops::Rem<i32> for IVec2 {
    fn rem(self, rhs: i32) -> Self::Output { self % IVec2::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i32> for IVec2 { fn rem_assign(&mut self, rhs: i32) { *self = *self % rhs } }

impl AsUniformValue for IVec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::IntVec2([self.x, self.y])
    }
}
/// create an interger vector with an x and y coordinate.
pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}
impl From<Vec2> for IVec2 {
    fn from (value: Vec2) -> Self {
        ivec2(value.x as i32, value.y as i32)
    }
}
impl From<[i32; 2]> for IVec2 {
    fn from(value: [i32; 2]) -> Self {
        ivec2(value[0], value[1])
    }
}
impl From<(i32, i32)> for IVec2 {
    fn from(value: (i32, i32)) -> Self {
        ivec2(value.0, value.1)
    }
}
impl From<IVec2> for [i32; 2] {
    fn from(value: IVec2) -> Self {
        [value.x, value.y]
    }
}
impl From<IVec2> for (i32, i32) {
    fn from(value: IVec2) -> Self {
        (value.x, value.y)
    }
}
