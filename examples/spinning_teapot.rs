use std::{thread, time::{Duration, Instant}};
use glium::{backend::glutin::SimpleWindowBuilder, uniform, DrawParameters, Program, Surface};
use glium_types::{prelude::*, teapot};
use winit::event::{Event, WindowEvent};

fn main(){
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let (indices, verts, norms) = mesh!(&display, &teapot::INDICES, &teapot::VERTICES, &teapot::NORMALS);
    let program = Program::from_source(&display, shaders::VERTEX,
    "#version 140
    out vec4 colour;
    in vec3 v_normal;

    uniform vec3 light;
    void main(){
        colour = vec4(vec3(dot(normalize(v_normal), light)), 1.0);
    }", None).unwrap();

    let draw_parameters = DrawParameters{
        //teapot uses clockwise culling. most other models use anti clockwise culling
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        ..params::alias_3d()
    };
    
    let time = Instant::now();
    event_loop.run(|event, target| {
        match event {
            Event::AboutToWait => {
                let time = time.elapsed().as_secs_f32();
                display.resize(window.inner_size().into());
                let mut frame = display.draw();
                let view = Mat4::view_matrix_3d(frame.get_dimensions(), 1.0, 1024.0, 0.1);
                let camera = Mat4::from_pos(vec3(0.0, 0.0, 20.0));
                
                // multiplying quaternions is equivelant to transformations,
                // so the bellow code will rotate around the z axis then x and then y.
                // this also works for matrices
                let rot = Quaternion::from_y_rot(time)
                    * Quaternion::from_x_rot(time / 2.0)
                    * Quaternion::from_z_rot(time / 4.0);
                
                //moves up 50.0 then scales and rotates.
                let model = Mat4::from_rot(rot) * Mat4::from_scale(Vec3::splat(0.1))
                    * Mat4::from_pos(vec3(0.0, 50.0, 0.0));
        
                println!("teapot origin at {:?}", vec4(0.0, 0.0, 0.0, 1.0).transform(&model).truncate());

                //input for the vertex shader and our fragment shader.
                let uniforms = uniform! { 
                    view: view, model: model, camera: camera,
                    light: vec3(0.5, 1.0, -0.5).normalise()
                };

                frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                frame.draw((&verts, &norms), &indices, &program, &uniforms, &draw_parameters).unwrap();
                
                frame.finish().unwrap();
                
                //throttle speed of rendering
                thread::sleep(Duration::from_millis(16));
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => target.exit(),
            _ => ()
        }
    }).unwrap();
}
