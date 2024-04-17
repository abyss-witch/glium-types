pub mod teapot;
pub mod vectors;
pub mod mat4;
pub mod mat3;
pub mod generics;
pub mod quaternion;
pub mod shaders;

pub mod prelude{
    pub use crate::{
        mesh, 
        generics::{VertexColour, Vertex, TextureCoords, Normal},
        vectors::vec3::{Vec3, vec3},
        vectors::vec2::{Vec2, vec2},
        mat4::Mat4,
        mat3::Mat3,
        quaternion::Quaternion,
        shaders
    };
}