use glium::uniforms::{AsUniformValue, UniformValue};
use super::bvec3::*;
/// a boolean vector with an x, y, z and w component
#[derive(Debug, Clone, Copy)]
pub struct BVec4 { pub x: bool, pub y: bool, pub z: bool, pub w: bool }
impl BVec4 {
    /// a vector full of trues
    pub const TRUE: Self = Self::new(true, true, true, true);
    /// a vector full of falses
    pub const FALSE: Self = Self::new(false, false, false, false);
    /// a vector where x is true
    pub const X: Self = Self::new(true, false, false, false);
    /// a vector where y is true
    pub const Y: Self = Self::new(false, true, false, false);
    /// a vector where z is true
    pub const Z: Self = Self::new(false, false, true, false);
    /// a vector where w is true
    pub const W: Self = Self::new(false, false, false, true);

    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self { Self { x, y, z, w } }
    pub const fn truncate(self) -> BVec3 { BVec3::new(self.x, self.y, self.z) }
}
impl AsUniformValue for BVec4 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::BoolVec4([self.x, self.y, self.z, self.w])
    }
}
/// a boolean vector with an x, y, z and w component
pub const fn bvec4(x: bool, y: bool, z: bool, w: bool) -> BVec4 { BVec4::new(x, y, z, w) }

impl std::ops::BitOr for BVec4 {
    fn bitor(self, rhs: Self) -> Self::Output { bvec4(self.x | rhs.x, self.y | rhs.y, self.z | rhs.z, self.w | rhs.w) }
    type Output = Self;
}
impl std::ops::BitOrAssign for BVec4 { fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs } }
impl std::ops::BitAnd for BVec4 {
    fn bitand(self, rhs: Self) -> Self::Output { bvec4(self.x & rhs.x, self.y & rhs.y, self.z & rhs.z, self.w & rhs.w) }
    type Output = Self;
}
impl std::ops::BitAndAssign for BVec4 { fn bitand_assign(&mut self, rhs: Self) { *self = *self & rhs } }
impl std::ops::BitXor for BVec4 {
    fn bitxor(self, rhs: Self) -> Self::Output { bvec4(self.x ^ rhs.x, self.y ^ rhs.y, self.z ^ rhs.z, self.w ^ rhs.w) }
    type Output = Self;
}
impl std::ops::BitXorAssign for BVec4 { fn bitxor_assign(&mut self, rhs: Self) { *self = *self ^ rhs } }
impl std::ops::Not for BVec4 {
    fn not(self) -> Self::Output { bvec4(!self.x, !self.y, !self.z, !self.w) }
    type Output = Self;
}

impl From<(bool, bool, bool, bool)> for BVec4 { fn from(f: (bool, bool, bool, bool)) -> Self { bvec4(f.0, f.1, f.2, f.3)} }
impl From<BVec4> for (bool, bool, bool, bool) { fn from(f: BVec4) -> Self { (f.x, f.y, f.z, f.w) } }
impl From<[bool; 4]> for BVec4 { fn from(f: [bool; 4]) -> Self { bvec4(f[0], f[1], f[2], f[3])} }
impl From<BVec4> for [bool; 4] { fn from(f: BVec4) -> Self { [f.x, f.y, f.z, f.w] } }
