//! Thin wrappers of Rust FFI bindings to FNA3D generated with `bindgen`
//!
//! TODO: complete the following guide (I'm learning for now)
//!
//! # How to make wrappers
//!
//! The follows are notes about wrapping Rust FFI generated with `bindgen`
//!
//! ## C
//!
//! ### Pointer types
//!
//! This is an example type from `bindgen`:
//!
//! ```csharp
//! pub struct FNA3D_Device {
//!     _unused: [u8; 0],
//! }
//! ```
//!
//! It's used to represent a pointer for the type. It's wrapped into a struct holding
//! `*mut FNA3D_Device` and andle destructing via `Drop` trait.
//!
//! ### *void
//!
//! `c_void` is used to represent a function pointer.
//! The Rust nomicon has a [corresponding page](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs).
//!
//! ### enums and booleans
//!
//! Because C is not so strict about them, `bindgen` translates `enum` s as `u32` and `bool` s as
//! `u8`. We need to wrap them so to
//!
//! ## Trait implementations
//!
//! `Default`

// TODO: remove `as u32` and maybe use `to_repr()`

use std::ptr;
// this should be `std::ffi::c_void` but `bindgen` uses:
use std::os::raw::c_void;

use crate::{fna3d_enums as enums, utils::AsVec4};
use enum_primitive::*;
use fna3d_sys as sys;

// --------------------------------------------------------------------------------
// Disposed types
//
// Those types have corresponding disposing function in `Device`

/// Disposed with a corresponding function in `Device`
pub type Buffer = sys::FNA3D_Buffer;
/// Disposed with a corresponding function in `Device`
pub type Renderbuffer = sys::FNA3D_Renderbuffer;
/// Disposed with a corresponding function in `Device`
pub type Effect = sys::FNA3D_Effect;
/// Disposed with a corresponding function in `Device`
pub type Query = sys::FNA3D_Query;
/// Disposed with a corresponding function in `Device`
pub type Texture = sys::FNA3D_Texture;

// --------------------------------------------------------------------------------
// Type aliases

// TODO: maybe wrap those types

pub type Viewport = sys::FNA3D_Viewport;

pub mod mojo {
    //! Aliases
    use fna3d_sys as sys;

    pub type Effect = sys::MOJOSHADER_effect;
    pub type EffectTechnique = sys::MOJOSHADER_effectTechnique;
    pub type EffectStateChanges = sys::MOJOSHADER_effectStateChanges;
}

pub type Color = sys::FNA3D_Color;
pub type Rect = sys::FNA3D_Rect;
pub type PresentationParameters = sys::FNA3D_PresentationParameters;

pub type BlendState = sys::FNA3D_BlendState;
pub type RasterizerState = sys::FNA3D_RasterizerState;
pub type SamplerState = sys::FNA3D_SamplerState;
pub type VertexElement = sys::FNA3D_VertexElement;
pub type VertexDeclaration = sys::FNA3D_VertexDeclaration;
pub type VertexBufferBinding = sys::FNA3D_VertexBufferBinding;
pub type RenderTargetBinding = sys::FNA3D_RenderTargetBinding;
pub type Vec4 = sys::FNA3D_Vec4;

// MONOSHADER_effect?

// --------------------------------------------------------------------------------
// Wrappers
//
// TODO: maybe make a macro to automate constructing such structs

/// Wraps `fna3d::FNA3d_DepthStencilState`
///
/// Hides boolean types exposed as `u8` and enumeration types exposed as `u32`
#[derive(Debug, Clone, PartialEq)]
pub struct DepthStencilState {
    pub depth_buffer_enable: bool,
    pub depth_buffer_write_enable: bool,
    pub depth_buffer_function: enums::CompareFunction,
    pub stencil_enable: bool,
    // TODO: maybe hide `i32` values?
    pub stencil_mask: i32,
    pub stencil_write_mask: i32,
    pub two_sided_stencil_mode: bool,
    pub stencil_fail: enums::StencilOperation,
    pub stencil_depth_buffer_fail: enums::StencilOperation,
    pub stencil_pass: enums::StencilOperation,
    pub stencil_function: enums::CompareFunction,
    pub ccw_stencil_fail: enums::StencilOperation,
    pub ccw_stencil_depth_buffer_fail: enums::StencilOperation,
    pub ccw_stencil_pass: enums::StencilOperation,
    pub ccw_stencil_function: enums::CompareFunction,
    pub reference_stencil: i32,
}

