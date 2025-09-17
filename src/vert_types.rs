use glium::implement_vertex;
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
/// a vertex used for rendering by glium. stores vertex position. equivelant to `position` in the supplied
/// vertex shader and `v_position` in fragment shader.
pub struct Vertex{
    pub position: (f32, f32, f32)
}
impl Vertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { position: (x, y, z) }
    }
}
impl From<Vec3> for Vertex {
    fn from(value: Vec3) -> Self {
        Vertex { position: (value.x, value.y, value.z) }
    }
}
implement_vertex!(Vertex, position);

#[derive(Debug, Clone, Copy)]
/// a normal used for rendering by glium. stores vertex direction. equivelant to `normal` in the supplied
/// vertex shader and `v_normal` in fragment shader.
pub struct Normal {
    pub normal: (f32, f32, f32)
}
impl From<Vec3> for Normal {
    fn from(value: Vec3) -> Self {
        Self { normal: (value.x, value.y, value.z) }
    }
}
impl Normal{
    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Self { normal: (x, y, z) }
    }
}
implement_vertex!(Normal, normal);

#[derive(Clone, Copy, Debug)]
/// a texture coordinate used for rendering by glium. also called a uv. stores uvs. equivelant to
/// `texture_coords` in the supplied vertex shader and `uv` in fragment shader.
pub struct TextureCoords{
    pub texture_coords: (f32, f32)
}
impl From<Vec2> for TextureCoords {
    fn from(value: Vec2) -> Self {
        Self { texture_coords: (value.x, value.y) }
    }
}
impl TextureCoords{
    pub const fn new(u: f32, v: f32) -> Self{
        Self { texture_coords: (u, v) }
    }
}
implement_vertex!(TextureCoords, texture_coords);

#[derive(Clone, Copy, Debug)]
/// a vertex colour used for rendering by glium. stores vertex colour. equivelant to `colour` in the supplied
/// vertex shader and `v_colour` in fragment shader.
pub struct VertexColour{
    pub colour: (f32, f32, f32, f32)
}
impl From<Vec4> for VertexColour{
    fn from(value: Vec4) -> Self {
        Self { colour: (value.x, value.y, value.z, value.w) }
    }
}
impl VertexColour{
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self{
        Self { colour: (r, g, b, a) }
    }
}
implement_vertex!(VertexColour, colour);

#[derive(Debug)]
pub enum MeshError {
    VertexErr(glium::vertex::BufferCreationError),
    IndexErr(glium::index::BufferCreationError)
}
impl From<glium::vertex::BufferCreationError> for MeshError {
    fn from(e: glium::vertex::BufferCreationError) -> Self { Self::VertexErr(e) }
}
impl From<glium::index::BufferCreationError> for MeshError {
    fn from(e: glium::index::BufferCreationError) -> Self { Self::IndexErr(e) }
}
use glium::{index::*, vertex::*, backend::Facade};
pub fn load_mesh<const X: usize, F, V, I>(facade: &F, vertices: [&[impl glium::Vertex + std::fmt::Debug]; X], index: &[I]) -> Result<([VertexBuffer<impl glium::Vertex>; X], IndexBuffer<I>), MeshError>
where F: Facade + ?Sized, V: glium::Vertex + Copy, I: Index {
    let mut vertex = Vec::new();
    for vertices in vertices {
        vertex.push(VertexBuffer::new(facade, vertices)?);
    }
    
    let index = IndexBuffer::new(facade, PrimitiveType::TrianglesList, index)?;
    Ok((vertex.try_into().unwrap(), index))
}
/// create a triangle mesh from a `Display`, `u32` indices followed by anything that can be stored
/// in a vertex buffer e.g `Vertex` and/or `TextureCoords`.
/// if an error occurs it returns the `MeshError` enum
/// ```no_run
/// use glium_types::{mesh, teapot};
/// let (indices, vertices, normals) = mesh!(
///     &display, &teapot::INDICES, &teapot::VERTICES, &teapot::NORMALS
/// ).unwrap();
/// ```
#[macro_export]
macro_rules! mesh {
    ($display: expr, $indices: expr, $( $x: expr ),*) => {
        {
           let result: Result<_, $crate::vert_types::MeshError> = 'block: {
                use $crate::glium::{index::PrimitiveType, IndexBuffer, VertexBuffer};

                Ok((
                    match IndexBuffer::new($display, $crate::glium::index::PrimitiveType::TrianglesList, $indices) {
                        Err(e) => break 'block Err($crate::vert_types::MeshError::IndexErr(e)),
                        Ok(v) => v,
                    },
                    $(
                        match VertexBuffer::new($display, $x) {
                            Err(e) => break 'block Err($crate::vert_types::MeshError::VertexErr(e)),
                            Ok(v) => v,
                        },
                    )*
                ))
            };
            result
        }
    }
}
