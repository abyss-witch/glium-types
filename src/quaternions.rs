use derive_cmp_ops::{CmpAdd, CmpAddAssign, CmpNeg, CmpSub, CmpSubAssign};

use crate::vectors::{Vec3, Vec4, DVec4, DVec3};

#[derive(Clone, Copy, CmpAdd, CmpSub, CmpAddAssign, CmpSubAssign, CmpNeg, Debug, PartialEq)]
///a 4 part vector often used to represent rotations. note that multiplication of quaternions
///is applying transformations.
pub struct Quat{
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32
}
impl Quat{
    pub const IDENTITY: Self = Self { r: 1.0, i: 0.0, j: 0.0, k: 0.0 };
    ///rotation around the x axis in radians
    pub fn from_x_rot(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: a.sin(), j: 0.0, k: 0.0 }
    }
    ///rotation around the y axis in radians
    pub fn from_y_rot(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: a.sin(), k: 0.0 }
    }
    ///rotation around the z axis in radians
    pub fn from_z_rot(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: 0.0, k: a.sin() }
    }
    ///rotation around the inputed axis in radians
    pub fn from_axis_rot(angle: f32, axis: Vec3) -> Self{
        let a = angle / 2.0;
        Self{
            r: a.cos(),
            i: a.sin() * axis.x,
            j: a.sin() * axis.y,
            k: a.sin() * axis.z,
        }
    }
    ///get innverse of a quaternion. panics if all quaternions values are 0
    pub fn inverse(self) -> Self {
        let Quat { r, i, j, k } = self;
        if r == 0.0 && i == 0.0 && j == 0.0 && k == 0.0 {
            panic!("tried to get inverse of zero quaternion");
        }
        let scalar = r*r + i*i + j*j + k*k;
        Self {
            r: r / scalar,
            i: -i / scalar,
            j: -j / scalar,
            k: -k / scalar
        }
    }
}
impl From<Vec4> for Quat {
    fn from(value: Vec4) -> Self {
        Self { r: value.x, i: value.y, j: value.z, k: value.w }
    }
}
impl From<DQuat> for Quat {
    fn from(value: DQuat) -> Self {
        Self { r: value.r as f32, i: value.i as f32, j: value.j as f32, k: value.k as f32 }
    }
}
impl std::ops::Mul for Quat{
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self;
        let b = rhs;
        Self { 
            r: a.r*b.r - a.i*b.i - a.j*b.j - a.k*b.k, 
            i: a.r*b.i + a.i*b.r + a.j*b.k - a.k*b.j, 
            j: a.r*b.j - a.i*b.k + a.j*b.r + a.k*b.i, 
            k: a.r*b.k + a.i*b.j - a.j*b.i + a.k*b.r
        }
    }
    type Output = Self;
}
#[test]
fn quaternion_inverse() {
    let a = Quat::from_x_rot(3.0);
    let inv_a = a.inverse();
    assert!(a * inv_a == Quat { r: 1.0, i: 0.0, j: 0.0, k: 0.0 });
}



#[derive(Clone, Copy, CmpAdd, CmpSub, CmpAddAssign, CmpSubAssign, CmpNeg, Debug, PartialEq)]
///a 4 part double vector often used to represent rotations. note that multiplication of quaternions
///is applying transformations.
pub struct DQuat{
    pub r: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64
}
impl DQuat{
    pub const IDENTITY: Self = Self { r: 1.0, i: 0.0, j: 0.0, k: 0.0 };
    ///rotation around the x axis in radians
    pub fn from_x_rot(angle: f64) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: a.sin(), j: 0.0, k: 0.0 }
    }
    ///rotation around the y axis in radians
    pub fn from_y_rot(angle: f64) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: a.sin(), k: 0.0 }
    }
    ///rotation around the z axis in radians
    pub fn from_z_rot(angle: f64) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: 0.0, k: a.sin() }
    }
    ///rotation around the inputed axis in radians
    pub fn from_axis_rot(angle: f64, axis: DVec3) -> Self{
        let a = angle / 2.0;
        Self{
            r: a.cos(),
            i: a.sin() * axis.x,
            j: a.sin() * axis.y,
            k: a.sin() * axis.z,
        }
    }
    ///get innverse of a quaternion. panics if all quaternions values are 0
    pub fn inverse(self) -> Self {
        let DQuat { r, i, j, k } = self;
        if r == 0.0 && i == 0.0 && j == 0.0 && k == 0.0 {
            panic!("tried to get inverse of zero quaternion");
        }
        let scalar = r*r + i*i + j*j + k*k;
        Self {
            r: r / scalar,
            i: -i / scalar,
            j: -j / scalar,
            k: -k / scalar
        }
    }
}
impl From<DVec4> for DQuat {
    fn from(value: DVec4) -> Self {
        Self { r: value.x, i: value.y, j: value.z, k: value.w }
    }
}
impl From<Quat> for DQuat {
    fn from(value: Quat) -> Self {
        Self { r: value.r as f64, i: value.i as f64, j: value.j as f64, k: value.k as f64 }
    }
}
impl std::ops::Mul for DQuat{
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self;
        let b = rhs;
        Self { 
            r: a.r*b.r - a.i*b.i - a.j*b.j - a.k*b.k, 
            i: a.r*b.i + a.i*b.r + a.j*b.k - a.k*b.j, 
            j: a.r*b.j - a.i*b.k + a.j*b.r + a.k*b.i, 
            k: a.r*b.k + a.i*b.j - a.j*b.i + a.k*b.r
        }
    }
    type Output = Self;
}
#[test]
fn dquaternion_inverse() {
    let a = DQuat::from_x_rot(3.0);
    let inv_a = a.inverse();
    assert!(a * inv_a == DQuat { r: 1.0, i: 0.0, j: 0.0, k: 0.0 });
}
