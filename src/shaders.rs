use glium::{program, uniform, uniforms::{EmptyUniforms, Uniforms, UniformsStorage}, DrawParameters, Program};

use crate::window::Window;

use super::Colour;

pub const SOLID_FRAGMENT: &'static str = r#"
    #version 140

    out vec4 color;
    uniform vec4 colour;

    void main() {
        color = colour;
    }
"#;
pub const VERTEX: &'static str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

pub struct SolidColour{
    program: Program,
    pub colour: Colour
}
impl SolidColour{
    pub fn new(colour: Colour, window: &Window) -> Self {
        Self { program: program!(&window.display, 140 => { vertex: VERTEX, fragment: SOLID_FRAGMENT} ).unwrap(), colour }
    }
}
impl<'a> Shader<UniformsStorage<'a, Colour, EmptyUniforms>> for SolidColour{
    fn program(&self) -> &Program {
        &self.program
    }
    fn uniforms(&self) -> UniformsStorage<'a, Colour, EmptyUniforms> {
        uniform! { colour: self.colour }
    }
}
pub trait Shader<U: Uniforms> {
    fn program(&self) -> &Program;
    fn uniforms(&self) -> U;
    fn draw_parameters(&self) -> DrawParameters {
        DrawParameters::default()
    }
}