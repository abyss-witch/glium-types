use glium::uniforms::{AsUniformValue, UniformValue};
use super::{bvec2::*, bvec4::*};
/// a boolean vector with an x, y and z component
#[derive(Debug, Clone, Copy)]
pub struct BVec3 { pub x: bool, pub y: bool, pub z: bool }
impl BVec3 {
    /// a vector full of trues
    pub const TRUE: Self = Self::new(true, true, true);
    /// a vector full of falses
    pub const FALSE: Self = Self::new(false, false, false);
    /// a vector where x is true
    pub const X: Self = Self::new(true, false, false);
    /// a vector where y is true
    pub const Y: Self = Self::new(false, true, false);
    /// a vector where z is true
    pub const Z: Self = Self::new(false, false, true);

    pub const fn new(x: bool, y: bool, z: bool) -> Self { Self { x, y, z } }
    pub const fn truncate(self) -> BVec2 { BVec2::new(self.x, self.y) }
    pub const fn extend(self, w: bool) -> BVec4 { BVec4::new(self.x, self.y, self.z, w) }
}
impl AsUniformValue for BVec3 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::BoolVec3([self.x, self.y, self.z])
    }
}
/// a boolean vector with an x, y and z component
pub const fn bvec3(x: bool, y: bool, z: bool) -> BVec3 { BVec3::new(x, y, z) }

impl std::ops::BitOr for BVec3 {
    fn bitor(self, rhs: Self) -> Self::Output { bvec3(self.x | rhs.x, self.y | rhs.y, self.z | rhs.z) }
    type Output = Self;
}
impl std::ops::BitOrAssign for BVec3 { fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs } }
impl std::ops::BitAnd for BVec3 {
    fn bitand(self, rhs: Self) -> Self::Output { bvec3(self.x & rhs.x, self.y & rhs.y, self.z & rhs.z) }
    type Output = Self;
}
impl std::ops::BitAndAssign for BVec3 { fn bitand_assign(&mut self, rhs: Self) { *self = *self & rhs } }
impl std::ops::BitXor for BVec3 {
    fn bitxor(self, rhs: Self) -> Self::Output { bvec3(self.x ^ rhs.x, self.y ^ rhs.y, self.z ^ rhs.z) }
    type Output = Self;
}
impl std::ops::BitXorAssign for BVec3 { fn bitxor_assign(&mut self, rhs: Self) { *self = *self ^ rhs } }
impl std::ops::Not for BVec3 {
    fn not(self) -> Self::Output { bvec3(!self.x, !self.y, !self.z) }
    type Output = Self;
}

impl From<(bool, bool, bool)> for BVec3 { fn from(f: (bool, bool, bool)) -> Self { bvec3(f.0, f.1, f.2)} }
impl From<BVec3> for (bool, bool, bool) { fn from(f: BVec3) -> Self { (f.x, f.y, f.z) } }
impl From<[bool; 3]> for BVec3 { fn from(f: [bool; 3]) -> Self { bvec3(f[0], f[1], f[2])} }
impl From<BVec3> for [bool; 3] { fn from(f: BVec3) -> Self { [f.x, f.y, f.z] } }
