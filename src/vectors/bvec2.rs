use glium::uniforms::{AsUniformValue, UniformValue};
use super::bvec3::*;
/// a boolean vector with an x and y component
#[derive(Debug, Clone, Copy)]
pub struct BVec2 { pub x: bool, pub y: bool }
impl BVec2 {
    /// a vector full of trues
    pub const TRUE: Self = Self::new(true, true);
    /// a vector full of falses
    pub const FALSE: Self = Self::new(false, false);
    /// a vector where x is true
    pub const X: Self = Self::new(true, false);
    /// a vector where y is true
    pub const Y: Self = Self::new(false, true);

    pub const fn new(x: bool, y: bool) -> Self { Self { x, y } }
    pub const fn truncate(self) -> bool { self.x }
    pub const fn extend(self, z: bool) -> BVec3 { BVec3::new(self.x, self.y, z) }
}
impl AsUniformValue for BVec2 {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::BoolVec2([self.x, self.y])
    }
}
/// a boolean vector with an x and y component
pub const fn bvec2(x: bool, y: bool) -> BVec2 { BVec2::new(x, y) }

impl std::ops::BitOr for BVec2 {
    fn bitor(self, rhs: Self) -> Self::Output { bvec2(self.x | rhs.x, self.y | rhs.y) }
    type Output = Self;
}
impl std::ops::BitOrAssign for BVec2 { fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs } }
impl std::ops::BitAnd for BVec2 {
    fn bitand(self, rhs: Self) -> Self::Output { bvec2(self.x & rhs.x, self.y & rhs.y) }
    type Output = Self;
}
impl std::ops::BitAndAssign for BVec2 { fn bitand_assign(&mut self, rhs: Self) { *self = *self & rhs } }
impl std::ops::BitXor for BVec2 {
    fn bitxor(self, rhs: Self) -> Self::Output { bvec2(self.x ^ rhs.x, self.y ^ rhs.y) }
    type Output = Self;
}
impl std::ops::BitXorAssign for BVec2 { fn bitxor_assign(&mut self, rhs: Self) { *self = *self ^ rhs } }
impl std::ops::Not for BVec2 {
    fn not(self) -> Self::Output { bvec2(!self.x, !self.y) }
    type Output = Self;
}

impl From<(bool, bool)> for BVec2 { fn from(f: (bool, bool)) -> Self { bvec2(f.0, f.1)} }
impl From<BVec2> for (bool, bool) { fn from(f: BVec2) -> Self { (f.x, f.y) } }
impl From<[bool; 2]> for BVec2 { fn from(f: [bool; 2]) -> Self { bvec2(f[0], f[1])} }
impl From<BVec2> for [bool; 2] { fn from(f: BVec2) -> Self { [f.x, f.y] } }
