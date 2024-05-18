use std::slice::EscapeAscii;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ ActiveEventLoop, ControlFlow, EventLoop },
    keyboard::{ KeyCode, PhysicalKey },
    window::Window,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent
    ) {
        match event {
            WindowEvent::CloseRequested => {
                print!("Close requested event");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                //Below copied from docs
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                //
                // Draw.
                //
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        println!("Escape key pressed");
                        event_loop.exit();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    //Control flow Wait will pauses the event loop until an event is raised
    //Control flow Poll continously runs the event loop, even if there are no events
    //Poll is used for things like games, Wait uses less compute (power/CPU) and won't update all the time.
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app);
}
