use glium::implement_vertex;
#[derive(Debug, Clone, Copy)]
///a vector made from and x y and z coordinate.
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
///create a vector with an x, y and z coordinate.
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3{
    Vec3 { x, y, z }
}
#[derive(Debug, Clone, Copy)]
///a vertex used for rendering by glium. stores vertex position. equivelant to `position` in vertex shader and `v_position` in fragment shader.
pub struct Vertex{
    pub position: (f32, f32, f32)
}
implement_vertex!(Vertex, position);
#[derive(Debug, Clone, Copy)]
///a normal used for rendering by glium. stores vertex direction. equivelant to `normal` in vertex shader and `v_normal` in fragment shader.
pub struct Normal{
    pub normal: (f32, f32, f32)
}
implement_vertex!(Normal, normal);
#[derive(Clone, Copy)]
///a texture coordinate used for rendering by glium. also called a uv. stores uvs. equivelant to `texture_coords` in vertex shader and `uv` in fragment shader.
pub struct TextureCoords{
    texture_coords: (f32, f32)
}
implement_vertex!(TextureCoords, texture_coords);
#[derive(Clone, Copy)]
///a vertex colour used for rendering by glium. stores vertex colour. equivelant to `colour` in vertex shader and `vertex_colour` in fragment shader.
pub struct Colour{
    colour: (f32, f32, f32, f32)
}
implement_vertex!(Colour, colour);