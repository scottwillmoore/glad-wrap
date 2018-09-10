Use Rust 2018 - because why not.

Move OpenGL wrappers into a new crate named 'glue'.

Create an OpenGL wrapper for a texture.

Begin working on higher level abstractions, such as a Mesh, Material, Camera, etc.

Create a gl::CheckError() macro - which is only used in debug mode.

Improve the safety of the OpenGL modules.
E.g. Ensure the current thread has access to the OpenGL context.
E.g. Ensure the vertex array has control over the buffers that it uses.

The use of RC to ensure binding is respected. An unbound buffer cannot be acted on.
Do not worry about multithreaded OpenGL safety - since I am not an expert in the area.

Fill out the enums in various places with all of the possible values.
E.g. BufferTarget only contains ArrayBuffer at the moment.