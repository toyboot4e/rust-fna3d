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

// TODO: add constructors, remove AsVec4
pub type Color = sys::FNA3D_Color;
pub type Rect = sys::FNA3D_Rect;
pub type PresentationParameters = sys::FNA3D_PresentationParameters;

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
// We _could_ use macors to define field accessors. Probablly the
// [paste](https://github.com/dtolnay/paste) is usefule for that. However, I prefered explicit
// definitions this time.

// ----------------------------------------
// BlendState

#[derive(Debug, Clone)]
pub struct BlendState {
    pub color_source_blend: enums::Blend,
    pub color_destination_blend: enums::Blend,
    pub color_blend_function: enums::BlendFunction,
    //
    pub alpha_source_blend: enums::Blend,
    pub alpha_destination_blend: enums::Blend,
    pub alpha_blend_function: enums::BlendFunction,
    //
    pub color_write_enable: enums::ColorWriteChannels,
    pub color_write_enable1: enums::ColorWriteChannels,
    pub color_write_enable2: enums::ColorWriteChannels,
    pub color_write_enable3: enums::ColorWriteChannels,
    //
    pub blend_factor: Color,
    pub multi_sample_mask: i32,
}

impl BlendState {
    pub fn as_sys_value(&self) -> sys::FNA3D_BlendState {
        sys::FNA3D_BlendState {
            colorSourceBlend: self.color_source_blend as u32,
            colorDestinationBlend: self.color_destination_blend as u32,
            colorBlendFunction: self.color_blend_function as u32,
            //
            alphaSourceBlend: self.alpha_source_blend as u32,
            alphaDestinationBlend: self.alpha_destination_blend as u32,
            alphaBlendFunction: self.alpha_blend_function as u32,
            //
            colorWriteEnable: self.color_write_enable as u32,
            colorWriteEnable1: self.color_write_enable1 as u32,
            colorWriteEnable2: self.color_write_enable2 as u32,
            colorWriteEnable3: self.color_write_enable3 as u32,
            //
            blendFactor: self.blend_factor,
            multiSampleMask: self.multi_sample_mask,
        }
    }
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
            color_source_blend: enums::Blend::SourceAlpha,
            color_destination_blend: enums::Blend::InverseSourceAlpha,
            color_blend_function: enums::BlendFunction::Add,
            //
            alpha_source_blend: enums::Blend::SourceAlpha,
            alpha_destination_blend: enums::Blend::InverseSourceAlpha,
            alpha_blend_function: enums::BlendFunction::Add,
            //
            color_write_enable: enums::ColorWriteChannels::All,
            color_write_enable1: enums::ColorWriteChannels::All,
            color_write_enable2: enums::ColorWriteChannels::All,
            color_write_enable3: enums::ColorWriteChannels::All,
            //
            blend_factor: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            // TODO: what does it mean?? should we wrap it?
            multi_sample_mask: -1,
        }
    }
}

// ----------------------------------------
// DepthStencilState

/// Wraps `fna3d_sys::FNA3d_DepthStencilState`
#[derive(Debug, Clone)]
pub struct DepthStencilState {
    raw: sys::FNA3D_DepthStencilState,
}

/// Wrap enums and booleans
impl DepthStencilState {
    pub fn raw(&mut self) -> &mut sys::FNA3D_DepthStencilState {
        &mut self.raw
    }

    // ----------------------------------------
    // depth buffer

    pub fn is_depth_buffer_enabled(&self) -> bool {
        self.raw.depthBufferEnable == 0
    }

    pub fn set_is_depth_buffer_enabled(&mut self, b: bool) {
        self.raw.depthBufferEnable = b as u8;
    }

    pub fn is_depth_buffer_write_enabled(&self) -> bool {
        self.raw.depthBufferWriteEnable == 0
    }

    pub fn set_is_depth_buffer_write_enabled(&mut self, b: bool) {
        self.raw.depthBufferWriteEnable = b as u8;
    }

    pub fn depth_buffer_function(&self) -> enums::CompareFunction {
        enums::CompareFunction::from_u32(self.raw.depthBufferFunction).unwrap()
    }

    pub fn set_depth_buffer_function(&mut self, f: enums::CompareFunction) {
        self.raw.depthBufferFunction = f as u32;
    }

    // ----------------------------------------
    // stencil

    pub fn is_stencil_enabled(&self) -> bool {
        self.raw.stencilEnable == 0
    }

    pub fn set_is_stencil_enabled(&mut self, b: bool) {
        self.raw.stencilEnable = b as u8;
    }

