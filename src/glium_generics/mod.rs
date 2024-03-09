pub mod teapot;
pub mod window;
pub mod matrix;
pub mod generics;
pub mod quaternion;

pub mod prelude{
    pub use crate::glium_generics::{
        generics::{Vec3, Vertex, TextureCoords, Normal, vec3},
        window::Window,
        matrix::Matrix,
        quaternion::Quaternion,
    };
}