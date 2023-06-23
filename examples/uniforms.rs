extern crate glium;
extern crate learn_opengl;

use glium::{uniform, Surface};
use learn_opengl::{glwin, rc, RcRefCell, Vertex};

fn main() {
    rc!(t, f32, -0.5);

    glwin!(
        "Example 002",
        400,
        400,
        |display: glium::Display, t: RcRefCell<f32>| {
            let shape = vec![
                Vertex {
                    position: [-0.5, -0.5],
                },
                Vertex {
                    position: [0.0, 0.5],
                },
                Vertex {
                    position: [0.5, -0.25],
                },
            ];

            let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            let vertex_shader_src = r#"
            #version 330 core

            in vec2 position;

            uniform float t;

            void main() {
                vec2 pos = position;
                pos.x += t;
                gl_Position = vec4(pos, 0.0, 1.0);
            }
            "#;

            let fragment_shader_src = r#"
            #version 330 core 

            out vec4 color;

            void main() {
                color = vec4(0.0, 1.0, 1.0, 0.1);
            }
            "#;

            let program =
                glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                    .unwrap();

            *t.borrow_mut() += 0.002;

            if *t.borrow() > 0.5 {
                *t.borrow_mut() = -0.5;
            }

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);

            target
                .draw(
                    &vertex_buffer,
                    indices,
                    &program,
                    &uniform! { t: *t.borrow() },
                    &Default::default(),
                )
                .unwrap();

            target.finish().unwrap();
        },
        t
    );
}
