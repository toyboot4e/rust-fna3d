# fna3d-sys

Rust FFI bindings to FNA3D generated with [bindgen](https://github.com/rust-lang/rust-bindgen)

## Note

To compile FNA3D, we need to have `mojoshader_version.h` in `FNA3D/MojoShader`. It is generated when we run `camke` for `MojoShader`. However, crates.io doesn't allow us to generate the file when we build the crate. Therefore, we carefully publish this crate with `cargo publish --allow-dirty`, including `mojoshader_version.h`.

`mojoshader_version.h` is manually cached in `wrappers` directory and copied to `FNA3D/MojoShader` when building.
