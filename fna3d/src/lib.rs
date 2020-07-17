//! Wrapper of FNA3D. Most functions are re-exported as methods of `Device`.
//!
//! * `fna3d` corresponds to `FNA3D.h`
//! * `fna3d::img` corresponds to `FNA3D_Image.h`.
//!
//! > NOTE: some methods require mutable references while they are NOT immutable. This is because C
//! pointers for non-constant values are translated as `*mut T`. We can actually define them as
//! `*const T` but it requires us to modify the output by `bindgen`.

// Rust FFI bindings to FNA3D. Probablly you don't have to touch it directly.
pub use fna3d_sys as sys;

// FNA3D.h (re-exported to the root)
mod fna3d;
pub use crate::fna3d::fna3d_device::*;
pub use crate::fna3d::fna3d_enums::*;
pub use crate::fna3d::fna3d_functions::*;
pub use crate::fna3d::fna3d_structs::*;

// FNA3D_Image.h
pub mod img;

pub mod docs;

pub mod utils {
    use fna3d_sys as sys;
    use std::os::raw::c_void;

    use crate::fna3d::fna3d_enums as enums;

    // TODO: remove this trait and wrap `FNA3D_Color`
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
