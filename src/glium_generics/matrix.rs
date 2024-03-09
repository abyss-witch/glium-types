use glium::uniforms::AsUniformValue;

use crate::{glium_generics::{generics::Vec3, quaternion::Quaternion}, vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
///a matrix often used for transformations in glium.
pub struct Matrix {
    matrix: [[f32; 4]; 4]
}
impl Matrix{
    const IDENTITY: Self = Matrix::from_values(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    pub const fn from_scale(scale: Vec3) -> Self{
        let (x, y, z) = (scale.x, scale.y, scale.z);
        Matrix::from_values(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_transform(pos: Vec3, scale: Vec3, rot: Quaternion) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        Matrix::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r), pos.x,
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r), pos.y,
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_pos_and_rot(pos: Vec3, rot: Quaternion) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        Matrix::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r), pos.x,
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r), pos.y,
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_scale_and_pos(pos: Vec3, scale: Vec3) -> Self{
        Matrix::from_values(
            scale.x, 0.0, 0.0, pos.x,
            0.0, scale.y, 0.0, pos.y,
            0.0, 0.0, scale.z, pos.z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_pos(pos: Vec3) -> Self{
        let (x, y, z) = (pos.x, pos.y, pos.z);
        Matrix::from_values(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_rot(rot: Quaternion) -> Self{
        rot.into()
    }
    ///creates a matrix that creates perspective. known as `view` in the vertex shader.
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
impl std::ops::Mul for Matrix{
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
impl Default for Matrix{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0 , 0.0, 0.0, 1.0f32],
        ] }
    }
    
}
impl AsUniformValue for Matrix{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat4(self.matrix)
    }
}
fn dot_vec4(a: [f32; 4], b: [f32; 4]) -> f32{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

#[test]
fn identity_matrix_multiplication(){
    let a = Matrix::IDENTITY;
    assert!(a*a == a, "{:?} == {:?}", a*a, a);
}
#[test]
fn transformation_matrix(){
    let pos = vec3(0.0, 1.0, 30.0);
    let rot = Quaternion::from_axis_rotation(380.0, vec3(1.0, 52.0, 8000.0));
    let scale = vec3(1.0, 3.0, 15000000.0);
    let trans_matrix = Matrix::from_transform(pos, scale, rot);
    let answer = Matrix::from_pos(pos) * Matrix::from_rot(rot) * Matrix::from_scale(scale);
    assert!(trans_matrix == answer);
}