use glium::uniforms::AsUniformValue;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
///a vector made from and x y and z coordinate.
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vec3{
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self { x, y, z }
    }
    ///create a vector where x, y and z equals value.
    pub fn splat(value: f32) -> Self{
        Self::new(value, value, value)
    }

    ///the length of the vector before being square rooted.
    pub fn length_squared(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    ///length of the vector.
    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }
    ///distance between two vectors before being square rooted.
    pub fn distance_squared(&self, other: Vec3) -> f32{
        (*self - other).length_squared()
    }
    ///distance between two vectors.
    pub fn distance(&self, other: Vec3) -> f32{
        (*self - other).length()
    }
    ///get the dot profuct of 2 vectors. equal to the cosign of the angle between vectors.
    pub fn dot(&self, other: Vec3) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    ///multiplies each value by the scalar.
    pub fn scale(&self, scalar: f32) -> Vec3{
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl AsUniformValue for Vec3{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec3([self.x, self.y, self.z])
    }
}
impl std::ops::Add for Vec3{
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
    type Output = Self;
}
impl std::ops::Sub for Vec3{
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
    type Output = Self;
}
impl std::ops::Mul for Vec3{
    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
    type Output = Self;
}
impl std::ops::Div for Vec3{
    fn div(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
    type Output = Self;
}
///create a vector with an x, y and z coordinate.
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3{
    Vec3 { x, y, z }
}