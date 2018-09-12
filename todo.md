Priority goals.

[x] Use Rust 2018 - because why not.

[x] Create a gl::CheckError() macro - which is only used in debug mode.

[x] Move OpenGL interface into a new crate named 'glad-wrap'.

[ ] Create an OpenGL wrapper for a texture.

[ ] The use of RC to ensure binding is respected. An unbound buffer cannot be acted on.
    Do not worry about multithreaded OpenGL safety - since I am not an expert in the area.

Delayed goals.

[ ] Improve the safety of the OpenGL modules.
    E.g. Ensure the current thread has access to the OpenGL context.
    E.g. Ensure the vertex array has control over the buffers that it uses.

Move to a different project.

[ ] Begin working on higher level abstractions, such as a Mesh, Material, Camera, etc.
