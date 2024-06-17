pub mod teapot;
pub mod vectors;
pub mod matrices;
pub mod vert_types;
pub mod quaternion;
pub mod shaders;
pub mod params;

pub mod prelude{
    pub use crate::{
        mesh, 
        vert_types::{VertexColour, Vertex, TextureCoords, Normal},
        vectors::{Vec3, vec3, Vec4, vec4, Vec2, vec2},
        matrices::{Mat4, Mat3},
        params,
        quaternion::Quaternion,
        shaders
    };
}
