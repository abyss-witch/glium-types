use glium::uniforms::AsUniformValue;
use crate::vectors::DVec2;
use super::{DMat3, DMat4};

#[derive(Clone, Copy, PartialEq, Debug)]
///a matrix often used for transformations in glium.
pub struct DMat2 {
    matrix: [[f64; 2]; 2]
}
impl DMat2{
    pub const IDENTITY: Self = DMat2::from_values(
        1.0, 0.0,
        0.0, 1.0,
    );
    pub const fn from_scale(scale: DVec2) -> Self{
        DMat2::from_values(
            scale.x, 0.0,
            0.0, scale.y
        )
    }
    pub fn from_transform(scale: DVec2, rot: f64) -> Self{
        Self::from_values(
            rot.cos()*scale.x, -rot.sin()*scale.y,
            rot.sin()*scale.x, rot.cos()*scale.y
        )
    }
    pub fn from_rot(rot: f64) -> Self{
        Self::from_values(
            rot.cos(), -rot.sin(),
            rot.sin(), rot.cos()
        )
    }
    ///creates a matrix with the following values.
    /// ```
    /// use glium_types::matrices::DMat2;
    /// let new_matrix = DMat2::from_values(
    ///     1.0, 0.0, 
    ///     0.0, 1.0
    /// );
    /// assert!(new_matrix == DMat2::IDENTITY);
    /// ```
    pub const fn from_values(
        a: f64, b: f64,
        c: f64, d: f64
        ) -> Self{
        Self{
            //opengl uses a diffent matrix format to the input. this is why the order is shifted.
            matrix: [
                [a, c],
                [b, d]
                
            ]
        }
    }
    pub fn scale(&self, scalar: f64) -> DMat2{
        let DMat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        Self::from_values(
            a * scalar, b * scalar,
            c * scalar, d * scalar
        )
    }
    pub fn transpose(self) -> Self{
        let DMat2 { matrix: [
            [a, c],
            [b, d]
        ] } = self;
        DMat2::from_values(
            a, c,
            b, d
        )
    }
    pub fn determinant(&self) -> f64{
        let DMat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        a*d - b*c
    }
    #[allow(non_snake_case)]
    pub fn inverse(&self) -> DMat2{
        let DMat2 { matrix: [
            [a, c],
            [b, d]
        ]} = self;
        
        let scalar = 1.0/self.determinant();
        Self::from_values(
            *d, -b,
            -c, *a
        ).scale(scalar)
    }
    pub const fn column(&self, pos: usize) -> [f64; 2]{
        self.matrix[pos]
    }
    pub const fn row(&self, pos: usize) -> [f64; 2]{
        let matrix = self.matrix;
        [matrix[0][pos], matrix[1][pos]]
    }
}
impl Default for DMat2{
    fn default() -> Self {
        Self { matrix: [
            [1.0, 0.0],
            [0.0, 1.0],
        ] }
    }
    
}
#[allow(clippy::needless_range_loop)]
impl std::ops::Mul for DMat2{
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.0; 2]; 2];
        for x in 0..2 {
            for y in 0..2 {
                matrix[x][y] = DVec2::dot(self.row(y).into(), rhs.column(x).into());
            }
        }
        Self {
            matrix
        }
    }
    type Output = Self;
}
impl std::ops::MulAssign for DMat2{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl AsUniformValue for DMat2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::DoubleMat2(self.matrix)
    }
}
impl From<DMat3> for DMat2{
    fn from(value: DMat3) -> Self {
        Self::from_values(
            value[0][0], value[0][1],
            value[1][0], value[1][1]
        )
    }
}
impl From<DMat4> for DMat2{
    fn from(value: DMat4) -> Self {
        Self::from_values(
            value[0][0], value[0][1],
            value[1][0], value[1][1]
        )
    }
}
impl std::ops::Index<usize> for DMat2{
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
    
    type Output = [f64; 2];
}
impl std::ops::IndexMut<usize> for DMat2{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}
#[test]
fn test_inverse(){
    let mut a = DMat2::from_values(
        4.0, 0.0,
        1.0, 3.0
    );
    a = a.inverse() * a;
    assert!(a == DMat2::IDENTITY)
}
#[test]
fn addition(){
    let a = DMat2::IDENTITY;
    assert_eq!(a + a, DMat2::from_values(
        2.0, 0.0,
        0.0, 2.0
    ))
}

impl std::ops::Add for DMat2{
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
impl std::ops::AddAssign for DMat2{
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
impl std::ops::Sub for DMat2{
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
impl std::ops::SubAssign for DMat2{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
