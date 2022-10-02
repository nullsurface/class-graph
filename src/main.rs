#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    // Setup objects for GUI   
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 attr;
        uniform mat4 matrix;
        
        void main() {
            attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        in vec2 attr;

        void main() {
            color = vec4(attr, 0.0, 1.0);
        }
    "#;
 
    // Vertex's for a simple triangle
    let vert1 = Vertex { position: [-0.5, -0.5]};
    let vert2 = Vertex { position: [0.0, 0.5]};
    let vert3 = Vertex { position: [0.5, -0.25]};
    let vert4 = Vertex { position: [0.5, 0.25]};
    let shape = vec![vert1, vert2, vert3, vert4];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut count: f32 = -0.5;
        
    // Main Loop for GUI Window
    event_loop.run(move |ev, _, control_flow| {
        
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

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        count += 0.0002;
        if count > 0.5 {
            count = -0.5;
        }
        
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [count, 0.0, 0.0, 1.0f32],
            ]
        };
 
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap(); 
        frame.finish().unwrap();
    });
}
