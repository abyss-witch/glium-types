use glium::uniforms::AsUniformValue;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///a vector made from and x y and z coordinate.
pub struct Vec2{
    pub x: f32,
    pub y: f32,
}
impl Vec2{
    pub fn new(x: f32, y: f32) -> Self{
        Self { x, y }
    }
    ///create a vector where x and y equals value.
    pub fn splat(value: f32) -> Self{
        Self::new(value, value)
    }
    ///the length of the vector before being square rooted.
    pub fn length_squared(&self) -> f32{
        self.x*self.x + self.y*self.y
    }
    ///length of the vector.
    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(&self, other: Vec2) -> f32{
        (*self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(&self, other: Vec2) -> f32{
        (*self - other).length()
    }
    ///get the dot profuct of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(&self, other: Vec2) -> f32{
        self.x * other.x + self.y * other.y
    }
    ///multiplies each value by the scalar.
    pub fn scale(&self, scalar: f32) -> Vec2{
        Self::new(self.x * scalar, self.y * scalar)
    }
    ///makes the length of the vector equal to 1. on fail returns vec2 of zeros
    pub fn normalise(&self) -> Self{
        let length = self.length();
        if length == 0.0 { return vec2(0.0, 0.0); }
        self.scale(1.0 / length)
    }
}
impl AsUniformValue for Vec2{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec2([self.x, self.y])
    }
}
impl std::ops::Add for Vec2{
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
    type Output = Self;
}
impl std::ops::Sub for Vec2{
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
    type Output = Self;
}
impl std::ops::Mul for Vec2{
    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
    type Output = Self;
}
impl std::ops::Div for Vec2{
    fn div(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
    type Output = Self;
}

impl std::ops::AddAssign for Vec2{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign for Vec2{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign for Vec2{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl std::ops::DivAssign for Vec2{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

///create a vector with an x and y coordinate.
pub const fn vec2(x: f32, y: f32) -> Vec2{
    Vec2 { x, y }
}