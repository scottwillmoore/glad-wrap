extern crate gl;
extern crate mini_engine;

use gl::types::*;
use mini_engine::*;

fn create_shader_program() {}

fn main() {
    let title = "Example: Draw Triangle";
    let dimensions = (1024.0, 760.0);
    let mut window = Window::new(title, dimensions);

    let program = Program::from_file(&["resources/triangle.vs", "resources/triangle.fs"]);

    window.run(|| unsafe {
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    });
}
