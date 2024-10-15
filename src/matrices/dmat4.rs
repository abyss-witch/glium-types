use glium::uniforms::AsUniformValue;

use crate::{matrices::DMat3, quaternions::DQuat, vectors::{dvec3, DVec4, DVec3}};

use super::DMat2;

#[derive(Clone, Copy, PartialEq, Debug)]
///a matrix often used for transformations in glium.
pub struct DMat4 {
    matrix: [[f64; 4]; 4]
}
impl DMat4{
    pub const IDENTITY: Self = DMat4::from_values(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    pub const fn from_scale(scale: DVec3) -> Self{
        let (x, y, z) = (scale.x, scale.y, scale.z);
        DMat4::from_values(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
    // transform from position, scale and rotation. much quicker than multiplying
    //  position * rot * scale matrices with the same result.
    pub fn from_transform(pos: DVec3, scale: DVec3, rot: DQuat) -> Self{
        let DQuat { r, i, j, k } = rot;
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        DMat4::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r), pos.x,
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r), pos.y,
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    //makes inverse of transform, useful for making camera transform.
    pub fn from_inverse_transform(pos: DVec3, scale: DVec3, rot: DQuat) -> Self {
        let DQuat { r, i, j, k } = rot;
        let DVec3 { x, y, z } = DVec3::ONE / scale;

        let scalar = r*r + i*i + j*j + k*k;
        let s = 2.0/(scalar*scalar);
        let sx = s*x;
        let sy = s*y;
        let sz = s*z;
        let a = x - sx*(j*j + k*k);
        let b = sx*(i*j + k*r);
        let c = sx*(i*k - j*r);
        let d = sy*(i*j - k*r);
        let e = y - sy*(i*i + k*k);
        let f = sy*(j*k + i*r);
        let g = sz*(i*k + j*r);
        let h = sz*(j*k - i*r);
        let i = z - sz*(i*i + j*j);
        DMat4::from_values(
            a, b, c, -a*pos.x - b*pos.y - c*pos.z,
            d, e, f, -d*pos.x - e*pos.y - f*pos.z,
            g, h, i, -g*pos.x - h*pos.y - i*pos.z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_pos_and_rot(pos: DVec3, rot: DQuat) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        DMat4::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r), pos.x,
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r), pos.y,
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j), pos.z,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_pos_and_scale(pos: DVec3, scale: DVec3) -> Self{
        DMat4::from_values(
            scale.x, 0.0, 0.0, pos.x,
            0.0, scale.y, 0.0, pos.y,
            0.0, 0.0, scale.z, pos.z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_scale_and_rot(scale: DVec3, rot: DQuat) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        DMat4::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r), 0.0,
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r), 0.0,
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)), 0.0,   
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub const fn from_pos(pos: DVec3) -> Self{
        let (x, y, z) = (pos.x, pos.y, pos.z);
        DMat4::from_values(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        )
    }
    pub fn from_rot(rot: DQuat) -> Self{
        rot.into()
    }
    //rotates the matrixs components
    pub fn tranpose(self) -> Self {
        let DMat4 { matrix: [
            [a, e, i, m],
            [b, f, j, n],
            [c, g, k, o],
            [d, h, l, p]
        ] } = self;
        DMat4::from_values(
            a, e, i, m,
            b, f, j, n,
            c, g, k, o,
            d, h, l, p
        )
    }
    ///creates a 3d perspective matrix. known as `view` in the vertex shader.
    pub fn view_matrix_3d(window_dimesnsions: (u32, u32), fov: f64, zfar: f64, znear: f64) -> Self{
        let (width, height) = window_dimesnsions;
        let aspect_ratio = height as f64 / width as f64;
        let f = 1.0 / (fov / 2.0).tan();
        Self{
            matrix: [
                [f * aspect_ratio, 0.0,              0.0              , 0.0],
                [         0.0    ,  f ,              0.0              , 0.0],
                [         0.0    , 0.0,  (zfar+znear)/(zfar-znear)    , 1.0],
                [         0.0    , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        }
    }
    ///creates a 2d perspective matrix. known as `view` in the vertex shader.
    pub fn view_matrix_2d(window_dimesnsions: (u32, u32)) -> Self{
        let (width, height) = window_dimesnsions;
        let aspect_ratio = height as f64 / width as f64;
        DMat4::from_scale(dvec3(aspect_ratio, 1.0, 1.0))
    }
    ///creates a matrix with the following values.
    /// ```
    /// use glium_types::matrices::DMat4;
    /// let new_matrix = DMat4::from_values(
    ///     1.0, 0.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0, 0.0,
    ///     0.0, 0.0, 1.0, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// );
    /// assert!(new_matrix == DMat4::IDENTITY);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub const fn from_values(
        a: f64, b: f64, c: f64, d: f64,
        e: f64, f: f64, g: f64, h: f64,
        i: f64, j: f64, k: f64, l: f64,
        m: f64, n: f64, o: f64, p: f64
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
    pub const fn column(&self, pos: usize) -> [f64; 4] {
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f64; 4] {
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos], matrix[2][pos], matrix[3][pos]]
    }
    pub fn position(&self) -> DVec3 {
        dvec3(self[3][0], self[3][1], self[3][2])
    }
    pub fn determinant(self) -> f64 {
        let a = self;
        let s0 = a[0][0] * a[1][1] - a[1][0] * a[0][1];
        let s1 = a[0][0] * a[1][2] - a[1][0] * a[0][2];
        let s2 = a[0][0] * a[1][3] - a[1][0] * a[0][3];
        let s3 = a[0][1] * a[1][2] - a[1][1] * a[0][2];
        let s4 = a[0][1] * a[1][3] - a[1][1] * a[0][3];
        let s5 = a[0][2] * a[1][3] - a[1][2] * a[0][3];

        let c5 = a[2][2] * a[3][3] - a[3][2] * a[2][3];
        let c4 = a[2][1] * a[3][3] - a[3][1] * a[2][3];
        let c3 = a[2][1] * a[3][2] - a[3][1] * a[2][2];
        let c2 = a[2][0] * a[3][3] - a[3][0] * a[2][3];
        let c1 = a[2][0] * a[3][2] - a[3][0] * a[2][2];
        let c0 = a[2][0] * a[3][1] - a[3][0] * a[2][1];

        s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0
    }
    /// get inverse of matrix. useful for camera.
    pub fn inverse(self) -> Self {
        let a = self;
        let s0 = a[0][0] * a[1][1] - a[1][0] * a[0][1];
        let s1 = a[0][0] * a[1][2] - a[1][0] * a[0][2];
        let s2 = a[0][0] * a[1][3] - a[1][0] * a[0][3];
        let s3 = a[0][1] * a[1][2] - a[1][1] * a[0][2];
        let s4 = a[0][1] * a[1][3] - a[1][1] * a[0][3];
        let s5 = a[0][2] * a[1][3] - a[1][2] * a[0][3];

        let c5 = a[2][2] * a[3][3] - a[3][2] * a[2][3];
        let c4 = a[2][1] * a[3][3] - a[3][1] * a[2][3];
        let c3 = a[2][1] * a[3][2] - a[3][1] * a[2][2];
        let c2 = a[2][0] * a[3][3] - a[3][0] * a[2][3];
        let c1 = a[2][0] * a[3][2] - a[3][0] * a[2][2];
        let c0 = a[2][0] * a[3][1] - a[3][0] * a[2][1];

        let invdet = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let mut b = DMat4::IDENTITY;
        
        b[0][0] = ( a[1][1] * c5 - a[1][2] * c4 + a[1][3] * c3) * invdet;
        b[0][1] = (-a[0][1] * c5 + a[0][2] * c4 - a[0][3] * c3) * invdet;
        b[0][2] = ( a[3][1] * s5 - a[3][2] * s4 + a[3][3] * s3) * invdet;
        b[0][3] = (-a[2][1] * s5 + a[2][2] * s4 - a[2][3] * s3) * invdet;

        b[1][0] = (-a[1][0] * c5 + a[1][2] * c2 - a[1][3] * c1) * invdet;
        b[1][1] = ( a[0][0] * c5 - a[0][2] * c2 + a[0][3] * c1) * invdet;
        b[1][2] = (-a[3][0] * s5 + a[3][2] * s2 - a[3][3] * s1) * invdet;
        b[1][3] = ( a[2][0] * s5 - a[2][2] * s2 + a[2][3] * s1) * invdet;

        b[2][0] = ( a[1][0] * c4 - a[1][1] * c2 + a[1][3] * c0) * invdet;
        b[2][1] = (-a[0][0] * c4 + a[0][1] * c2 - a[0][3] * c0) * invdet;
        b[2][2] = ( a[3][0] * s4 - a[3][1] * s2 + a[3][3] * s0) * invdet;
        b[2][3] = (-a[2][0] * s4 + a[2][1] * s2 - a[2][3] * s0) * invdet;

        b[3][0] = (-a[1][0] * c3 + a[1][1] * c1 - a[1][2] * c0) * invdet;
        b[3][1] = ( a[0][0] * c3 - a[0][1] * c1 + a[0][2] * c0) * invdet;
        b[3][2] = (-a[3][0] * s3 + a[3][1] * s1 - a[3][2] * s0) * invdet;
        b[3][3] = ( a[2][0] * s3 - a[2][1] * s1 + a[2][2] * s0) * invdet;

        b
    }
    pub fn scale(self, scalar: f64) -> DMat4 {
        let DMat4 { matrix: [
            [a, e, i, m],
            [b, f, j, n],
            [c, g, k, o],
            [d, h, l, p]
        ] } = self;
        DMat4::from_values(
            a*scalar, b*scalar, c*scalar, d*scalar,
            e*scalar, f*scalar, g*scalar, h*scalar,
            i*scalar, j*scalar, k*scalar, l*scalar,
            m*scalar, n*scalar, o*scalar, p*scalar
        )
    }
}
impl Default for DMat4{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ] }
    }
    
}
#[allow(clippy::needless_range_loop)] //in this case i think it look nicer :)
impl std::ops::Mul for DMat4{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 4];  4];
        for x in 0..4{
            for y in 0..4{
                matrix[x][y] = DVec4::dot(self.row(y).into(), rhs.column(x).into());
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for DMat4{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl AsUniformValue for DMat4{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleMat4(self.matrix)
    }
}
impl std::ops::Add for DMat4{
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
impl std::ops::AddAssign for DMat4{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl std::ops::Sub for DMat4{
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
impl std::ops::SubAssign for DMat4{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl From<DMat3> for DMat4{
    fn from(value: DMat3) -> Self {
        Self::from_values(
            value[0][0], value[0][1], value[0][2], 0.0,
            value[1][0], value[1][1], value[1][2], 0.0,
            value[2][0], value[2][1], value[2][2], 0.0,
            0.0,         0.0,         0.0,         1.0
        )
    }
}
impl From<DMat2> for DMat4{
    fn from(value: DMat2) -> Self {
        Self::from_values(
            value[0][0], value[0][1], 0.0, 0.0,
            value[1][0], value[1][1], 0.0, 0.0,
            0.0,         0.0,         1.0, 0.0,
            0.0,         0.0,         0.0, 1.0
        )
    }
}
impl From<DQuat> for DMat4 {
    fn from(value: DQuat) -> DMat4 {
        let DQuat { r, i, j, k } = value;
         DMat4::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r), 0.0,
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r), 0.0,
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j), 0.0,   
            0.0, 0.0, 0.0, 1.0
        )
    }
}
impl std::ops::Index<usize> for DMat4{
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
    
    type Output = [f64; 4];
}
impl std::ops::IndexMut<usize> for DMat4{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}
#[test]
fn test_from_inverse_transform() {
    let rot = DQuat::from_x_rot(1.3);
    let pos = dvec3(1.0, 2.0, 0.3);
    let scale = dvec3(1.1, 2.0, 3.9);
    let transform = DMat4::from_transform(pos, scale, rot);
    let inv_transform = DMat4::from_inverse_transform(pos, scale, rot);
    let result = transform * inv_transform;
    assert!(eq_mats(DMat4::IDENTITY, result));
}
#[test]
fn mat4_inverse() {
    let rot = DQuat::from_x_rot(1.3);
    let pos = dvec3(1.0, 2.0, 0.3);
    let scale = dvec3(1.1, 2.0, 3.9);
    let a = DMat4::from_transform(pos, scale, rot);
    assert!(eq_mats(DMat4::IDENTITY, a.inverse() * a));
}
#[test]
fn test_transform(){
    let rot = DQuat::from_x_rot(1.3);
    let pos = dvec3(1.0, 2.0, 0.3);
    let scale = dvec3(1.1, 2.0, 3.9);
    let transform = DMat4::from_transform(pos, scale, rot);
    let result = DMat4::from_pos(pos) * DMat4::from_rot(rot) * DMat4::from_scale(scale);
    assert!(eq_mats(transform, result))
}
#[allow(dead_code)]
fn eq_mats(a: DMat4, b: DMat4) -> bool {
    for x in 0..4 {
        for y in 0..4 {
            if f64::abs(a[x][y] - b[x][y]) > f64::EPSILON {
                return false;
            }
        }
    }
    true
}
