use std::{thread, time::{Duration, Instant}};
use glium::Surface;
use winit::platform::pump_events::PumpStatus;
use crate::window::Window;

mod window;
fn main(){
    let mut window = Window::new();
    let now = Instant::now();
    loop {
        let frame = window.new_frame();

        frame.clear_color(0.0, now.elapsed().as_secs_f32(), 0.0, 1.0);

        if let PumpStatus::Exit(_) = window.update(){
            break;
        }

        thread::sleep(Duration::from_millis(16));
    }
}