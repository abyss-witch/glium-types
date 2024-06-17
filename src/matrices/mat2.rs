use glium::uniforms::AsUniformValue;
use crate::prelude::Vec2;
use super::{Mat3, Mat4};

#[derive(Clone, Copy, PartialEq, Debug)]
///a matrix often used for transformations in glium.
pub struct Mat2 {
    matrix: [[f32; 2]; 2]
}
impl Mat2{
    pub const IDENTITY: Self = Mat2::from_values(
        1.0, 0.0,
        0.0, 1.0,
    );
    pub const fn from_scale(scale: Vec2) -> Self{
        Mat2::from_values(
            scale.x, 0.0,
            0.0, scale.y
        )
    }
    pub fn from_transform(scale: Vec2, rot: f32) -> Self{
        Self::from_values(
            rot.cos()*scale.x, -rot.sin()*scale.y,
            rot.sin()*scale.x, rot.cos()*scale.y
        )
    }
    pub fn from_rot(rot: f32) -> Self{
        Self::from_values(
            rot.cos(), -rot.sin(),
            rot.sin(), rot.cos()
        )
    }
    ///creates a matrix with the following values.
    /// ```
    /// use glium_types::matrices::Mat2;
    /// let new_matrix = Mat2::from_values(
    ///     1.0, 0.0, 
    ///     0.0, 1.0
    /// );
    /// assert!(new_matrix == Mat2::IDENTITY);
    /// ```
    pub const fn from_values(
        a: f32, b: f32,
        c: f32, d: f32
        ) -> Self{
        Self{
            //opengl uses a diffent matrix format to the input. this is why the order is shifted.
            matrix: [
                [a, c],
                [b, d]
                
            ]
        }
    }
    pub fn scale(&self, scalar: f32) -> Mat2{
        let Mat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        Self::from_values(
            a * scalar, b * scalar,
            c * scalar, d * scalar
        )
    }
    pub fn transpose(self) -> Self{
        let Mat2 { matrix: [
            [a, c],
            [b, d]
        ] } = self;
        Mat2::from_values(
            a, c,
            b, d
        )
    }
    pub fn determinant(&self) -> f32{
        let Mat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        a*d - b*c
    }
    #[allow(non_snake_case)]
    pub fn inverse(&self) -> Mat2{
        let Mat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        
        let scalar = 1.0/self.determinant();
        Self::from_values(
            *d, -b,
            -c, *a
        ).scale(scalar)
    }
    pub const fn column(&self, pos: usize) -> [f32; 2]{
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f32; 2]{
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos]]
    }
}
impl Default for Mat2{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0],
            [0.0, 1.0],
        ] }
    }
    
}
#[allow(clippy::needless_range_loop)]
impl std::ops::Mul for Mat2{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 2]; 2];
        for x in 0..2 {
            for y in 0..2 {
                matrix[x][y] = Vec2::dot(self.row(y).into(), rhs.column(x).into());
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for Mat2{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl AsUniformValue for Mat2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Mat2(self.matrix)
    }
}
impl From<Mat3> for Mat2{
    fn from(value: Mat3) -> Self {
        Self::from_values(
            value[0][0], value[0][1],
            value[1][0], value[1][1]
        )
    }
}
impl From<Mat4> for Mat2{
    fn from(value: Mat4) -> Self {
        Self::from_values(
            value[0][0], value[0][1],
            value[1][0], value[1][1]
        )
    }
}
impl std::ops::Index<usize> for Mat2{
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
    
    type Output = [f32; 2];
}

#[test]
fn test_inverse(){
    let mut a = Mat2::from_values(
        4.0, 0.0,
        1.0, 3.0
    );
    a = a.inverse() * a;
    assert!(a == Mat2::IDENTITY)
}
#[test]
fn addition(){
    let a = Mat2::IDENTITY;
    assert_eq!(a + a, Mat2::from_values(
        2.0, 0.0,
        0.0, 2.0
    ))
}

impl std::ops::Add for Mat2{
    fn add(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self{
            matrix:[
                [a[0][0] + b[0][0], a[0][1] + b[0][1]],
                [a[1][0] + b[1][0], a[1][1] + b[1][1]]
            ]
        }
    }
    type Output = Self;
}
impl std::ops::AddAssign for Mat2{
    fn add_assign(&mut self, rhs: Self) {
        let a = self.matrix;
        let b = rhs.matrix;
        *self = Self{
            matrix:[
                [a[0][0] + b[0][0], a[0][1] + b[0][1]],
                [a[1][0] + b[1][0], a[1][1] + b[1][1]]
            ]
        };
    }
}
impl std::ops::Sub for Mat2{
    fn sub(self, rhs: Self) -> Self::Output {
        let a = self.matrix;
        let b = rhs.matrix;
        Self{
            matrix:[
                [a[0][0] - b[0][0], a[0][1] - b[0][1]],
                [a[1][0] - b[1][0], a[1][1] - b[1][1]]
            ]
        }
    }
    type Output = Self;
}
impl std::ops::SubAssign for Mat2{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
