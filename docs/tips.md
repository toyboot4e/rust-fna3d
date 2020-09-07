# Tips for using `fna3d`

## Cargo

Use nightly version of `cargo doc` to build the document. It's for the [Infra Rustdoc Links](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html) feature. Crates.io also uses it by default.

## Graphics

If you're new to graphics:

* You may want to know about rendering pipeline to use FNA3D. That can be learned by reading some tutorial on a specific low-level graphics API. One example is [learnopengl.com](https://docs.rs/bindgen/0.55.1/bindgen/struct.Builder.html#method.rustified_enum); it's a good read and it maps well to FNA3D although OpenGL is old.

* You may want some bigger `struct`s than ones FNA3D provides. For example, resource binding struct or pipeline object as [Sokol](https://github.com/floooh/sokol/blob/master/sokol_gfx.h) does. [`miniquad`](https://docs.rs/miniquad/), which is inspired by Sokol, can also be a good learning resource.
