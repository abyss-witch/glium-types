use derive_cmp_ops::{CmpAdd, CmpAddAssign, CmpNeg, CmpSub, CmpSubAssign};

use crate::{matrices::Mat3, matrices::Mat4, vectors::Vec3};

#[derive(Clone, Copy, CmpAdd, CmpSub, CmpAddAssign, CmpSubAssign, CmpNeg)]
///a 4 part vector often used to represent rotations. note that multiplication of quaternions is applying transformations.
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
    //rotation around the inputed axis in radians
    pub fn from_axis_rotation(angle: f32, axis: Vec3) -> Self{
        let a = angle / 2.0;
        Self{
            r: a.cos(),
            i: a.sin() * axis.x,
            j: a.sin() * axis.y,
            k: a.sin() * axis.z,
        }
    }
}
impl Into<Mat4> for Quaternion{
    fn into(self) -> Mat4 {
        let (r, i, j, k) = (self.r, self.i, self.j, self.k);
        Mat4::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r), 0.0,
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r), 0.0,
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j), 0.0,   
            0.0, 0.0, 0.0, 1.0
        )
    }
}
impl Into<Mat3> for Quaternion{
    fn into(self) -> Mat3 {
        let (r, i, j, k) = (self.r, self.i, self.j, self.k);
        Mat3::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r),
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r),
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j)
        )
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