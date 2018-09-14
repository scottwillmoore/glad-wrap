# Tasks to be completed...

Maybe, don't aim to be the safest, but at least be the most ergonomic.

**Tasks:**

* Ensure all objects return a handle (`Rc<T>`) so that their lifetimes are okay.
* Ensure dropped buffers cannot invalidate vertex arrays that depend on the buffers.
* Refractor the buffer struct to be safer, and seperate creation from buffering data.
* Create a struct for an OpenGL texture.
* Ensure multi-threaded access of OpenGL data is prevented. More research required.


**Completed:**

* Use Rust 2018 -  because why not.
* Create a `gl::CheckError()` macro.
* Move OpenGL interface into a new crate named `glad-wrap`.