use glium::uniforms::AsUniformValue;
use crate::{matrices::Mat4, quaternions::Quat, vectors::{Vec3, Vec2}};

use super::Mat2;

#[derive(Clone, Copy, PartialEq, Debug)]
/// a matrix often used for transformations in glium
pub struct Mat3 {
    matrix: [[f32; 3]; 3]
}
impl Mat3{
    pub const IDENTITY: Self = Mat3::from_values(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    pub const fn from_colum_major_array(array: [[f32; 3]; 3]) -> Self { Self { matrix: array } }
    pub const fn into_column_major_array(self) -> [[f32; 3]; 3] { self.matrix }
    pub const fn from_row_major_array(array: [[f32; 3]; 3]) -> Self { Self { matrix: array }.transpose() }
    pub const fn into_row_major_array(self) -> [[f32; 3]; 3] { self.transpose().matrix }
    pub const fn from_scale(scale: Vec3) -> Self {
        let (x, y, z) = (scale.x, scale.y, scale.z);
        Mat3::from_values(
            x, 0.0, 0.0,
            0.0, y, 0.0,
            0.0, 0.0, z,
        )
    }
    pub fn from_2d_transform(pos: Vec2, scale: Vec2, rot: f32) -> Self {
        Self::from_values(
            scale.x * rot.cos(), scale.y * -rot.sin(), pos.x,
            scale.x * rot.sin(), scale.y * rot.cos(), pos.y,
            0.0, 0.0, 1.0
        )
    }
    pub fn from_transform(scale: Vec3, rot: Quat) -> Self{
        let (r, i, j, k) = (rot.r, rot.i, rot.j, rot.k);
        let (sx, sy, sz) = (scale.x * 2.0, scale.y * 2.0, scale.z * 2.0);
        Mat3::from_values(
            sx*(0.5 - (j*j + k*k)), sy*(i*j - k*r), sz*(i*k + j*r),
            sx*(i*j + k*r), sy*(0.5 - (i*i + k*k)), sz*(j*k - i*r),
            sx*(i*k - j*r), sy*(j*k + i*r), sz*(0.5 - (i*i + j*j)) 
        )
    }
    pub fn from_rot(rot: Quat) -> Self { rot.into() }
    ///creates a matrix with the following values.
    ///
    /// ```
    /// use glium_types::matrices::Mat3;
    /// let new_matrix = Mat3::from_values(
    ///     1.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0,
    ///     0.0, 0.0, 1.0
    /// );
    /// assert!(new_matrix == Mat3::IDENTITY);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub const fn from_values(
        a: f32, b: f32, c: f32,
        d: f32, e: f32, f: f32,
        g: f32, h: f32, i: f32
        ) -> Self{
        Self{
            // opengl uses a diffent matrix format to the input. this is why the order is shifted.
            matrix: [
                [a, d, g],
                [b, e, h],
                [c, f, i],
            ]
        }
    }
    pub fn scale(&self, scalar: f32) -> Mat3{
        let Mat3 { matrix: [
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
    pub fn determinant(&self) -> f32{
        let Mat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        a*(e*i - f*h) - b*(d*i - f*g) + c*(d*h - e*g)
    }
    #[allow(non_snake_case)]
    pub fn inverse(&self) -> Mat3{
        let Mat3 { matrix: [
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
    pub const fn transpose(self) -> Self {
        let Mat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        Mat3::from_values(
            a, d, g,
            b, e, h,
            c, f, i
        )
    }
    pub const fn column(&self, pos: usize) -> [f32; 3] {
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f32; 3] {
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos], matrix[2][pos]]
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0, 0.0,],
            [0.0, 1.0, 0.0,],
            [0.0, 0.0, 1.0,],
        ] }
    }
    
}
impl std::ops::Mul<Vec3> for Mat3 {
    fn mul(self, rhs: Vec3) -> Self::Output { rhs.transform(self) }
    type Output = Vec3;
}
impl std::ops::Mul<f32> for Mat3 {
    fn mul(self, rhs: f32) -> Self::Output { self.scale(rhs) }
    type Output = Self;
}
impl std::ops::Mul<Mat3> for f32 {
    fn mul(self, rhs: Mat3) -> Self::Output { rhs * self }
    type Output = Mat3;
}
impl std::ops::MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs }
}
impl std::ops::Div<Mat3> for f32 {
    fn div(self, rhs: Mat3) -> Self::Output {
        let Mat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = rhs;
        Mat3::from_values(
            self/a, self/b, self/c,
            self/d, self/e, self/f,
            self/g, self/h, self/i
        )
    }
    type Output = Mat3;
}
impl std::ops::Div<f32> for Mat3 {
    fn div(self, rhs: f32) -> Self::Output { self.scale(1.0/rhs) }
    type Output = Self;
}
impl std::ops::DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, rhs: f32) { *self = *self / rhs }
}
impl std::ops::Rem<Mat3> for f32 {
    fn rem(self, rhs: Mat3) -> Self::Output {
        let Mat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = rhs;
        Mat3::from_values(
            self%a, self%b, self%c,
            self%d, self%e, self%f,
            self%g, self%h, self%i
        )
    }
    type Output = Mat3;
}
impl std::ops::Rem<f32> for Mat3 {
    fn rem(self, rhs: f32) -> Self::Output {
        let Mat3 { matrix: [
            [a, d, g],
            [b, e, h],
            [c, f, i],
        ]} = self;
        Mat3::from_values(
            a%rhs, b%rhs, c%rhs,
            d%rhs, e%rhs, f%rhs,
            g%rhs, h%rhs, i%rhs
        )
    }
    type Output = Self;
}
impl std::ops::RemAssign<f32> for Mat3 {
    fn rem_assign(&mut self, rhs: f32) { *self = *self % rhs }
}
#[allow(clippy::needless_range_loop)]
impl std::ops::Mul for Mat3{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 3];  3];
        for x in 0..3 {
            for y in 0..3 {
                matrix[x][y] = Vec3::dot(self.row(y).into(), rhs.column(x).into());
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Div for Mat3 {
    fn div(self, rhs: Self) -> Self::Output { self * rhs.inverse() }
    type Output = Self;
}
impl std::ops::DivAssign for Mat3 {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs }
}
impl AsUniformValue for Mat3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat3(self.matrix)
    }
}
impl std::ops::Add for Mat3{
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
impl std::ops::AddAssign for Mat3{
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
impl std::ops::Sub for Mat3{
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
impl std::ops::SubAssign for Mat3{
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
impl From<Mat4> for Mat3{
    fn from(value: Mat4) -> Self {
        Self::from_values(
            value[0][0], value[0][1], value[0][2],
            value[1][0], value[1][1], value[1][2],
            value[2][0], value[2][1], value[2][2]
        )
    }
}
impl From<Mat2> for Mat3{
    fn from(value: Mat2) -> Self {
        Self::from_values(
            value[0][0], value[0][1], 0.0,
            value[1][0], value[1][1], 0.0,
            0.0, 0.0, 1.0
        )
    }
}
impl From<Quat> for Mat3 {
    fn from(value: Quat) -> Mat3 {
        let Quat { r, i, j, k } = value;
        Mat3::from_values(
            1.0 - 2.0*(j*j + k*k), 2.0*(i*j - k*r), 2.0*(i*k + j*r),
            2.0*(i*j + k*r), 1.0 - 2.0*(i*i + k*k), 2.0*(j*k - i*r),
            2.0*(i*k - j*r), 2.0*(j*k + i*r), 1.0 - 2.0*(i*i + j*j)
        )
    }
}
impl std::ops::Index<usize> for Mat3{
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
    type Output = [f32; 3];
}
impl std::ops::IndexMut<usize> for Mat3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}
#[test]
fn test_inverse(){
    let mut a = Mat3::from_values(
        4.0, 0.0, 2.0,
        1.0, 3.0, 8.0,
        2.0, 3.0, 4.0
    );
    a = a.inverse() * a;
    assert!(a == Mat3::IDENTITY)
}