    pub fn stencil_mask(&self) -> i32 {
        self.raw.stencilMask
    }

    pub fn set_stencil_mask(&mut self, mask: i32) {
        self.raw.stencilMask = mask;
    }

    pub fn stencik_write_mask(&self) -> i32 {
        self.raw.stencilWriteMask
    }

    pub fn set_stencik_write_mask(&mut self, mask: i32) {
        self.raw.stencilWriteMask = mask;
    }

    pub fn is_two_sided_stencil_mode(&self) -> bool {
        self.raw.twoSidedStencilMode == 0
    }

    pub fn set_two_sided_stencil_mode(&mut self, b: bool) {
        self.raw.twoSidedStencilMode = b as u8;
    }

    pub fn stencil_fail(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.stencilFail).unwrap()
    }

    pub fn set_stencil_fail(&mut self, stencil: enums::StencilOperation) {
        self.raw.stencilFail = stencil as u32;
    }

    pub fn stencil_depth_buffer_fail(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.stencilDepthBufferFail).unwrap()
    }

    pub fn set_stencil_depth_buffer_fail(&mut self, stencil: enums::StencilOperation) {
        self.raw.stencilDepthBufferFail = stencil as u32;
    }

    pub fn stencil_pass(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.stencilPass).unwrap()
    }

    pub fn set_stencil_pass(&mut self, stencil: enums::StencilOperation) {
        self.raw.stencilPass = stencil as u32;
    }

    //     pub stencil_function: enums::CompareFunction,
    pub fn stencil_function(&self) -> enums::CompareFunction {
        enums::CompareFunction::from_u32(self.raw.depthBufferFunction).unwrap()
    }

    // ----------------------------------------
    // ccw

    pub fn ccw_stencil_fail(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.ccwStencilFail).unwrap()
    }

    pub fn set_ccw_stencil_fail(&mut self, stencil: enums::StencilOperation) {
        self.raw.ccwStencilFail = stencil as u32;
    }

    pub fn ccw_stencil_depth_buffer_fail(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.ccwStencilDepthBufferFail).unwrap()
    }

    pub fn set_ccw_stencil_depth_buffer_fail(&mut self, stencil: enums::StencilOperation) {
        self.raw.ccwStencilDepthBufferFail = stencil as u32;
    }

    pub fn ccw_stencil_pass(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.ccwStencilPass).unwrap()
    }

    pub fn set_ccw_stencil_pass(&mut self, stencil: enums::StencilOperation) {
        self.raw.ccwStencilPass = stencil as u32;
    }

    pub fn ccw_stencil_function(&self) -> enums::StencilOperation {
        enums::StencilOperation::from_u32(self.raw.ccwStencilFunction).unwrap()
    }

    pub fn set_ccw_stencil_function(&mut self, stencil: enums::StencilOperation) {
        self.raw.ccwStencilFunction = stencil as u32;
    }

    pub fn reference_stencil(&self) -> i32 {
        self.raw.referenceStencil
    }

    pub fn set_renference_stencil(&mut self, stencil: i32) {
        self.raw.referenceStencil = stencil
    }
}

impl Default for DepthStencilState {
    fn default() -> Self {
        Self {
            raw: sys::FNA3D_DepthStencilState {
                depthBufferEnable: true as u8,
                depthBufferWriteEnable: true as u8,
                depthBufferFunction: enums::CompareFunction::Less as u32,
                stencilEnable: false as u8,
                stencilMask: 0,
                stencilWriteMask: 0,
                twoSidedStencilMode: false as u8,
                stencilFail: enums::StencilOperation::Keep as u32,
                stencilDepthBufferFail: enums::StencilOperation::Keep as u32,
                stencilPass: enums::StencilOperation::Keep as u32,
                stencilFunction: enums::CompareFunction::Always as u32,
                ccwStencilFail: enums::StencilOperation::Keep as u32,
                ccwStencilDepthBufferFail: enums::StencilOperation::Keep as u32,
                ccwStencilPass: enums::StencilOperation::Keep as u32,
                ccwStencilFunction: enums::CompareFunction::Always as u32,
                referenceStencil: 0,
            },
        }
    }
}

impl DepthStencilState {
    // TODO: what is this??
    pub fn none() -> Self {
        let mut me = Self::default();
        me.set_is_depth_buffer_enabled(false);
        me.set_is_depth_buffer_write_enabled(false);
        // TODO: is this coorect?
        me.set_depth_buffer_function(enums::CompareFunction::Always);
        me
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
