/*! Rust FFI bindings to FNA3D generated with [bindgen]

WARNING: Probablly it doesn't compile on Window. I need to buy one.

I'm using `bindgen` with  default settings, but if you're interested, see the API documentation
of [`Builder`] to configure enum generation.

[bindgen]:  https://github.com/rust-lang/rust-bindgen
[`Builder`]: https://docs.rs/bindgen/newest/bindgen/struct.Builder.html
*/

// suppress all errors
#![allow(warnings)]

// Include generated bindings
include!("ffi/fna3d_bindings.rs");

pub mod mojo {
    //! MojoShader types
    //!
    //! These types are concrete while mojoshader types under `crate::` don't tell the field types.
    include!("ffi/mojoshader_bindings.rs");
}

#[cfg(test)]
mod test {
    /// Makes sure we can link to FNA3D. Fails if we can't
    #[test]
    fn test_link() {
        let v = unsafe { super::FNA3D_LinkedVersion() };
        println!("FNA3D version: {}", v);
    }
}
