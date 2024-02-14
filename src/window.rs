use std::time::Duration;

use glium::{glutin::surface::WindowSurface, Display, Frame};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

#[allow(dead_code)]
///Used to create a window and draw things to it using the **glium** library. In the future it will suport inputs.
pub struct Window{
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    display: Display<WindowSurface>,
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
}