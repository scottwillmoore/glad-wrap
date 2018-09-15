# Design decisions to improve OpenGL

There are many safety concerns that can arise when using the OpenGL API which can be prevented (with hopefully zero-cost) using the Rust compiler. Below I have documented many of the potential problems that could occur when using vanilla OpenGL. For each problem, I have provided various solutions that can be implemented in Rust to prevent the problem from occuring.

The OpenGL wiki documents some of the [common mistakes](https://www.khronos.org/opengl/wiki/Common_Mistakes) that can occur when using OpenGL and provides potential solutions to these problems. This was used as a starting-point for this document.

### Invalid function arguments.

**Problem:**

*Throughout the OpenGL API it is very easy to cause errors by passing invalid arguments into OpenGL functions. OpenGL even defines two error types `GL_INVALID_ENUM` and `GL_INVALID_VALUE` to indicate that the wrong arguments were passed into a function.*

**Solution:**

All errors caused by *invalid enums* should be entirely preventable (at compile-time) with Rust's type system. We can define our own enums with an `Into<GLenum>` implementation to prevent ever passing an invalid enum into a function.

I also believe, all errors caused by *invalid values* should also be entirely preventable (at compile-time) with Rust's type system. This will not be as straight forward or easy as their are numerous different cases that can cause these issues. For example, out of range values, invalid buffer id's, etc. Many of these issues are addressed in other parts of this document.

### Incorrect order of operations.

**Problem:**

*It is possible for errors to arise when performing operations in the incorrect order. For example, attempting to upload data to a buffer before it has been bound, or attempting to draw elements before an (operational) vertex array has been bound.*

*More work will be required to document the many cases that an incorrect order of operations could cause unintended issues.*

**Solution:**

I am not sure if we can prevent all of these errors, as the problem is very open ended. I do believe we can eliminate a large subset of these errors through good API design.

For example, we can prevent uploading data to a buffer that hasn't been bound through the use of the type system. Uploading data to a buffer shouldn't be a method of `Buffer` it should be a method of `BoundBuffer`.

````rust
struct Buffer(GLuint);

struct BoundBuffer<'a> {
    buffer: &'a Buffer,
    target: BufferTarget,
}

impl Buffer {
    fn bind(&self, target: BufferTarget) -> BoundBuffer { ... }
}

impl BoundBuffer {
    // We cannot call this method on Buffer until it has been bound.
    fn buffer_data<T>(&self, data: &[T]) { ... }
}
````

### Invalidating a bound buffer by binding another buffer.

**Problem:**

*This links in with the 'Incorrect order of operations' problem which was discussed before. A BoundBuffer can be invalidated by binding another buffer to the same target. All operations on the first BoundBuffer now no longer modify the first buffer, but instead the second buffer.*

**Solution:**

This should also be possible to solve using Rust's lifetimes.

I still need to work through how this will work - and improve the ergonomics because at the moment it would still be quite tedius to use.

Current problems include:

* How do we indicate we are finished with a BoundBuffer. Currently another BoundBuffer cannot be created until the other bound buffer is out of scope (deallocated).

* We want to have multiple buffers bound provided that they are bound to different targets. We need this for functions such as `glCopyBufferSubData`.

* The syntax sudden becomes quite verbose in order to ensure all of these requirements are satisfied.

````rust
struct Buffer(GLuint);

struct BoundBuffer<'a, T: 'a> {
    buffer: &'a Buffer,
    target: BufferTarget,
    phantom: 
}

let b1 = Buffer::new();
let b2 = Buffer::new();

let bb1 = b1.bind(BufferTarget::ArrayBuffer);
let bb2 = b2.bind(BufferTarget::ArrayBuffer); // Error: cannot allocate another BoundBuffer as bb1 still exists.

bb2.buffer_data(&[1, 0, 0]);
````

### Deleting an object that is dependency of another object.

**Problem:**

*Throughout the OpenGL API there is often the case where an OpenGL object depends on another OpenGL object. If the depedency is deleted while the depdendant object still exists problems can arise. (I believe) OpenGL will keep the depedency allocated using its own reference counting mechanisms, but this is not a good idea.*

*For example, a vertex array object depends on multiple vertex buffers and one index buffer. If any of these buffers are deleted while the vertex array is still being used, there shouldn't be any problems. However, you do not have control over the deleted buffer anymore, which could be considered a memory leak. A vertex array is not considered a large object, but it could be the cause of a large amount of memory still being allocated.*

**Solution:**

Use Rust's `Rc<T>` to ensure that these buffers are not deleted (when they go out of scope) until all references of them throughout the program are dropped. A vertex array will take a `Rc<Buffer(GLuint)>` or `Buffer(Rc<GLuint>)` (the second would be cleaner) which will ensure that it has access to the buffer, and the buffer will not be deleted until it is finished with it.

````rust
struct Buffer(Rc<GLuint>);

struct VertexArray {
    // It owns the Buffer which can be created with Buffer::clone.
    vertices: Buffer,
}
````

This mirrors the internal representation of OpenGL buffers much better, and allows us to always have control over the buffers we allocate. This still creates an issue of 'invisible' allocation, whereby a vertex array could be the cause of a large chuck of memory being allocated.

**Solution:**

A better solution might be to use Rust's lifetimes to ensure that dependent objects cannot out-live the dependency. Therefore the Rust compiler will not let you compile if a dependency is dropped before the dependent object is dropped.

````rust
struct Buffer(GLuint);

struct VertexArray<'a> {
    // It owns a reference to the Buffer and cannot out-live the Buffer.
    vertices: &'a Buffer,
}
````

### Ensure the OpenGL context is created.

**Problem:**

If we call an OpenGL function before the context has been created then errors will occur. This isn't the greatest deal, as generally the programmer will ensure the context is created before calling OpenGL functions, but leaving this exposes makes this API potentially unsafe.

**Solution:**

To solve this I believe we would need a type to store the OpenGL context. The methods on the OpenGL context can only be called once a Context has been created through `Context::new` which ensures that the context is created.

````rust
struct Context { ... }

impl Context {
    fn new() -> Result<Context, ContextError> {
        // Ensure the OpenGL context is created successfully.
    }

    fn gen_buffers(&self, ...) {}
    fn bind_buffer(&self, ...) {}
    fn buffer_data(&self, ...) {}
    fn vertex_array_pointer(&self, ...) {}
    ...
}
````

In the case of our OpenGL objects we must pass in the context (to prove that the OpenGL context has been created). The context must be passed to any function calling an OpenGL function, or a reference to it can be stored in the Buffer struct (with the inccurred overhead).

````rust
struct Buffer(GLuint);

impl Buffer {
    fn new(context: &Context) -> Buffer {
        let id = 0;
        context.gen_buffers(&mut id);
        ...
    }
}
````

I do not believe this solution has been fully refined. It makes the API a lot more verbose for not much benefit. It also (I believe) inccurs overhead on our functions - so there might be a better way to do this.

### Do not use OpenGL objects from other threads.

**Problem:**

OpenGL should not be used from other threads*. At least for the majority of cases.

**Solution:**

Mark all of our structs with `!Send` and `!Sync` to ensure they cannot be given to over threads.

There is probably more to be said for this issue.