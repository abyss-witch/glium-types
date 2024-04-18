use glium::DrawParameters;
///draw parameters for 3d rendering. same as default except with counter clockwise culling and depth test.
///```
/// use glium_types::params;
/// //create draw parameter for 3d rendering with transparency
/// let draw_parameters = glium::DrawParameters{
///     blend: glium::Blend::alpha_blending(),
///     ..params::alias_3d()
/// };
///```
pub fn alias_3d<'a>() -> DrawParameters<'a>{
    DrawParameters{
        backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
        depth: glium::Depth{
            write: true,
            test: glium::DepthTest::IfLess,
            ..Default::default()
        },
        ..Default::default()
    }
}