extern crate glium;
extern crate learn_opengl;

use glium::Surface;
use learn_opengl::glwin;

fn main() {
    glwin!("Example 001", 400, 400, |display: glium::Display| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    });
}
