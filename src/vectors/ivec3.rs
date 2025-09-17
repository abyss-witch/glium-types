use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec3::Vec3, ivec2::*, ivec4::*, bvec3::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// an interger vector made from a x, y and z coordinate.
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}
impl IVec3 {
    /// a zero vector
    pub const ZERO: Self = ivec3(0, 0, 0);
    /// a vector full of ones
    pub const ONE: Self = ivec3(1, 1, 1);
    ///the x axis
    pub const X: Self = ivec3(1, 0, 0);
    /// the y axis
    pub const Y: Self = ivec3(0, 1, 0);
    /// the z axis
    pub const Z: Self = ivec3(0, 0, 1);

    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
    pub const fn extend(self, w: i32) -> IVec4 { ivec4(self.x, self.y, self.z, w) }
    pub const fn truncate(self) -> IVec2 { ivec2(self.x, self.y) }
    /// create a vector where x, y and z equals `value`.
    pub const fn splat(value: i32) -> Self { Self::new(value, value, value) }

    /// the length of the vector before being square rooted.
    pub fn length_squared(self) -> i32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: IVec3) -> i32 {
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: IVec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors.
    /// the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{IVec3, ivec3};
    /// let x = ivec3(1, 0, 0);
    /// let y = ivec3(0, 1, 0);
    /// let z = ivec3(0, 0, 1);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(self, other: IVec3) -> IVec3 {
        ivec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: i32) -> IVec3 {
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
impl std::ops::Mul<IVec3> for i32 {
    fn mul(self, rhs: IVec3) -> Self::Output { rhs * self }
    type Output = IVec3;
}
impl std::ops::Mul<i32> for IVec3 {
    fn mul(self, rhs: i32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i32> for IVec3 { fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs } }
impl std::ops::Div<IVec3> for i32 {
    fn div(self, rhs: IVec3) -> Self::Output { IVec3::splat(self) / rhs }
    type Output = IVec3;
}
impl std::ops::Div<i32> for IVec3 {
    fn div(self, rhs: i32) -> Self::Output { self / IVec3::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i32> for IVec3 { fn div_assign(&mut self, rhs: i32) { *self = *self / rhs } }
impl std::ops::Rem<IVec3> for i32 {
    fn rem(self, rhs: IVec3) -> Self::Output { IVec3::splat(self) % rhs }
    type Output = IVec3;
}
impl std::ops::Rem<i32> for IVec3 {
    fn rem(self, rhs: i32) -> Self::Output { self % IVec3::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i32> for IVec3 { fn rem_assign(&mut self, rhs: i32) { *self = *self % rhs } }

impl AsUniformValue for IVec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::IntVec3([self.x, self.y, self.z])
    }
}
impl From<Vec3> for IVec3 {
    fn from(value: Vec3) -> Self {
        Self { x: value.x as i32, y: value.y as i32, z: value.z as i32 }
    }
}
impl From<(i32, i32, i32)> for IVec3 {
    fn from(value: (i32, i32, i32)) -> Self {
        Self { x: value.0, y: value.1, z: value.2 }
    }
}
impl From<[i32; 3]> for IVec3 {
    fn from(value: [i32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
impl From<IVec3> for [i32; 3] {
    fn from(value: IVec3) -> Self {
        [value.x, value.y, value.z]
    }
}
impl From<IVec3> for (i32, i32, i32) {
    fn from(value: IVec3) -> Self {
        (value.x, value.y, value.z)
    }
}
///create an interger vector with an x, y and z coordinate.
pub const fn ivec3(x: i32, y: i32, z: i32) -> IVec3{
    IVec3 { x, y, z }
}
