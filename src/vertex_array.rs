use gl::types::*;

use crate::buffer::{Buffer, BufferTarget};

// Fun fact: normalised must be false for HalfFloat, Float, Double, Fixed.
// Can we enforce this requirement?
#[derive(Debug, Clone, Copy)]
pub enum VertexAttributeType {
    Byte,
    Double,
    Fixed,
    Float,
    HalfFloat,
    Int,
    Short,
    UnsignedByte,
    UnsignedInt,
    UnsignedShort,
}

impl Into<GLenum> for VertexAttributeType {
    fn into(self) -> GLenum {
        use crate::VertexAttributeType::*;
        match self {
            Byte => gl::BYTE,
            Double => gl::DOUBLE,
            Fixed => gl::FIXED,
            Float => gl::FLOAT,
            HalfFloat => gl::HALF_FLOAT,
            Int => gl::INT,
            Short => gl::SHORT,
            UnsignedByte => gl::UNSIGNED_BYTE,
            UnsignedInt => gl::UNSIGNED_INT,
            UnsignedShort => gl::UNSIGNED_SHORT,
        }
    }
}

struct VertexAttribute {
    size: u32,
    kind: VertexAttributeType,
    normalised: bool,
    stride: usize,
    offset: usize,
}

pub struct VertexArrayBuilder {
    attributes: Vec<VertexAttribute>,
}

impl VertexArrayBuilder {
    pub fn new() -> VertexArrayBuilder {
        VertexArrayBuilder {
            attributes: Vec::new(),
        }
    }

    pub fn attribute(
        mut self,
        size: u32,
        kind: VertexAttributeType,
        normalised: bool,
        stride: usize,
        offset: usize,
    ) -> VertexArrayBuilder {
        self.attributes.push(VertexAttribute {
            size,
            kind,
            normalised,
            stride,
            offset,
        });

        self
    }

    pub fn build(self, buffer: &Buffer) -> VertexArray {
        VertexArray::new(buffer, &self.attributes)
    }
}

// A VertexArray should have ownership of the buffer used.
// At the very least, it should be able to ensure the buffer is not deleted while being used.
//
// Solution: Use reference counted buffers, Rc<Buffer>.
// A vertex array takes a vertex buffer (ArrayBuffer) and a index buffer (ElementArrayBuffer).
//
// This may be better if these differences were strongly typed.
// E.g. struct Buffer<T> => struct Buffer<ArrayBuffer> => type ArrayBuffer.
// But, why do we need strongly typed buffers, what guarantees does this provide us? I can't actually think of any.
// Can a buffer not be a ArrayBuffer and an IndexBuffer (theoretically, although doubtful in practice).
//
// Implement it without strongly typed buffers for the moment. Think about it.
pub struct VertexArray {
    id: GLuint,
    // vertex_buffer: Rc<Buffer>,
    // index_buffer: Option<Rc<Buffer>>,
    // TODO: Implement this...
}

impl VertexArray {
    fn new(vertex_buffer: &Buffer, attributes: &[VertexAttribute]) -> VertexArray {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) };
        let vertex_array = VertexArray { id };

        vertex_array.bind();
        vertex_buffer.bind(BufferTarget::ArrayBuffer);

        for (index, attribute) in attributes.iter().enumerate() {
            let index = index as GLuint;
            let normalised = if attribute.normalised {
                gl::TRUE
            } else {
                gl::FALSE
            };

            unsafe {
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    attribute.size as GLint,
                    attribute.kind.into(),
                    normalised,
                    attribute.stride as GLint,
                    attribute.offset as *const GLvoid,
                );
            }
        }

        vertex_array
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) };
    }

    pub fn draw(&self) {
        self.bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3) };
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) };
    }
}
