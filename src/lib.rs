extern crate glium;

mod macros;

use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub type RcRefCell<T> = std::rc::Rc<std::cell::RefCell<T>>;
