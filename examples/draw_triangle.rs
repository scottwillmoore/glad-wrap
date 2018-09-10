use mini_engine::*;

struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] },
];

fn main() {
    let title = "Example: Draw Triangle";
    let dimensions = (1024.0, 760.0);
    let mut window = Window::new(title, dimensions);

    let program = Program::from_file(&["resources/triangle.vs", "resources/triangle.fs"]).unwrap();

    let vertex_buffer = Buffer::new(
        TRIANGLE_VERTICES,
        BufferTarget::ArrayBuffer,
        BufferUsage::StaticDraw,
    );

    use crate::VertexAttributeType::*;
    let stride = std::mem::size_of::<Vertex>();
    let vertex_array = VertexArrayBuilder::new()
        .attribute(2, Float, false, stride, 0)
        .attribute(3, Float, false, stride, std::mem::size_of::<[f32; 2]>())
        .build(&vertex_buffer);

    window.run(|| {
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.bind();

        vertex_array.draw();
    });
}
