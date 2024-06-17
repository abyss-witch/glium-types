use derive_cmp_ops::CmpOps;
use glium::uniforms::AsUniformValue;
use super::{vec3::Vec3, ivec2::{ivec2, IVec2}, ivec4::{ivec4, IVec4}};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, CmpOps)]
///a vector made from a x, y and z coordinate.
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}
impl IVec3 {
    ///a zero vector
    pub const ZERO: Self = ivec3(0, 0, 0);
    ///a vector full of ones
    pub const ONE: Self = ivec3(1, 1, 1);
    ///the x axis
    pub const X: Self = ivec3(1, 0, 0);
    ///the y axis
    pub const Y: Self = ivec3(0, 1, 0);
    ///the z axis
    pub const Z: Self = ivec3(0, 0, 1);

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn extend(self, w: i32) -> IVec4{
        ivec4(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> IVec2{
        ivec2(self.x, self.y)
    }
    ///create a vector where x, y and z equals `value`.
    pub fn splat(value: i32) -> Self{
        Self::new(value, value, value)
    }

    ///the length of the vector before being square rooted.
    pub fn length_squared(self) -> i32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(self, other: IVec3) -> i32 {
        (self - other).length_squared()
    }
    ///get the dot product of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(self, other: IVec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///get the cross product of 2 vectors. equal to the vector that is perpendicular to both input vectors. the output vector is not normalised.
    /// ```
    /// use glium_types::vectors::{IVec3, ivec3};
    /// let x = ivec3(1, 0, 0);
    /// let y = ivec3(0, 1, 0);
    /// let z = ivec3(0, 0, 1);
    /// assert!(x.cross(y) == z);
    /// ```
    pub fn cross(&self, other: IVec3) -> IVec3{
        ivec3(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x
        )
    }
    ///multiplies each value by the scalar.
    pub fn scale(self, scalar: i32) -> IVec3{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl AsUniformValue for IVec3{
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
///create a double vector with an x, y and z coordinate.
pub const fn ivec3(x: i32, y: i32, z: i32) -> IVec3{
    IVec3 { x, y, z }
}
