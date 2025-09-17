# glium-types
this is a crate designed to include all the things you need to easly work with glium.
it includes:
 - float, interger, boolean, unsigned interger with both regular and double vector variants for 2-4 dimesnions
 - square matrices and double square matrices for 2-4 dimensions
 - quaternions and double quaternions
 - vertex types: vertices, normals, vertex colours and texturecoords.

thats all the uniform types supported by glium! (excluding ones already present in glium e.g textures)

and to reduce boilerplate it also includes:
 - mesh macro
 - a premade vertex shader
 - 3d drawing parameters
 - debug teapot

```rust
use std::{thread, time::{Duration, Instant}};
use glium::{backend::glutin::SimpleWindowBuilder, uniform, DrawParameters, Program, Surface};
use glium_types::{prelude::*, teapot};
use winit::event::{Event, WindowEvent};

fn main() {
    // this is an outdated way to start winit. 
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let (indices, verts, norms) = mesh!(&display, &teapot::INDICES, &teapot::VERTICES, &teapot::NORMALS).unwrap();
    let program = Program::from_source(&display,
    "#version 140
    in vec3 position;
    in vec3 normal;
    
    uniform mat4 camera;
    uniform mat4 model;
    uniform mat4 perspective;

    out vec3 v_normal;

    void main() {
        // a neat mathimatical trick is that 4x4 matrices can apply position to a vec3 if you put
        // a 1 at the end of the vec3
        gl_Position = perspective * camera * model * vec4(position, 1);

        // since a normal doesnt need to have its position transformed we use just a 3x3 matrix
        // and we invert and transpose the matrix because normals have to be transformed
        // differently because they are directions, not positions
        mat3 norm_mat = transpose(inverse(mat3(camera * model)));
        v_normal = normalize(norm_mat * normal);
    }",
    "#version 140
    out vec4 colour;
    in vec3 v_normal;

    uniform vec3 light;
    void main(){
        colour = vec4(vec3(dot(normalize(v_normal), light)), 1.0);
    }", None).unwrap();

    let draw_parameters = DrawParameters {
        // teapot uses clockwise culling. most other models use anti clockwise culling
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
                let perspective = Mat4::perspective_3d(frame.get_dimensions(), 1.0, 1024.0, 0.1);
                // the camera matrix has to be inverted for things to display correcltly
                let camera = Mat4::from_pos(vec3(0.0, 0.0, -20.0)).inverse(); 
                
                // multiplying quaternions is equivelant to transformations,
                // so the bellow code will rotate around the z axis then x and then y.
                // this is also true for matrices
                let rot = Quat::from_y_rot(time)
                    * Quat::from_x_rot(time / 2.0)
                    * Quat::from_z_rot(time / 4.0);
                
                // moves up 50.0 then scales and then rotates
                let model = Mat4::from_rot(rot) * Mat4::from_scale(Vec3::splat(0.1))
                    * Mat4::from_pos(vec3(0.0, 50.0, 0.0));
        
                println!("teapot origin at {:?}", (model * vec4(0.0, 0.0, 0.0, 1.0)).truncate());

                // input for the vertex shader and our fragment shader
                let uniforms = uniform! { 
                    perspective: perspective, model: model, camera: camera,
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
```
