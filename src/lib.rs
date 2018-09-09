extern crate gl;
extern crate glutin;

// TODO: Create structs for Camera, Mesh, Material, Text, etc. The real basics.

// Move my mini OpenGL wrapper into its own crate - but still in this repository.
// TODO: Revise OpenGL wrapper names.
// TODO: Create a gl::CheckError() macro - which is only used in debug mode.
// TODO: Improve safety of wrappers - ensure context, etc.

mod buffer;
mod program;
mod shader;
mod vertex_array;
mod window;

pub use buffer::{Buffer, BufferTarget, BufferUsage};
pub use program::Program;
pub use shader::{Shader, ShaderType};
pub use vertex_array::{VertexArray, VertexArrayBuilder, VertexAttributeType};
pub use window::Window;
