use glium::uniforms::AsUniformValue;

use crate::{vec3::Vec3, prelude::vec3, quaternion::Quaternion};

#[derive(Clone, Copy, PartialEq, Debug)]
///a matrix often used for transformations in glium.
pub struct Mat4 {
    matrix: [[f32; 4]; 4]
}
impl Mat4{
    pub const IDENTITY: Self = Mat4::from_values(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    pub const fn from_scale(scale: Vec3) -> Self{
        let (x, y, z) = (scale.x, scale.y, scale.z);
        Mat4::from_values(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_transform(pos: Vec3, scale: Vec3, rot: Quaternion) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        Mat4::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r), pos.x,
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r), pos.y,
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_pos_and_rot(pos: Vec3, rot: Quaternion) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        Mat4::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r), pos.x,
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r), pos.y,
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_scale_and_pos(pos: Vec3, scale: Vec3) -> Self{
        Mat4::from_values(
            scale.x, 0.0, 0.0, pos.x,
            0.0, scale.y, 0.0, pos.y,
            0.0, 0.0, scale.z, pos.z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_pos(pos: Vec3) -> Self{
        let (x, y, z) = (pos.x, pos.y, pos.z);
        Mat4::from_values(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_rot(rot: Quaternion) -> Self{
        rot.into()
    }
    ///creates a perspective matrix. known as `view` in the vertex shader.
    pub fn view_matix(window_dimesnsions: (u32, u32), fov: f32, zfar: f32, znear: f32) -> Self{
        let (width, height) = window_dimesnsions;
        let aspect_ratio = height as f32 / width as f32;
        let f = 1.0 / (fov / 2.0).tan();
        Self{
            matrix: [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        }
    }
    ///creates a 2d perspective matrix. known as `view` in the vertex shader.
    pub fn orthagraphic_view_matix(window_dimesnsions: (u32, u32)) -> Self{
        let (width, height) = window_dimesnsions;
        let aspect_ratio = height as f32 / width as f32;
        Mat4::from_scale(vec3(aspect_ratio, 1.0, 1.0))
    }
    ///creates a matrix with the following values.
    ///
    /// ```
    /// let new_matrix = Matrix::from_values(
    ///     1.0, 0.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0, 0.0,
    ///     0.0, 0.0, 1.0, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// );
    /// assert!(new_matrix == Matrix::IDENTITY);
    /// ```
    pub const fn from_values(
        a: f32, b: f32, c: f32, d: f32,
        e: f32, f: f32, g: f32, h: f32,
        i: f32, j: f32, k: f32, l: f32,
        m: f32, n: f32, o: f32, p: f32
        ) -> Self{
        Self{
            //opengl uses a diffent matrix format to the input. this is why the order is shifted.
            matrix: [
                [a, e, i, m],
                [b, f, j, n],
                [c, g, k, o],
                [d, h, l, p]
            ]
        }
    }
    pub const fn column(&self, pos: usize) -> [f32; 4]{
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f32; 4]{
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos], matrix[2][pos], matrix[3][pos]]
    }
}
impl Default for Mat4{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }
    
}
impl std::ops::Mul for Mat4{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 4];  4];
        for x in 0..4{
            for y in 0..4{
                matrix[x][y] = dot_vec4(self.row(y), rhs.column(x));
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for Mat4{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl AsUniformValue for Mat4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.matrix)
    }
}
impl std::ops::Add for Mat4{
    fn add(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self{
            matrix:[
                [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2], a[0][3] + b[0][3]],
                [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2], a[1][3] + b[1][3]],
                [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2], a[2][3] + b[2][3]],
                [a[3][0] + b[3][0], a[3][1] + b[3][1], a[3][2] + b[3][2], a[3][3] + b[3][3]],
            ]
        }
    }
    type Output = Self;
}
impl std::ops::AddAssign for Mat4{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl std::ops::Sub for Mat4{
    fn sub(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self{
            matrix:[
                [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2], a[0][3] - b[0][3]],
                [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2], a[1][3] - b[1][3]],
                [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2], a[2][3] - b[2][3]],
                [a[3][0] - b[3][0], a[3][1] - b[3][1], a[3][2] - b[3][2], a[3][3] - b[3][3]],
            ]
        }
    }
    type Output = Self;
}
impl std::ops::SubAssign for Mat4{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
fn dot_vec4(a: [f32; 4], b: [f32; 4]) -> f32{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
#[test]
fn test(){
    let mut a = Mat4::IDENTITY;
    a += a;
    assert!(a == Mat4::IDENTITY + Mat4::IDENTITY)
}