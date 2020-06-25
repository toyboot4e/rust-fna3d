//! Wrapper of FNA3D, which corresponds to `Microsoft.Xna.Framework.Graphics` namespace (?)
//!
//! This crate was mainly generated by [`bindgen`](https://crates.io/crates/bindgen). `fna3d`
//! corresponds to `FNA3D.h` and `fna3d::img` corresponds to `FNA3D_Image.h`.
//!
//! Internally, `build.rs` uses `bindgen` to generate forengin function interface in `ffi`.
//! Those bindings gemerated are autimatically included. If you want to see the generated file,
//! you can find it in `target`, or you can install the `bindgen` command line tool via `cargo` and
//! run it over a wrapping header file.

mod fna3d_device;
mod fna3d_enums;
mod fna3d_functions;
mod fna3d_structs;

pub use fna3d_sys as sys;

// FNA3D.h
pub use fna3d_device::*;
pub use fna3d_enums::*;
pub use fna3d_functions::*;
pub use fna3d_structs::*;

// FNA3D_Image.h
pub use fna3d_structs::img;

pub mod utils {
    use std::os::raw::c_void;

    use crate::fna3d_enums as enums;
    use fna3d_sys as sys;

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
