# glad-wrap

[![crates.io-badge]][crates.io] &emsp; [![docs.rs-badge]][docs.rs]

[crates.io]: https://crates.io/crates/glad-wrap
[crates.io-badge]: https://img.shields.io/crates/v/glad-wrap.svg
[docs.rs]: https://docs.rs/glad-wrap
[docs.rs-badge]: https://docs.rs/glad-wrap/badge.svg

**An ergonomic (and mostly safe) OpenGL 3.2+ wrapper written in Rust.**

## Overview

...

### Goals

**Safe.**

- Be completely stateless. All state is passed to functions as parameters.

- Ensure the lifetimes of resources are guarenteed. For example: Do not leave a vertex array with an invalid buffer.

**Simple.**

- Eliminate the need for any calls to unsafe functions.

- Use high level functions to make tedious tasks easier. For example: Create and setup vertex array objects with a single function call.

**Low-level.**

- Try to mirror OpenGL functions and objects as close as possible. This should reduce overhead for users (who know OpenGL) that are attempting to use this library.

- Where possible have zero additional cost. Only compromise performance if safety is required.

**Cross-platform.**

- Work on Windows, MacOS and Linux.

## Getting Started

...

````rust
use glad-wrap::*;
````

More examples can be found in [`./examples`](https://github.com/scottwillmoore/glad-wrap/blob/master/examples).

## License

This project is licensed under the [MIT license](https://github.com/scottwillmoore/glad-wrap/blob/master/license).
