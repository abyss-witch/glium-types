use crate::{vec3::Vec3, matrix::Mat4};

#[derive(Clone, Copy)]
///a 4 part vector often used to represent rotations.
pub struct Quaternion{
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32
}
impl Quaternion{
    pub fn from_x_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: a.sin(), j: 0.0, k: 0.0 }
    }
    pub fn from_y_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: a.sin(), k: 0.0 }
    }
    pub fn from_z_rotation(angle: f32) -> Self{
        let a = angle / 2.0;
        Self { r: a.cos(), i: 0.0, j: 0.0, k: a.sin() }
    }
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