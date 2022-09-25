#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

struct Vertex {
    position: [f32; 3]
}

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Vertex's for a simple triangle
    let vert1 = Vertex { position: [-0.5, -0.5];
    let vert2 = Vertex { position: [0.0, 0.5];
    let vert3 = Vertex { position: [0.5, -0.25];
    let shape = vec![vert1, vert2, vert3];
    
    
    // Main Loop for GUI Window
    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
