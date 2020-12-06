# Rust-FNA3D

Wrapper of [FNA3D](https://github.com/FNA-XNA/FNA3D), the graphics library for [FNA](https://fna-xna.github.io/)

## Crates

Choose `fna3d` or `fna3h`:

* `fna3d-sys`: Rust FFI to FNA3D generated with `bindgen`
* `fna3d`: Thin wrapper of `fna3d-sys`
* `fna3h`: `fna3d` types in a hierarchy (sub modules).

## About

Rust-FNA3D cares about desktop platforms and it works on macOS and Linux. To support Windows, I need to buy one.

For usage, please refer to the [API documentation](https://docs.rs/rust-fna3d).

## Status

Close to ready. Remaining tasks:

* [ ] Windows support of `build.rs`
* [ ] Add more wrapper types rather than re-exporting raw types
* [ ] Add more methods to wrapper types
* [ ] `derive` more types

## Contact

Free free to contact with me. I love _any_ kind of improvements and anything will be welcomed!
