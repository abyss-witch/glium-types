use std::time::Duration;

use glium::{glutin::surface::WindowSurface, implement_uniform_block, implement_vertex, index, uniforms::{AsUniformValue, Uniforms}, Display, DrawParameters, Frame, Program, Surface, Vertex, VertexBuffer};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

use crate::shaders::Shader;

#[allow(dead_code)]
///Used to create a window and draw things to it using the **glium** library. In the future it will suport inputs.
pub struct Window{
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    pub display: Display<WindowSurface>,
    frame: Option<Frame>
}
impl Window{
    ///Creates new window.
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
        Self { window, event_loop, display, frame: None }
    }
    ///Creates and returns a new frame.
    pub fn new_frame(&mut self) -> &mut Frame{
        let frame = self.display.draw();
        self.frame = Some(frame);
        match &mut self.frame {
            Some(frame) => frame,
            None => todo!(),
        }
    }
    ///Updates window events and draws frame. This functions may be split in the future.
    pub fn update(&mut self) -> PumpStatus {
        if let Some(frame) = self.frame.take(){
            frame.finish().unwrap();
        }
        let status = self.event_loop.pump_events(Some(Duration::ZERO), |event, window_target|{
            
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    _ => (),
                },
                _ => (),
            };

        });
        status
    }
    ///Create vertex buffer from shape.
    pub fn vertex_buffer<V: Vertex>(&self, shape: &Vec<V>) -> VertexBuffer<V> {
        glium::VertexBuffer::new(&self.display, shape).unwrap()
    }

    ///Generates a mesh using vertices and indices.
    /// ```no_run
    /// use Window::vec2;
    /// 
    /// let shape = vec![vec2(-0.5, -0.5), vec2(0.0, 0.5), vec2(0.5, -0.25)];
    /// let indices = index::NoIndices(PrimitiveType::TrianglesList);
    /// 
    /// let mesh = window.gen_mesh(&shape, indices.into());
    /// ```
    pub fn gen_mesh<'a, V: Vertex>(&self, shape: &Vec<V>, indices: index::IndicesSource<'a>) -> Mesh<'a, V>{
        let vertices = self.vertex_buffer(shape);
        Mesh{
            vertices,
            indices
        }
    }
    pub fn draw<V: Vertex, U: Uniforms>(&mut self, mesh: &Mesh<V>, program: &Program, uniforms: &U, draw_parameters: &DrawParameters) {
        match &mut self.frame {
            Some(frame) => frame.draw(&mesh.vertices, mesh.indices.clone(), program, uniforms, draw_parameters).unwrap(),
            None => panic!("called 'draw()' without creating a frame first."),
        };
    }
    ///Draw as mesh using a shader.
    pub fn draw_with_shader<V: Vertex, U: Uniforms>(&mut self, mesh: &Mesh<V>, shader: &dyn Shader<U>) {
        self.draw(mesh, shader.program(), &shader.uniforms(), &shader.draw_parameters());
    }
    
}





#[derive(Copy, Clone)]
pub struct Vec2 {
    position: [f32; 2],
}

#[allow(dead_code)]
impl Vec2{
    pub fn new(x: f32, y: f32) -> Self{
        Self { position: [x, y] }
    }
    pub fn x(&mut self) -> &mut f32{
        &mut self.position[0]
    }
    pub fn y(&mut self) -> &mut f32{
        &mut self.position[1]
    }
}
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}
implement_vertex!(Vec2, position);
implement_uniform_block!(Vec2, position);

#[derive(Copy, Clone)]
pub struct Colour {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32
}
impl AsUniformValue for Colour{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec4([self.red, self.green, self.blue, self.alpha])
    }
}
impl Colour{
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }
}
implement_vertex!(Colour, red, green, blue, alpha);

pub struct Mesh<'a, V: Vertex> {
    vertices: VertexBuffer<V>,
    indices: index::IndicesSource<'a>
}