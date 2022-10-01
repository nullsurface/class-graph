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
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
 
    // Vertex's for a simple triangle
    let mut vert1 = Vertex { position: [-0.5, -0.5]};
    let mut vert2 = Vertex { position: [0.0, 0.5]};
    let mut vert3 = Vertex { position: [0.5, -0.25]};
    let mut vert4 = Vertex { position: [0.5, 0.25]};
    let mut shape = vec![vert1, vert2, vert3, vert4];
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut count = -0.5;
        
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
        if (count > 0.5) {
            count = -0.5;
        }

        vert1 = Vertex { position: [count + -0.5, -0.5]};
        vert2 = Vertex { position: [count + 0.0, 0.5]};
        vert3 = Vertex { position: [count + 0.5, -0.25]};
        vert4 = Vertex { position: [count + 0.5, 0.25]};

        shape = vec![vert1, vert2, vert3, vert4];
        vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap(); 
        frame.finish().unwrap();
    });
}
