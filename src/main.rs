use std::{thread, time::{Duration, Instant}};
use glium::{index::{self, PrimitiveType}, Surface};
use window::vec2;
use winit::platform::pump_events::PumpStatus;
use crate::window::{Colour, Window};

mod shaders;
mod window;
fn main(){
    let mut window = Window::new();
    let now = Instant::now();
    const ONE_60TH_OF_A_SECOND: Duration = Duration::from_millis(1000/60);
    
    let shape = vec![vec2(-0.5, -0.5), vec2(0.0, 0.5), vec2(0.5, -0.25)];
    let indices = index::NoIndices(PrimitiveType::TrianglesList).into();
    let mesh = window.gen_mesh(&shape, indices);

    let shader = shaders::SolidColour::new(Colour::new(0.0, 0.5, 0.6, 1.0), &window);
    loop {
        //using with glium
        let frame = window.new_frame();
        frame.clear_color(0.0, now.elapsed().as_secs_f32(), 0.0, 1.0);
        
        window.draw_with_shader(&mesh, &shader);

        if let PumpStatus::Exit(_) = window.update() { break; }
        thread::sleep(ONE_60TH_OF_A_SECOND);
    }
}