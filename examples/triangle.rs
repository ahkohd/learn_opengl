extern crate glium;
extern crate learn_opengl;

use glium::Surface;
use learn_opengl::{glwin, Vertex};

fn main() {
    glwin!("Example 001", 400, 400, |display: glium::Display| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

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

        // load vertices to video card
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        // link vertices to obtain triangles
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // let's write our vertex shader
        let vertex_shader = r#"
        #version 330 core

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#;

        // let's write our fragment shader
        let fragment_shader = r#"
            #version 330 core

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 1.0, 1.0);
            }
        "#;

        let program =
            glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

        target
            .draw(
                &vertex_buffer,
                indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    });
}
