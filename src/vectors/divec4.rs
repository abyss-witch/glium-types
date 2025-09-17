use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, divec3::*, bvec4::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// a double interger vector made from a x, y, z and w coordinate.
pub struct DIVec4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64
}
impl DIVec4 {
    /// a zero vector
    pub const ZERO: Self = divec4(0, 0, 0, 0);
    /// a vector full of ones
    pub const ONE: Self = divec4(1, 1, 1, 1);
    /// the x axis
    pub const X: Self = divec4(1, 0, 0, 0);
    /// the y axis
    pub const Y: Self = divec4(0, 1, 0, 0);
    /// the z axis
    pub const Z: Self = divec4(0, 0, 1, 0);
    /// the w axis
    pub const W: Self = divec4(0, 0, 0, 1);

    pub const fn new(x: i64, y: i64, z: i64, w: i64) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> DIVec3{ divec3(self.x, self.y, self.z) }
    /// create a vector where x, y and z equals `value`
    pub const fn splat(value: i64) -> Self{ Self::new(value, value, value, value) }
    
    /// the length of the vector without being square rooted.
    pub fn length_squared(self) -> i64 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DIVec4) -> i64{
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DIVec4) -> i64{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: i64) -> DIVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
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
impl std::ops::Mul<DIVec4> for i64 {
    fn mul(self, rhs: DIVec4) -> Self::Output { rhs * self }
    type Output = DIVec4;
}
impl std::ops::Mul<i64> for DIVec4 {
    fn mul(self, rhs: i64) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i64> for DIVec4 { fn mul_assign(&mut self, rhs: i64) { *self = *self * rhs } }
impl std::ops::Div<DIVec4> for i64 {
    fn div(self, rhs: DIVec4) -> Self::Output { DIVec4::splat(self) / rhs }
    type Output = DIVec4;
}
impl std::ops::Div<i64> for DIVec4 {
    fn div(self, rhs: i64) -> Self::Output { self / DIVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i64> for DIVec4 { fn div_assign(&mut self, rhs: i64) { *self = *self / rhs } }
impl std::ops::Rem<DIVec4> for i64 {
    fn rem(self, rhs: DIVec4) -> Self::Output { DIVec4::splat(self) % rhs }
    type Output = DIVec4;
}
impl std::ops::Rem<i64> for DIVec4 {
    fn rem(self, rhs: i64) -> Self::Output { self % DIVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i64> for DIVec4 { fn rem_assign(&mut self, rhs: i64) { *self = *self % rhs } }

impl AsUniformValue for DIVec4 {
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
/// create an interger vector with an x, y, z and w coordinate.
pub const fn divec4(x: i64, y: i64, z: i64, w: i64) -> DIVec4 {
    DIVec4 { x, y, z, w }
}
