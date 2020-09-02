//! Wrapper of [FNA3D](https://github.com/FNA-XNA/FNA3D)
//!
//! Most functionalities are re-exported as [`Device`] methods. First call
//! [`prepare_window_attributes`] then prepare your [`Device`].
//!
//! # About
//!
//! FNA3D is the 3D graphics library for [FNA](https://fna-xna.github.io/) written in C99.
//! `fna3d-sys` is Rust FFI to FNA3D and `fna3d` is a wrapper around `fna3d-sys`.
//!
//! ## Meta
//!
//! There's a [module](./_meta/index.html) that explains how I made this wrapper.
//!
//! ## TODOs
//!
//! * Provide with hierachy of modules
//!
//! [`prepare_window_attributes`]: ./fn.prepare_window_attributes.html
//! [`Device`]: ./struct.Device.html

pub use bitflags;
pub use enum_primitive;

// Rust FFI bindings to FNA3D. Probablly you don't have to touch it directly.
pub use fna3d_sys as sys;

// FNA3D.h (re-exported to the root)
mod fna3d;
pub use crate::fna3d::fna3d_device::*;
pub use crate::fna3d::fna3d_enums::*;
pub use crate::fna3d::fna3d_functions::*;
pub use crate::fna3d::fna3d_structs::*;

// FNA3D_Image.h (exported as `img`)
pub mod img;

// mojoshader.h (exprted as `mojo`)
pub mod mojo;

pub mod _meta;

pub mod utils {
    //! Helpers to get started with Rust-FNA3D
    //!
    //! * TODO: remove this module

    use fna3d_sys as sys;
    use std::os::raw::c_void;

    use crate::fna3d::fna3d_enums as enums;

    /// FIXME: is it really working?
    pub fn hook_log_functions_default() {
        unsafe {
            // info, warn, error respectively
            sys::FNA3D_HookLogFunctions(Some(log), Some(log), Some(log));
        }
        // ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>;
        unsafe extern "C" fn log(msg: *const ::std::os::raw::c_char) {
            let slice = ::std::ffi::CStr::from_ptr(msg);
            let string = slice.to_string_lossy().into_owned();
            println!("{}", string);
        }
    }

    /// `handle` is actually `SDL_Window*` in Rust-SDL2-sys
    pub fn default_params_from_window_handle(
        window_handle: *mut c_void,
    ) -> sys::FNA3D_PresentationParameters {
        sys::FNA3D_PresentationParameters {
            backBufferWidth: 1280,
            backBufferHeight: 720,
            backBufferFormat: enums::SurfaceFormat::Color as u32,
            multiSampleCount: 0,
            // this is actually `SDL_Window*`
            deviceWindowHandle: window_handle,
            isFullScreen: false as u8,
            depthStencilFormat: enums::DepthFormat::D24S8 as u32,
            presentationInterval: enums::PresentInterval::Default as u32,
            displayOrientation: enums::DisplayOrientation::Defaut as u32,
            renderTargetUsage: enums::RenderTargetUsage::DiscardContents as u32,
            // renderTargetUsage: enums::RenderTargetUsage::PlatformContents as u32,
        }
    }
}
