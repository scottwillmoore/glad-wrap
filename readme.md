# glad-wrap

[![crates.io-badge]][crates.io] &emsp; [![docs.rs-badge]][docs.rs]

[crates.io]: https://crates.io/crates/glad-wrap
[crates.io-badge]: https://img.shields.io/crates/v/glad-wrap.svg
[docs.rs]: https://docs.rs/glad-wrap
[docs.rs-badge]: https://docs.rs/glad-wrap/badge.svg

**An ergonomic (and mostly safe) OpenGL 3.3+ wrapper written in Rust.**

## Overview

Please read the [`design.md`](https://github.com/scottwillmoore/glad-wrap/blob/master/design.md) document which should explain why this library has been created and what problems it addresses.

I would **love** feedback on this document! What other problems can you think of? Is there a better way to solve some of these problems?

## Goals

For now the project goals are:

* Solve all the problems in the [`design.md`](https://github.com/scottwillmoore/glad-wrap/blob/master/design.md) document.

* Ensure the library is nice (easy and concise) to use.

* Remove the need for any unsafe function calls to OpenGL.

## Getting Started

````rust
use glad-wrap::*;

// TODO: Finish the example.
````

More examples can be found in [`./examples`](https://github.com/scottwillmoore/glad-wrap/blob/master/examples).

## License

This project is licensed under the [MIT license](https://github.com/scottwillmoore/glad-wrap/blob/master/license).
