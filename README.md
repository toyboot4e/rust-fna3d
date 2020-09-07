# Rust-FNA3D

Wrapper of [FNA3D](https://github.com/FNA-XNA/FNA3D), the graphics library for [FNA](https://fna-xna.github.io/)

## About

Please refer to the [API documentation](https://docs.rs/rust-fna3d). It's for making a higher-level framework on it; take [ANF](https://github.com/toyboot4e/anf) as an example.

### Notes

* [docs/quick_start.md](https://github.com/toyboot4e/rust-fna3d/blob/master/docs/quick_start.md): how  to add dependency to Rust-FNA3D
* [docs/tips.md](https://github.com/toyboot4e/rust-fna3d/blob/master/docs/tips.md): using `cargo +nightly doc`
* [docs/wrapping_c.md](https://github.com/toyboot4e/rust-fna3d/blob/master/docs/wrapping_c.md): how I wrapped FNA3D in Rust
* [docs/referendes.md](https://github.com/toyboot4e/rust-fna3d/blob/master/docs/refs.md): other repositories using FNA3D

## State of this wrapper

Almost ready. Remaining tasks:

* [ ] Do not run `cmake` every time we run our project
* [ ] Publish it on crates.io and make sure it works fine
* [ ] Add more wrapper types rather than re-exporting raw types
* [ ] Add more methods to wrapper types
* [ ] `derive` more types

## Contact

Free free to contact with me. I love _any_ kind of improvements and anything will be welcomed!
