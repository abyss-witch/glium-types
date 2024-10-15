use glium::uniforms::AsUniformValue;
use crate::{matrices::DMat4, quaternions::DQuat, vectors::{DVec3, DVec2}};

use super::DMat2;

#[derive(Clone, Copy, PartialEq, Debug)]
///a double matrix often used for transformations in glium.
pub struct DMat3 {
    matrix: [[f64; 3]; 3]
}
impl DMat3{
    pub const IDENTITY: Self = DMat3::from_values(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    pub const fn from_scale(scale: DVec3) -> Self{
        let (x, y, z) = (scale.x, scale.y, scale.z);
        DMat3::from_values(
            x, 0.0, 0.0,
            0.0, y, 0.0,
            0.0, 0.0, z,
        )
    }
    pub fn from_2d_transform(pos: DVec2, scale: DVec2, rot: f64) -> Self {
        Self::from_values(
            scale.x * rot.cos(), scale.y * -rot.sin(), pos.x,
            scale.x * rot.sin(), scale.y * rot.cos(), pos.y,
            0.0, 0.0, 1.0
        )
    }
    pub fn from_transform(scale: DVec3, rot: DQuat) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        DMat3::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r),
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r),
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)) 
        )
    }
    pub fn from_rot(rot: DQuat) -> Self{
        rot.into()
    }
    ///creates a matrix with the following values.
    ///
    /// ```
    /// use glium_types::matrices::DMat3;
    /// let new_matrix = DMat3::from_values(
    ///     1.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0,
    ///     0.0, 0.0, 1.0
    /// );
    /// assert!(new_matrix == DMat3::IDENTITY);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub const fn from_values(
        a: f64, b: f64, c: f64,
        d: f64, e: f64, f: f64,
        g: f64, h: f64, i: f64
        ) -> Self{
        Self{
            //opengl uses a diffent matrix format to the input. this is why the order is shifted.
            matrix: [
                [a, d, g],
                [b, e, h],
                [c, f, i],
            ]
        }
    }
    pub fn scale(&self, scalar: f64) -> DMat3{
        let DMat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        Self::from_values(
            a * scalar, b * scalar, c * scalar,
            d * scalar, e * scalar, f * scalar,
            g * scalar, h * scalar, i * scalar
        )
    }
    pub fn determinant(&self) -> f64{
        let DMat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        a*(e*i - f*h) - b*(d*i - f*g) + c*(d*h - e*g)
    }
    #[allow(non_snake_case)]
    pub fn inverse(&self) -> DMat3{
        let DMat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        let A = e*i - f*h;
        let B = -(d*i - f*g);
        let C = d*h - e*g;
        
        let determinet = a*A + b*B + c*C;
        let scalar = 1.0/determinet;
        Self::from_values(
            A, -(b*i - c*h), b*f - c*e,
            B, a*i - c*g, -(a*f - c*d),
            C, -(a*h - b*g), a*e - b*d
        ).scale(scalar)
    }
    pub fn transpose(self) -> Self {
        let DMat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        DMat3::from_values(
            a, d, g,
            b, e, h,
            c, f, i
        )
    }
    pub const fn column(&self, pos: usize) -> [f64; 3]{
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f64; 3]{
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos], matrix[2][pos]]
    }
}

impl Default for DMat3{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0, 0.0,],
            [0.0, 1.0, 0.0,],
            [0.0, 0.0, 1.0,],
        ] }
    }
    
}
#[allow(clippy::needless_range_loop)]
impl std::ops::Mul for DMat3{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 3];  3];
        for x in 0..3 {
            for y in 0..3 {
                matrix[x][y] = DVec3::dot(self.row(y).into(), rhs.column(x).into());
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for DMat3{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl AsUniformValue for DMat3{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleMat3(self.matrix)
    }
}
impl std::ops::Add for DMat3{
    fn add(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self{
            matrix:[
                [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2]],
                [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2]],
                [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2]],
            ]
        }
    }
    type Output = Self;
}
impl std::ops::AddAssign for DMat3{
    fn add_assign(&mut self, rhs: Self) {
        let a = self.matrix;
        let b = rhs.matrix;
        *self = Self{
            matrix:[
                [a[0][0] + b[0][0], a[0][1] + b[0][1], a[0][2] + b[0][2]],
                [a[1][0] + b[1][0], a[1][1] + b[1][1], a[1][2] + b[1][2]],
                [a[2][0] + b[2][0], a[2][1] + b[2][1], a[2][2] + b[2][2]],
            ]
        };
    }
}
impl std::ops::Sub for DMat3{
    fn sub(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self { matrix:[
                [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2]],
                [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2]],
                [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2]],
        ] }
    }
    type Output = Self;
}
impl std::ops::SubAssign for DMat3{
    fn sub_assign(&mut self, rhs: Self) {
        let a = self.matrix;
        let b = rhs.matrix;
        *self = Self { matrix:[
                [a[0][0] - b[0][0], a[0][1] - b[0][1], a[0][2] - b[0][2]],
                [a[1][0] - b[1][0], a[1][1] - b[1][1], a[1][2] - b[1][2]],
                [a[2][0] - b[2][0], a[2][1] - b[2][1], a[2][2] - b[2][2]],
        ] };
    }
}
impl From<DMat4> for DMat3{
    fn from(value: DMat4) -> Self {
        Self::from_values(
            value[0][0], value[0][1], value[0][2],
            value[1][0], value[1][1], value[1][2],
            value[2][0], value[2][1], value[2][2]
        )
    }
}
impl From<DMat2> for DMat3{
    fn from(value: DMat2) -> Self {
        Self::from_values(
            value[0][0], value[0][1], 0.0,
            value[1][0], value[1][1], 0.0,
            0.0, 0.0, 1.0
        )
    }
}
impl From<DQuat> for DMat3 {
    fn from(value: DQuat) -> DMat3 {
        let DQuat { r, i, j, k } = value;
        DMat3::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r),
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r),
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j)
        )
    }
}
impl std::ops::Index<usize> for DMat3{
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
    
    type Output = [f64; 3];
}
impl std::ops::IndexMut<usize> for DMat3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}
