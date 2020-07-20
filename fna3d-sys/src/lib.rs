//! Rust FFI bindings to FNA3D generated with [bindgen](https://github.com/rust-lang/rust-bindgen)
//!
//! The `sys` name came from the [naming convension](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages).
//! Note that the package name is `fna3d-sys` while the library name is `fna3d_sys`.
//!
//! # The build script (`build.rs`)
//!
//! Internally, `build.rs` uses `bindgen` to generate forengin function interface in `ffi`.
//! Those bindings gemerated are autimatically included. If you want to see the generated file,
//! you can find it in `target`, or you can install the `bindgen` command line tool via `cargo` and
//! run it over a wrapping header file.

// Supress casing errors
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include generated bindings
include!(concat!(env!("OUT_DIR"), "/fna3d_bindings.rs"));

pub mod mojo {
    //! MojoShader

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
