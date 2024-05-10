use glium::implement_vertex;
#[derive(Debug, Clone, Copy)]
///a vertex used for rendering by glium. stores vertex position. equivelant to `position` in vertex shader and `v_position` in fragment shader.
pub struct Vertex{
    pub position: (f32, f32, f32)
}
implement_vertex!(Vertex, position);
impl Vertex{
    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Vertex { position: (x, y, z) }
    }
}
#[derive(Debug, Clone, Copy)]
///a normal used for rendering by glium. stores vertex direction. equivelant to `normal` in vertex shader and `v_normal` in fragment shader.
pub struct Normal{
    pub normal: (f32, f32, f32)
}
impl Normal{
    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Self { normal: (x, y, z) }
    }
}
implement_vertex!(Normal, normal);
#[derive(Clone, Copy, Debug)]
///a texture coordinate used for rendering by glium. also called a uv. stores uvs. equivelant to `texture_coords` in vertex shader and `uv` in fragment shader.
pub struct TextureCoords{
    pub texture_coords: (f32, f32)
}
impl TextureCoords{
    pub const fn new(u: f32, v: f32) -> Self{
        Self { texture_coords: (u, v) }
    }
}
implement_vertex!(TextureCoords, texture_coords);
#[derive(Clone, Copy, Debug)]
///a vertex colour used for rendering by glium. stores vertex colour. equivelant to `colour` in vertex shader and `vertex_colour` in fragment shader.
pub struct VertexColour{
    pub colour: (f32, f32, f32, f32)
}
impl VertexColour{
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self{
        Self { colour: (r, g, b, a) }
    }
}
implement_vertex!(VertexColour, colour);

#[macro_export]
macro_rules! mesh {
    ($display: expr, $indices: expr, $( $x: expr ),*) => {
        {
            use glium::{index::PrimitiveType, IndexBuffer, VertexBuffer};
            let display = $display;
            (
                IndexBuffer::new(display, PrimitiveType::TrianglesList, $indices).unwrap(),
                $(
                    VertexBuffer::new(display, $x).unwrap(),
                )*                
            )
        }
    }
}
