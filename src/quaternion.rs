use derive_cmp_ops::{CmpAdd, CmpAddAssign, CmpNeg, CmpSub, CmpSubAssign};

use crate::vectors::Vec3;

#[derive(Clone, Copy, CmpAdd, CmpSub, CmpAddAssign, CmpSubAssign, CmpNeg, Debug, PartialEq)]
///a 4 part vector often used to represent rotations. note that multiplication of quaternions
///is applying transformations.
pub struct Quaternion{
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32
}
impl Quaternion{
    ///rotation around the x axis in radians
    pub fn from_x_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: a.sin(), j: 0.0, k: 0.0 }
    }
    ///rotation around the y axis in radians
    pub fn from_y_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: a.sin(), k: 0.0 }
    }
    ///rotation around the z axis in radians
    pub fn from_z_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: 0.0, k: a.sin() }
    }
    ///rotation around the inputed axis in radians
    pub fn from_axis_rotation(angle: f32, axis: Vec3) -> Self{
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
        let Quaternion { r, i, j, k } = self;
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

impl std::ops::Mul for Quaternion{
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
    let a = Quaternion::from_x_rotation(3.0);
    let inv_a = a.inverse();
    assert!(a * inv_a == Quaternion { r: 1.0, i: 0.0, j: 0.0, k: 0.0 });
}