impl DepthStencilState {
    // FIXME: This creates needless copy from `&mut self`.
    pub fn as_sys_value(&self) -> sys::FNA3D_DepthStencilState {
        sys::FNA3D_DepthStencilState {
            depthBufferEnable: self.depth_buffer_enable as u8,
            depthBufferWriteEnable: self.depth_buffer_write_enable as u8,
            depthBufferFunction: self.depth_buffer_function as u32,
            stencilEnable: self.stencil_enable as u8,
            stencilMask: self.stencil_mask,
            stencilWriteMask: self.stencil_write_mask,
            twoSidedStencilMode: self.two_sided_stencil_mode as u8,
            stencilFail: self.stencil_fail as u32,
            stencilDepthBufferFail: self.stencil_depth_buffer_fail as u32,
            stencilPass: self.stencil_pass as u32,
            stencilFunction: self.stencil_function as u32,
            ccwStencilFail: self.ccw_stencil_fail as u32,
            ccwStencilDepthBufferFail: self.stencil_depth_buffer_fail as u32,
            ccwStencilPass: self.ccw_stencil_pass as u32,
            ccwStencilFunction: self.ccw_stencil_function as u32,
            referenceStencil: self.reference_stencil,
        }
    }
}

impl Default for DepthStencilState {
    //  .(true, true, .Less, false, 0, 0, .Keep, .Keep, .Keep, .Always, 0);
    // public static readonly DepthStencilState ZTestNoWrite = .(true, false, .Less, false, 0, 0, .Keep, .Keep, .Keep, .Always, 0);
    // public static readonly DepthStencilState None = .(false, false, .Always, false, 0, 0, .Keep, .Keep, .Keep, .Always, 0);
    fn default() -> Self {
        Self {
            depth_buffer_enable: true,
            depth_buffer_write_enable: true,
            depth_buffer_function: enums::CompareFunction::Less,
            stencil_enable: false,
            stencil_mask: 0,
            stencil_write_mask: 0,
            two_sided_stencil_mode: false, //
            stencil_fail: enums::StencilOperation::Keep,
            stencil_depth_buffer_fail: enums::StencilOperation::Keep,
            stencil_pass: enums::StencilOperation::Keep,
            stencil_function: enums::CompareFunction::Always,
            ccw_stencil_fail: enums::StencilOperation::Keep, //
            ccw_stencil_depth_buffer_fail: enums::StencilOperation::Keep, //
            ccw_stencil_pass: enums::StencilOperation::Keep, //
            ccw_stencil_function: enums::CompareFunction::Always, //
            reference_stencil: 0,
        }
    }
}

impl DepthStencilState {
    fn none() -> Self {
        Self {
            depth_buffer_enable: false,
            depth_buffer_write_enable: false,
            depth_buffer_function: enums::CompareFunction::Always,
            stencil_enable: false,
            stencil_mask: 0,
            stencil_write_mask: 0,
            two_sided_stencil_mode: false, //
            stencil_fail: enums::StencilOperation::Keep,
            stencil_depth_buffer_fail: enums::StencilOperation::Keep,
            stencil_pass: enums::StencilOperation::Keep,
            stencil_function: enums::CompareFunction::Always,
            ccw_stencil_fail: enums::StencilOperation::Keep, //
            ccw_stencil_depth_buffer_fail: enums::StencilOperation::Keep, //
            ccw_stencil_pass: enums::StencilOperation::Keep, //
            ccw_stencil_function: enums::CompareFunction::Always, //
            reference_stencil: 0,
        }
    }
}

// --------------------------------------------------------------------------------
// FNA3D_Image.h

pub mod img {
    // TODO: wrap them
    use fna3d_sys as sys;

    // type ImageSkipFunc = sys::FNA3D_Image_SkipFunc;
    // type ImageReadFunc = sys::FNA3D_Image_ReadFunc;
    // type ImageEofFunc = sys::FNA3D_Image_EOFFunc;

    // extern "C" {
    //     pub fn FNA3D_Image_Load(
    //         readFunc: FNA3D_Image_ReadFunc,
    //         skipFunc: FNA3D_Image_SkipFunc,
    //         eofFunc: FNA3D_Image_EOFFunc,
    //         context: *mut ::std::os::raw::c_void,
    //         w: *mut i32,
    //         h: *mut i32,
    //         len: *mut i32,
    //         forceW: i32,
    //         forceH: i32,
    //         zoom: u8,
    //     ) -> *mut u8;
    // }

    // extern "C" {
    //     pub fn FNA3D_Image_Free(mem: *mut u8);
    // }

    // pub type FNA3D_Image_WriteFunc = ::std::option::Option<
    //     unsafe extern "C" fn(
    //         context: *mut ::std::os::raw::c_void,
    //         data: *mut ::std::os::raw::c_void,
    //         size: i32,
    //     ),
    // >;

    // extern "C" {
    //     pub fn FNA3D_Image_SavePNG(
    //         writeFunc: FNA3D_Image_WriteFunc,
    //         context: *mut ::std::os::raw::c_void,
    //         srcW: i32,
    //         srcH: i32,
    //         dstW: i32,
    //         dstH: i32,
    //         data: *mut u8,
    //     );
    // }

    // extern "C" {
    //     pub fn FNA3D_Image_SaveJPG(
    //         writeFunc: FNA3D_Image_WriteFunc,
    //         context: *mut ::std::os::raw::c_void,
    //         srcW: i32,
    //         srcH: i32,
    //         dstW: i32,
    //         dstH: i32,
    //         data: *mut u8,
    //         quality: i32,
    //     );
    // }
}
