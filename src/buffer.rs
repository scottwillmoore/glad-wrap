use gl::types::*;

#[derive(Debug, Clone, Copy)]
pub enum BufferTarget {
    ArrayBuffer,
}

impl Into<GLenum> for BufferTarget {
    fn into(self) -> GLenum {
        use crate::BufferTarget::*;
        match self {
            ArrayBuffer => gl::ARRAY_BUFFER,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    StaticDraw,
}

impl Into<GLenum> for BufferUsage {
    fn into(self) -> GLenum {
        use crate::BufferUsage::*;
        match self {
            StaticDraw => gl::STATIC_DRAW,
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
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}
