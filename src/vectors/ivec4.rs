use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec4::Vec4, ivec3::{ivec3, IVec3}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///an interger vector made from a x, y, z and w coordinate.
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

    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self{
        Self { x, y, z, w }
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: i32) -> Self{
        Self::new(value, value, value, value)
    }
    pub fn truncate(&self) -> IVec3{
        ivec3(self.x, self.y, self.z)
    }
    ///the length of the vector without being square rooted.
    pub fn length_squared(self) -> i32 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: IVec4) -> i32{
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: IVec4) -> i32{
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: i32) -> IVec4{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}
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
///create a double vector with an x, y, z and w coordinate.
pub const fn ivec4(x: i32, y: i32, z: i32, w: i32) -> IVec4{
    IVec4 { x, y, z, w }
}
