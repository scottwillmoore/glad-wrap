use gl::types::*;

type BufferHandle = Rc<Buffer>;

struct BoundBuffer {
    buffer: BufferHandle,
    target: GLenum,
}

impl BoundBuffer {
    fn buffer_data<T>(data: &[T]) {}
}

struct Buffer(GLuint);

impl Buffer {
    fn new() -> BufferHandle {}
    fn bind() -> BoundBuffer {}
}

fn example() {
    let triangles = Buffer::new();
    let sqaures = Buffer::new();

    triangles.bind(ArrayBuffer);
    squares.bind(ArrayBuffer);
    triangles.buffer_data(); // This modifies squares, not triangles.

    context.bind(triangles, ArrayBuffer);
    context.bind(squares, ArrayBuffer); // ArrayBuffer already bound? Can we do this with Rust lifetimes?

    let triangles = vertices.bind();
    editor.buffer_data(&[1, 2, 3]);
    editor.finish();
}
