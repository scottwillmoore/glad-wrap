# Tasks to be completed...

Maybe, don't aim to be the safest, but at least be the most ergonomic.

# Goals:

- [ ] Make OpenGL ids safe to use. Cannot be modified, cannot be given to the wrong functions, etc.
- [ ] Restrict the domain of certain functions, such as glVertexAttribPointer(size: [1..4]).
- [ ] Remove the need for unsafe when interfacing with OpenGL.
- [ ] Usable with Vectors and Matrices.
- [ ] Ensure resources stay alive and don't invalidate other resources when destroyed.
- [ ] Ensure that the correct object is bound when methods are called. (Why do we care about this? Hard to enforce...)


## Current tasks:

- [ ] Ensure all objects return a handle (Rc<Object>) so that their lifetimes are okay.
- [ ] Ensure dropped buffers cannot invalidate vertex arrays that depend on the buffers.
- [ ] Refractor the buffer struct to be safer, and seperate creation from buffering data.
- [ ] Create a struct for an OpenGL texture.

## Postponed tasks:

- [ ] Ensure multi-threaded access of OpenGL data is prevented. More research required.

## Completed tasks:

- [x] Use Rust 2018 - because why not.
- [x] Create a gl::CheckError() macro.
- [x] Move OpenGL interface into a new crate named 'glad-wrap'.
