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

// FNA3D_Image.h (exported as `img`)
pub mod img;

// mojoshader.h (exprted as `mojo`)
pub mod mojo;

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
            renderTargetUsage: enums::RenderTargetUsage::PlatformContents as u32,
        }
    }
}

#[allow(dead_code)]
pub mod colors {
    use super::Color;

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 0,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub const CORNFLOWER_BLUE: Color = Color {
        r: 100,
        g: 149,
        b: 237,
        a: 0,
    };
}