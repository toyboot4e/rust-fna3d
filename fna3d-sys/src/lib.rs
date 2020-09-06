//! Rust FFI bindings to FNA3D generated with [bindgen]
//!
//! The `sys` name came from the [naming convension] -- though FNA3D is not a system library. Note
//! that the package name is `fna3d-sys` while the library name is `fna3d_sys`.
//!
//! # The build script (`build.rs`)
//!
//! Internally, `build.rs` uses `bindgen` to generate foreign function interface (FFI). Those
//! bindings gemerated are autimatically included via `include!` macro. If you want to see the
//! contents of generated files, you can find it in `target`, or you can can run `bindgen` command,
//! which can be installed with `cargo`, over header files.
//!
//! [bindgen]:  https://github.com/rust-lang/rust-bindgen
//! [naming convension]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages

// Supress casing errors
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include generated bindings
include!(concat!(env!("OUT_DIR"), "/fna3d_bindings.rs"));

pub mod mojo {
    //! MojoShader (strictly typed)

    include!(concat!(env!("OUT_DIR"), "/mojoshader_bindings.rs"));
}

#[cfg(test)]
mod test {
    /// Makes sure the link to FNA3D (fails if FNA3D is not linked)
    #[test]
    fn test_link() {
        let v = unsafe { super::FNA3D_LinkedVersion() };
        println!("FNA3D version: {}", v);
    }
}
