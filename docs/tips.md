# Tips for using `fna3d`

## Cargo

### `cargo +nightly doc`

[Infra Rustdoc Links](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html). Crates.io uses it by default.

### Duplicate crates detection

`cargo tree -d` prints duplicate crates. Cargo batches dependencies as much as possible thanks to the semvar [specification](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html). If not, it's time to update dependent crates.

## If you're new to graphics

You may want to know about rendering pipeline to use FNA3D. That can be learned by reading some tutorial on a specific low-level graphics API. One example is [learnopengl.com](https://docs.rs/bindgen/0.55.1/bindgen/struct.Builder.html#method.rustified_enum); it's a good read and it maps well to FNA3D although OpenGL is old.

> Vulkan is pretty lower-level and not suitable for learning to use FNA3D

You may want some bigger `struct`s than ones FNA3D provides. For example, resource binding struct or pipeline object as [Sokol](https://github.com/floooh/sokol/blob/master/sokol_gfx.h) does. [`miniquad`](https://docs.rs/miniquad/), which is inspired by Sokol, can also be a good learning resource.
