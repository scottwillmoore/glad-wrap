use gl::types::*;

#[derive(Debug, Clone, Copy)]
pub enum BufferTarget {
    ArrayBuffer,
    ElementArrayBuffer,
    UniformBuffer,
}

impl Into<GLenum> for BufferTarget {
    fn into(self) -> GLenum {
        use crate::BufferTarget::*;
        match self {
            ArrayBuffer => gl::ARRAY_BUFFER,
            ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            UniformBuffer => gl::UNIFORM_BUFFER,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    DynamicCopy,
    DynamicDraw,
    DynamicRead,
    StaticCopy,
    StaticDraw,
    StaticRead,
    StreamCopy,
    StreamDraw,
    StreamRead,
}

impl Into<GLenum> for BufferUsage {
    fn into(self) -> GLenum {
        use crate::BufferUsage::*;
        match self {
            DynamicCopy => gl::DYNAMIC_COPY,
            DynamicDraw => gl::DYNAMIC_DRAW,
            DynamicRead => gl::DYNAMIC_READ,
            StaticCopy => gl::STATIC_COPY,
            StaticDraw => gl::STATIC_DRAW,
            StaticRead => gl::STATIC_READ,
            StreamCopy => gl::STREAM_COPY,
            StreamDraw => gl::STREAM_DRAW,
            StreamRead => gl::STREAM_READ,
        }
    }
}

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn new<T>(data: &[T], target: BufferTarget, usage: BufferUsage) -> Buffer {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        let buffer = Buffer { id };

        buffer.bind(target);
        buffer.buffer_data(data, target, usage);

        buffer
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self, target: BufferTarget) {
        unsafe { gl::BindBuffer(target.into(), self.id) };
    }

    fn buffer_data<T>(&self, data: &[T], target: BufferTarget, usage: BufferUsage) {
        unsafe {
            gl::BufferData(
                target.into(),
                (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage.into(),
            )
        };
    }

    fn buffer_sub_data<T>(&self, data: &[T], offset: usize, target: BufferTarget) {
        unsafe {
            gl::BufferSubData(
                target.into(),
                offset as GLintptr,
                (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
            )
        };
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}
