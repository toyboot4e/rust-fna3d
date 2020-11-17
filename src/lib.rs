//! Wrapper of [FNA3D], the graphics library for [FNA]
//!
//! See [examples](https://github.com/toyboot4e/rust-fna3d/tree/master/examples) to get started.
//!
//! ## What is `fna3d`?
//!
//! `fna3d` is a wrapper around `fna3d-sys`, which is Rust FFI to [FNA3D] generated with [bindgen].
//!
//!  [FNA3D] is the 3D graphics library for [FNA] written in C99 in 2020. [FNA] is a
//! reimplementation of [XNA]. [XNA] is a famous game framework.
//!
//! ## What does Rust-FNA3D do?
//!
//! [`Device`] is reference counted and it drops FNA3D device when they go out of scope.
//!
//! Other changes are trivial; they're just for improvements to the default output of `bindgen`:
//!
//! * Wrapping the original API with rusty types: slices, enums and booleans
//! * Wrapping some legacy API (XNA-compatibility API) with more meaningful one
//!
//! Details are noted in this [file] on GitHub.
//!
//! [ANF]: https://github.com/toyboot4e/anf
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [FNA]: https://fna-xna.github.io
//! [XNA]: https://en.wikipedia.org/wiki/Microsoft_XNA
//! [bindgen]: https://github.com/rust-lang/rust-bindgen
//! [file]: https://github.com/toyboot4e/rust-fna3d/blob/master/docs/wrapping_c.md

mod fna3d;
pub mod img;
pub mod mojo;

pub use crate::fna3d::{fna3d_device::*, fna3d_enums::*, fna3d_functions::*, fna3d_structs::*};
pub use {bitflags, fna3d_sys as sys};

pub mod utils {
    //! Helpers

    pub use {
        enum_primitive_derive::Primitive,
        num_traits::{FromPrimitive, ToPrimitive},
    };

    use {fna3d_sys as sys, std::os::raw::c_void};

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
            // log::warn!("{}", string);
        }
    }

    /// The argument `handle: *mut c_void` is often `*SDL_Window`
    pub fn default_params_from_window_handle(
        window_handle: *mut c_void,
    ) -> sys::FNA3D_PresentationParameters {
        let (w, h) = crate::get_drawable_size(window_handle);

        sys::FNA3D_PresentationParameters {
            backBufferWidth: w as i32,
            backBufferHeight: h as i32,
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

    pub fn no_change_effect() -> crate::mojo::EffectStateChanges {
        crate::mojo::EffectStateChanges {
            render_state_change_count: 0,
            render_state_changes: std::ptr::null(),
            sampler_state_change_count: 0,
            sampler_state_changes: std::ptr::null(),
            vertex_sampler_state_change_count: 0,
            vertex_sampler_state_changes: std::ptr::null(),
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
