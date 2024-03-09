use std::{thread, time::{Duration, Instant}};
use glium::{program, uniform, Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer};
use winit::platform::pump_events::PumpStatus;
use glium_generics::prelude::*;
use glium_generics::teapot;
mod glium_generics;

const VERTEX: &'static str = include_str!("../assets/shaders/vertex.glsl");
const FRAGMENT: &'static str = include_str!("../assets/shaders/toon_shaded_frag.glsl");
fn main(){
    let mut window = Window::new();
    const ONE_60TH_OF_A_SECOND: Duration = Duration::from_millis(1000/60);
    let now = Instant::now();

    //load teapot mesh.
    let vertices = VertexBuffer::new(&window.display, &teapot::VERTICES).unwrap();
    let normals = VertexBuffer::new(&window.display, &teapot::NORMALS).unwrap();
    let indices = IndexBuffer::new(&window.display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    //load shader
    let shader = &program!(&window.display, 140 => { vertex: VERTEX, fragment: FRAGMENT }).expect("failed to load toon shader");
    let draw_parameters = &DrawParameters{
        blend: Blend::alpha_blending(),
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        multisampling: false,
        depth:  glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };
    
    loop {
        let elapsed = now.elapsed().as_secs_f32();

           

        //using glium
        let frame = window.new_frame();
        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        
        let view_matrix = Matrix::view_matix(frame.get_dimensions(), 3.141592 / 3.0, 1024.0, 0.1);

        let rot = Quaternion::from_axis_rotation(elapsed, vec3(0.57, 0.57, -0.57));
        let pos = vec3(0.0, elapsed.sin(), 20.0);
        let scale = vec3(0.1, elapsed.sin()/10.0 + 0.12, 0.1);
        let trans_matrix = Matrix::from_transform(pos, scale, rot);

        let uniforms = &uniform! { matrix: trans_matrix, view: view_matrix };
        frame.draw((&vertices, &normals), &indices, shader, uniforms, draw_parameters).unwrap();
        
        if let PumpStatus::Exit(_) = window.update() { break; }
        thread::sleep(ONE_60TH_OF_A_SECOND);
    }
}