pub mod teapot;
pub mod vec3;
pub mod matrix;
pub mod generics;
pub mod quaternion;
pub mod shaders;

pub mod prelude{
    pub use crate::{
        mesh, 
        generics::{VertexColour, Vertex, TextureCoords, Normal},
        vec3::{Vec3, vec3},
        matrix::Mat4,
        quaternion::Quaternion,
        shaders
    };
}