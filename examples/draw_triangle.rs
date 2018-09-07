extern crate mini_engine;

use mini_engine::Window;

fn main() {
    let title = "Example: Draw Triangle";
    let dimensions = (1024.0, 760.0);
    let mut window = Window::new(title, dimensions);

    while !window.should_close() {
        window.swap_buffers();
    }
}
