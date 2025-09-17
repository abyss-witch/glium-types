use derive_cmp_ops::*;
use glium::uniforms::AsUniformValue;
use super::{vec3::Vec3, duvec2::*, uvec3::UVec3, duvec4::*, bvec3::*};
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpAdd, CmpAddAssign, CmpDiv,
    CmpDivAssign, CmpMul, CmpMulAssign, CmpRem, CmpRemAssign, CmpSub, CmpSubAssign
)]
/// an unsigned interger vector made from an x, y and z coordinate
pub struct DUVec3 {
    pub x: u64,
    pub y: u64,
    pub z: u64
}
impl DUVec3 {
    /// a zero vector
    pub const ZERO: Self = duvec3(0, 0, 0);
    /// a vector full of ones
    pub const ONE: Self = duvec3(1, 1, 1);
    /// the x axis
    pub const X: Self = duvec3(1, 0, 0);
    /// the y axis
    pub const Y: Self = duvec3(0, 1, 0);
    /// the z axis
    pub const Z: Self = duvec3(0, 0, 1);

    pub const fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }
    pub const fn extend(self, w: u64) -> DUVec4{
        duvec4(self.x, self.y, self.z, w)
    }
    pub const fn truncate(self) -> DUVec2{
        duvec2(self.x, self.y)
    }
    /// create a vector where x, y and z equals `value`
    pub const fn splat(value: u64) -> Self{
        Self::new(value, value, value)
    }

    /// the length of the vector before being square rooted
    pub fn length_squared(self) -> u64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// distance between two vectors before being square rooted
    pub fn distance_squared(self, other: DUVec3) -> u64 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors
    pub fn dot(self, other: DUVec3) -> u64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors.
    /// the output vector is not normalised
    /// ```
    /// use glium_types::vectors::{DUVec3, duvec3};
    /// let x = duvec3(1, 0, 0);
    /// let y = duvec3(0, 1, 0);
    /// let z = duvec3(0, 0, 1);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(self, other: DUVec3) -> DUVec3{
        duvec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    /// multiplies each value by the scalar
    pub fn scale(self, scalar: u64) -> DUVec3{
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
impl std::ops::Mul<DUVec3> for u64 {
    fn mul(self, rhs: DUVec3) -> Self::Output { rhs * self }
    type Output = DUVec3;
}
impl std::ops::Mul<u64> for DUVec3 {
    fn mul(self, rhs: u64) -> Self::Output { self.scale(rhs) }
    type Output = DUVec3;
}
impl std::ops::MulAssign<u64> for DUVec3 { fn mul_assign(&mut self, rhs: u64) { *self = *self * rhs } }
impl std::ops::Div<DUVec3> for u64 {
    fn div(self, rhs: DUVec3) -> Self::Output { DUVec3::splat(self) / rhs }
    type Output = DUVec3;
}
impl std::ops::Div<u64> for DUVec3 {
    fn div(self, rhs: u64) -> Self::Output { self / DUVec3::splat(rhs) }
    type Output = DUVec3;
}
impl std::ops::DivAssign<u64> for DUVec3 { fn div_assign(&mut self, rhs: u64) { *self = *self / rhs } }

impl AsUniformValue for DUVec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
       glium::uniforms::UniformValue::UnsignedInt64Vec3([self.x, self.y, self.z])
    }
}
impl From<Vec3> for DUVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x as u64, y: value.y as u64, z: value.z as u64 }
    }
}
impl From<UVec3> for DUVec3 {
    fn from(value: UVec3) -> Self {
        Self { x: value.x as u64, y: value.y as u64, z: value.z as u64 }
    }
}
impl From<(u64, u64, u64)> for DUVec3 {
    fn from(value: (u64, u64, u64)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
impl From<[u64; 3]> for DUVec3 {
    fn from(value: [u64; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
impl From<DUVec3> for [u64; 3] {
    fn from(value: DUVec3) -> Self {
        [value.x, value.y, value.z]
    }
}
impl From<DUVec3> for (u64, u64, u64) {
    fn from(value: DUVec3) -> Self {
        (value.x, value.y, value.z)
    }
}
/// create an unsigned interger vector with an x, y and z coordinate.
pub const fn duvec3(x: u64, y: u64, z: u64) -> DUVec3 {
    DUVec3 { x, y, z }
}
