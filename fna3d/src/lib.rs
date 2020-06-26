//! Wrapper of Rust FFI bindings to FNA3D, which corresponds to the most of the internal
//! implementation of `Microsoft.Xna.Framework.Graphics.GraphicsDevice`
//!
//! # Overview
//!
//! * `fna3d` corresponds to `FNA3D.h`
//! * `fna3d::img` corresponds to `FNA3D_Image.h`.
//!
//! `fna3d-sys` crate is re-exported as `fna3d::sys`, but probablly you don't have to touch it
//! directly.
//!
//! > NOTE: some methods require mutable references while they are NOT immutable. This is because C
//! pointers for non-constant values are translated as `*mut T`. We can actually define them as
//! `*T` but it requires us to modify the output by `bindgen`.
//!
//! # Guide to making a wrapper for C
//!
//! The follows are what Rust-FNA3D take care to wrap Rust-FNA3D-sys, which is Rust FFI bindings
//! to FNA3D generated with `bindgen`.
//!
//! > TODO: complete the following guide (I'm learning for now)
//!
//! ## Wrapping constants to an enum
//!
//! Consider such constants as an example:
//!
//! ```
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET: FNA3D_ClearOptions = 1;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER: FNA3D_ClearOptions = 2;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL: FNA3D_ClearOptions = 4;
//! pub type FNA3D_ClearOptions = u32;
//! ```
//!
//! We want to wrap them into an `enum`:
//!
//! ```
//! use enum_primitive::*;
//! use fna3d_sys as sys;
//! enum_from_primitive! {
//!     #[derive(Debug, Copy, Clone, PartialEq)]
//!     #[repr(u32)]
//!     pub enum ClearOptions {
//!         Target = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET,
//!         DepthBuffer = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER,
//!         Stencil = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL,
//!     }
//! }
//! ```
//!
//! `enum_primitive` crate was used to implement `ClearOptions::from_u32` automatically.
//!
//! ## Wrapping a struct
//!
//! Consider wrapping such a `struct` as an example:
//!
//! ```
//! use fna3d_sys::*;
//! #[repr(C)]
//! #[derive(Debug, Copy, Clone)]
//! pub struct FNA3D_DepthStencilState {
//!     // use boolean type to wrap this
//!     pub depthBufferEnable: u8,
//!     // ~~
//!     // use an enum type to wrap this:
//!     pub depthBufferFunction: FNA3D_CompareFunction,
//!     // ~~
//! }
//! ```
//!
//! First we'll make a struct:
//!
//! ```
//! use fna3d_sys as sys;
//!
//! #[derive(Debug, Clone)]
//! pub struct DepthStencilState {
//!     raw: sys::FNA3D_DepthStencilState,
//! }
//! ```
//!
//! ### Accessors
//!
//! * [x] use snake case
//! * [x] wrap enums and booleans
//!
//! ```ignore
//! impl DepthStencilState {
//!     // Use `bool` to wrap `u8`
//!     pub fn is_depth_buffer_enabled(&self) -> bool {
//!         self.raw.depthBufferEnable != 0
//!     }
//!
//!     pub fn set_is_depth_buffer_enabled(&mut self, b: bool) {
//!         self.raw.depthBufferEnable = b as u8;
//!     }
//!
//!     // Use `enums::CompareFunction` to wrap `FNA3D_CompareFunction` i.e. `u32`
//!     pub fn depth_buffer_function(&self) -> enums::CompareFunction {
//!         enums::CompareFunction::from_u32(self.raw.depthBufferFunction).unwrap()
//!     }
//!
//!     pub fn set_depth_buffer_function(&mut self, f: enums::CompareFunction) {
//!         self.raw.depthBufferFunction = f as u32;
//!     }
//! }
//! ```
//!
//! * [ ] wrap function pointers?
//! * [ ] take care to ownership?
//!
//! ### Trait implementations
//!
//! * [x] `Default`
//!
//! ## Output by `bindgen`
//!
//! Background of why we need to wrap structs
//!
//! ### enums and booleans
//!
//! Because C is not so strict about them, `bindgen` translates `enum` s as `u32` and `bool` s as
//! `u8`. We need to wrap them to make a cozy interface.
//!
//! `enum_primitive` is a great crate to make `enum` s from primitive values.
//!
//! ### Zero-sized structs
//!
//! They are used to represent a pointer type. Example:
//!
//! ```
//! #[repr(C)]
//! #[derive(Debug, Copy, Clone)]
//! pub struct FNA3D_Device {
//!     _unused: [u8; 0],
//! }
//! ```
//!
//! ### *void
//!
//! `c_void` is used to represent a function pointer.
//! There's a corresponding page in Rust nomicon:
//! [https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs)

pub use fna3d_sys as sys;

// FNA3D.h (re-exported to the root)
mod fna3d;
pub use crate::fna3d::fna3d_device::*;
pub use crate::fna3d::fna3d_enums::*;
pub use crate::fna3d::fna3d_functions::*;
pub use crate::fna3d::fna3d_structs::*;

// FNA3D_Image.h
pub mod img;

pub mod utils {
    use fna3d_sys as sys;
    use std::os::raw::c_void;

    use crate::fna3d::fna3d_enums as enums;

    // TODO: use trait
    pub trait AsVec4 {
        fn as_vec4(&self) -> sys::FNA3D_Vec4;
    }

    impl AsVec4 for sys::FNA3D_Color {
        fn as_vec4(&self) -> sys::FNA3D_Vec4 {
            sys::FNA3D_Vec4 {
                x: self.r as f32 / 255 as f32,
                y: self.g as f32 / 255 as f32,
                z: self.b as f32 / 255 as f32,
                w: self.a as f32 / 255 as f32,
            }
        }
    }

    /// `handle` is actually `SDL_Window*` in Rust-SDL2-sys
    pub fn params_from_window_handle(handle: *mut c_void) -> sys::FNA3D_PresentationParameters {
        let surface = enums::SurfaceFormat::Color;
        let stencil = enums::DepthFormat::D24S8;
        let target = enums::RenderTargetUsage::PlatformContents;
        let is_fullscreen = false;
        sys::FNA3D_PresentationParameters {
            backBufferWidth: 1280,
            backBufferHeight: 720,
            backBufferFormat: surface as u32,
            multiSampleCount: 1,
            // this is actually `SDL_Window*`
            deviceWindowHandle: handle,
            isFullScreen: is_fullscreen as u8,
            depthStencilFormat: stencil as u32,
            presentationInterval: enums::PresentInterval::Defalt as u32,
            displayOrientation: enums::DisplayOrientation::Defaut as u32,
            // FIXME:
            renderTargetUsage: target as u32,
        }
    }
}
