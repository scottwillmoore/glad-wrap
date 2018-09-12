// The "idea" of a context.

// There is no way to create an OpenGL object without it being stored in the Context.
// When an OpenGL object is created the context ensures it stays alive for as long as it needs too.
// ^ Do we need a context for this? I feel like ::new() and ::drop() are good enough for this case.

// A context may be better for ensuring we only access in the right thread and at the right times?

// Hmmmmh, still don't quite get what is the "purpose" of using a Context.
// I believe most guarentees can be maintained without the Context - but maybe some stuff cannot...

// ...

// Classes of OpenGL errors:
// Wrong argument order or types. Not as easy to do in Rust.
// Forgetting to do a step, e.g. generate the buffer.
// Forgetting to bind the buffer - or binding the wrong buffer.
// glGenBuffers(...)
// glBindBuffer(...)
// glBufferData(...)

type Handle<T> = Rc<T>;
type WeakHandle<T> = Weak<T>;

struct Context {
    buffers: Vec<WeakHandle<T>>,
}
