use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, ivec3::*, bvec4::*};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CmpOps)]
/// an interger vector made from a x, y, z and w coordinate.
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32
}
impl IVec4 {
    ///a zero vector
    pub const ZERO: Self = ivec4(0, 0, 0, 0);
    ///a vector full of ones
    pub const ONE: Self = ivec4(1, 1, 1, 1);
    ///the x axis
    pub const X: Self = ivec4(1, 0, 0, 0);
    ///the y axis
    pub const Y: Self = ivec4(0, 1, 0, 0);
    ///the z axis
    pub const Z: Self = ivec4(0, 0, 1, 0);
    ///the w axis
    pub const W: Self = ivec4(0, 0, 0, 1);

    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> IVec3 { ivec3(self.x, self.y, self.z) }
    /// create a vector where x, y and z equals `value`.
    pub const fn splat(value: i32) -> Self { Self::new(value, value, value, value) }

    /// the length of the vector without being square rooted.
    pub fn length_squared(self) -> i32 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    /// distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: IVec4) -> i32{
        (self - other).length_squared()
    }
    /// get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: IVec4) -> i32{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    /// multiplies each value by the scalar.
    pub fn scale(self, scalar: i32) -> IVec4{
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
impl std::ops::Mul<IVec4> for i32 {
    fn mul(self, rhs: IVec4) -> Self::Output { rhs * self }
    type Output = IVec4;
}
impl std::ops::Mul<i32> for IVec4 {
    fn mul(self, rhs: i32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::MulAssign<i32> for IVec4 { fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs } }
impl std::ops::Div<IVec4> for i32 {
    fn div(self, rhs: IVec4) -> Self::Output { IVec4::splat(self) / rhs }
    type Output = IVec4;
}
impl std::ops::Div<i32> for IVec4 {
    fn div(self, rhs: i32) -> Self::Output { self / IVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<i32> for IVec4 { fn div_assign(&mut self, rhs: i32) { *self = *self / rhs } }
impl std::ops::Rem<IVec4> for i32 {
    fn rem(self, rhs: IVec4) -> Self::Output { IVec4::splat(self) % rhs }
    type Output = IVec4;
}
impl std::ops::Rem<i32> for IVec4 {
    fn rem(self, rhs: i32) -> Self::Output { self % IVec4::splat(rhs) }
    type Output = Self;
}
impl std::ops::RemAssign<i32> for IVec4 { fn rem_assign(&mut self, rhs: i32) { *self = *self % rhs } }

impl AsUniformValue for IVec4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::IntVec4([self.x, self.y, self.z, self.w])
    }
}
impl From<Vec4> for IVec4 {
    fn from(value: Vec4) -> Self {
        Self { x: value.x as i32, y: value.y as i32, z: value.z as i32, w: value.w as i32 }
    }
}
impl From<(i32, i32, i32, i32)> for IVec4 {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        Self { x: value.0, y: value.1, z: value.2, w: value.3 }
    }
}
impl From<[i32; 4]> for IVec4 {
    fn from(value: [i32; 4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}
impl From<IVec4> for [i32; 4] {
    fn from(value: IVec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<IVec4> for (i32, i32, i32, i32) {
    fn from(value: IVec4) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
///create an interger vector with an x, y, z and w coordinate.
pub const fn ivec4(x: i32, y: i32, z: i32, w: i32) -> IVec4{
    IVec4 { x, y, z, w }
}
