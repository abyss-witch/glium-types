pub mod teapot;
pub mod vectors;
pub mod matrices;
pub mod vert_types;
pub mod quaternions;
pub mod shaders;
pub mod params;
pub use glium;

pub mod prelude{
    pub use crate::{
        vert_types::{VertexColour, Vertex, TextureCoords, Normal},
        vectors::*,
        matrices::*,
        params,
        quaternions::*,
        shaders,
        mesh
    };
}
