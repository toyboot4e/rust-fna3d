//! Wrapper of [FNA3D](https://github.com/FNA-XNA/FNA3D)
//!
//! Most functions are re-exported as [`Device`] methods.
//!
//! | modules       | corresponding header files |
//! |---------------|----------------------------|
//! | `fna3d`       | `fna3d.h`                  |
//! | `fna3d::img`  | `fna3d_image.h`            |
//! | `fna3d::mojo` | `mojoshader.h`             |
//!
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

pub mod docs;

pub mod utils {
    use fna3d_sys as sys;
    use std::os::raw::c_void;

    use crate::fna3d::fna3d_enums as enums;

    /// `handle` is actually `SDL_Window*` in Rust-SDL2-sys
    pub fn params_from_window_handle(
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
