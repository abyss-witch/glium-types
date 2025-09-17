use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec3::Vec3, divec2::{divec2, DIVec2}, divec4::{divec4, DIVec4}, bvec3::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// a double interger vector made from a x, y and z coordinate.
pub struct DIVec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64
}
impl DIVec3 {
    /// a zero vector
    pub const ZERO: Self = divec3(0, 0, 0);
    /// a vector full of ones
    pub const ONE: Self = divec3(1, 1, 1);
    /// the x axis
    pub const X: Self = divec3(1, 0, 0);
    /// the y axis
    pub const Y: Self = divec3(0, 1, 0);
    /// the z axis
    pub const Z: Self = divec3(0, 0, 1);

    pub const fn new(x: i64, y: i64, z: i64) -> Self { Self { x, y, z } }
    pub const fn extend(self, w: i64) -> DIVec4 { divec4(self.x, self.y, self.z, w) }
    pub const fn truncate(self) -> DIVec2 { divec2(self.x, self.y) }
    /// create a vector where x, y and z equals `value`.
    pub const fn splat(value: i64) -> Self{ Self::new(value, value, value) }

    /// the length of the vector before being square rooted.
    pub fn length_squared(self) -> i64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: DIVec3) -> i64 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: DIVec3) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// get the cross product of 2 vectors. equal to the vector that is perpendicular to both
    /// input vectors. the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{DIVec3, divec3};
    /// let x = divec3(1, 0, 0);
    /// let y = divec3(0, 1, 0);
    /// let z = divec3(0, 0, 1);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(self, other: DIVec3) -> DIVec3 {
        divec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: i64) -> DIVec3 {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    /// returns whether the 2 components are equal
    pub fn eq(self, rhs: Self) -> BVec3 { bvec3(self.x == rhs.x, self.y == rhs.y, self.z == rhs.z) }
    /// returns whether the 1st components are less than the 2nd
    pub fn less(self, rhs: Self) -> BVec3 { bvec3(self.x < rhs.x, self.y < rhs.y, self.z < rhs.z) }
    /// returns whether the 1st components are more than the 2nd
    pub fn more(self, rhs: Self) -> BVec3 { bvec3(self.x > rhs.x, self.y > rhs.y, self.z > rhs.z) }
    /// returns whether the 1st components are less than or equal to the 2nd
    pub fn less_or_eq(self, rhs: Self) -> BVec3 { bvec3(self.x <= rhs.x, self.y <= rhs.y, self.z <= rhs.z) }
    /// returns whether the 1st components are more than or equal to the 2nd
    pub fn more_or_eq(self, rhs: Self) -> BVec3 { bvec3(self.x >= rhs.x, self.y >= rhs.y, self.z >= rhs.z) }
}
impl std::ops::Mul<DIVec3> for i64 {
    fn mul(self, rhs: DIVec3) -> Self::Output { rhs * self }
    type Output = DIVec3;
}
impl std::ops::Mul<i64> for DIVec3 {
    fn mul(self, rhs: i64) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i64> for DIVec3 { fn mul_assign(&mut self, rhs: i64) { *self = *self * rhs } }
impl std::ops::Div<DIVec3> for i64 {
    fn div(self, rhs: DIVec3) -> Self::Output { DIVec3::splat(self) / rhs }
    type Output = DIVec3;
}
impl std::ops::Div<i64> for DIVec3 {
    fn div(self, rhs: i64) -> Self::Output { self / DIVec3::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i64> for DIVec3 { fn div_assign(&mut self, rhs: i64) { *self = *self / rhs } }
impl std::ops::Rem<DIVec3> for i64 {
    fn rem(self, rhs: DIVec3) -> Self::Output { DIVec3::splat(self) % rhs }
    type Output = DIVec3;
}
impl std::ops::Rem<i64> for DIVec3 {
    fn rem(self, rhs: i64) -> Self::Output { self % DIVec3::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i64> for DIVec3 { fn rem_assign(&mut self, rhs: i64) { *self = *self % rhs } }

impl AsUniformValue for DIVec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Int64Vec3([self.x, self.y, self.z])
    }
}
impl From<Vec3> for DIVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x as i64, y: value.y as i64, z: value.z as i64 }
    }
}
impl From<(i64, i64, i64)> for DIVec3 {
    fn from(value: (i64, i64, i64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
impl From<[i64; 3]> for DIVec3 {
    fn from(value: [i64; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
/// create an double interger vector with an x, y and z coordinate.
pub const fn divec3(x: i64, y: i64, z: i64) -> DIVec3 {
    DIVec3 { x, y, z }
}
