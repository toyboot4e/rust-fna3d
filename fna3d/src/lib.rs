//! Wrapper of [FNA3D], the graphics library for [FNA]
//!
//! It's for making a higher-level framework on it! Take [ANF] as an example.
//!
//! ## Usage
//!
//! First call [`prepare_window_attributes`] then prepare your [`Device`].
//!
//! ## What is `fna3d`?
//!
//! `fna3d` is a wrapper around [`fna3d-sys`], which is Rust FFI to [FNA3D] generated with
//! [bindgen].
//!
//!  [FNA3D] is the 3D graphics library for [FNA] written in C99. [FNA] is a reimplementation of
//! [XNA]. [XNA] is a famous game framework.
//!
//! ## What does Rust-FNA3D do?
//!
//! The biggest difference from the original FNA3D API is that [`Device`] strongly enforces the
//! ownership rule. Other changes are for improving the output `bindgen` with a thin layer:
//!
//! * It re-exports most functionalities as [`Device`] methods.
//! * It wraps the API with rusty types: slices, enums and booleans
//! * It wrap some legacy API with more meaningful one
//!
//! Details are noted in the[document]
//!
//! [ANF]: https://github.com/toyboot4e/anf
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [FNA]: https://fna-xna.github.io
//! [XNA]: https://en.wikipedia.org/wiki/Microsoft_XNA
//! [bindgen]: https://github.com/rust-lang/rust-bindgen
//! [document]: https://github.com/toyboot4e/rust-fna3d/docs/wrapping_c.md

pub use fna3d_sys as sys;

// FNA3D.h
mod fna3d;
pub use crate::fna3d::fna3d_device::*;
pub use crate::fna3d::fna3d_enums::*;
pub use crate::fna3d::fna3d_functions::*;
pub use crate::fna3d::fna3d_structs::*;

// FNA3D_Image.h
pub mod img;

// mojoshader.h (and some more)
pub mod mojo;

pub mod utils {
    //! Helpers

    pub use enum_primitive::FromPrimitive;

    use fna3d_sys as sys;
    use std::os::raw::c_void;

    use crate::fna3d::fna3d_enums as enums;

    /// Hooks default log functions to FNA3D
    ///
    /// FIXME: is it really working?
    pub fn hook_log_functions_default() {
        unsafe {
            // info, warn and error, respectively
            sys::FNA3D_HookLogFunctions(Some(log), Some(log), Some(log));
        }

        unsafe extern "C" fn log(msg: *const ::std::os::raw::c_char) {
            let slice = ::std::ffi::CStr::from_ptr(msg);
            let string = slice.to_string_lossy().into_owned();
            println!("{}", string);
        }
    }

    /// The argument `handle: *mut c_void` is often `*SDL_Window`
    pub fn default_params_from_window_handle(
        window_handle: *mut c_void,
    ) -> sys::FNA3D_PresentationParameters {
        sys::FNA3D_PresentationParameters {
            backBufferWidth: 1280,
            backBufferHeight: 720,
            backBufferFormat: enums::SurfaceFormat::Color as u32,
            multiSampleCount: 0,
            // this is actually `SDL_Window*` (though it's `*mut c_void`)
            deviceWindowHandle: window_handle,
            isFullScreen: false as u8,
            depthStencilFormat: enums::DepthFormat::D24S8 as u32,
            presentationInterval: enums::PresentInterval::Default as u32,
            displayOrientation: enums::DisplayOrientation::Defaut as u32,
            renderTargetUsage: enums::RenderTargetUsage::DiscardContents as u32,
            // renderTargetUsage: enums::RenderTargetUsage::PlatformContents as u32,
        }
    }

    bitflags::bitflags! {
        /// TODO: use this type in API
        pub struct ColorMask: u32 {
            const NONE = 1;
            const R = 1 << 0;
            const G = 1 << 1;
            const B = 1 << 2;
            const A = 1 << 3;
            const RGB = 0x7; // R | G | B
            const RGBA = 0xF; // R | G | B | A
            // const FORCE_U32 = 0x7FFFFFF;
        }
    }
}
