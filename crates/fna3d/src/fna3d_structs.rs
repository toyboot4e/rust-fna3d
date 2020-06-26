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
    raw: sys::FNA3D_BlendState,
}

impl BlendState {
    pub fn raw(&mut self) -> &mut sys::FNA3D_BlendState {
        &mut self.raw
    }

    // ----------------------------------------
    // Color blending

    pub fn color_src_blend(&self) -> enums::Blend {
        enums::Blend::from_u32(self.raw.colorSourceBlend).unwrap()
    }

    pub fn set_color_src_blend(&mut self, blend: enums::Blend) {
        self.raw.colorSourceBlend = blend as u32;
    }

    pub fn color_dest_blend(&self) -> enums::Blend {
        enums::Blend::from_u32(self.raw.colorDestinationBlend).unwrap()
    }

    pub fn set_color_dest_blend(&mut self, blend: enums::Blend) {
        self.raw.colorDestinationBlend = blend as u32;
    }

    pub fn color_blend_fn(&self) -> enums::BlendFunction {
        enums::BlendFunction::from_u32(self.raw.colorBlendFunction).unwrap()
    }

    pub fn set_color_blend_fn(&mut self, value: enums::BlendFunction) {
        self.raw.colorBlendFunction = value as u32;
    }

    // ----------------------------------------
    // Alpha blending

    pub fn alpha_src_blend(&self) -> enums::Blend {
        enums::Blend::from_u32(self.raw.alphaSourceBlend).unwrap()
    }

    pub fn set_alpha_src_blend(&mut self, blend: enums::Blend) {
        self.raw.alphaSourceBlend = blend as u32;
    }

    pub fn alpha_dest_blend(&self) -> enums::Blend {
        enums::Blend::from_u32(self.raw.alphaDestinationBlend).unwrap()
    }

    pub fn set_alpha_dest_blend(&mut self, blend: enums::Blend) {
        self.raw.alphaDestinationBlend = blend as u32;
    }

    pub fn alpha_blend_fn(&self) -> enums::BlendFunction {
        enums::BlendFunction::from_u32(self.raw.alphaBlendFunction).unwrap()
    }

    pub fn set_alpha_blend_fn(&mut self, blend_fn: enums::BlendFunction) {
        self.raw.alphaBlendFunction = blend_fn as u32;
    }

    // ----------------------------------------
    // Color write

    pub fn color_write_enable(&self) -> enums::ColorWriteChannels {
        enums::ColorWriteChannels::from_u32(self.raw.colorWriteEnable).unwrap()
    }

    pub fn set_color_write_enable(&mut self, channel: enums::ColorWriteChannels) {
        self.raw.colorWriteEnable = channel as u32;
    }

    pub fn color_write_enable1(&self) -> enums::ColorWriteChannels {
        enums::ColorWriteChannels::from_u32(self.raw.colorWriteEnable1).unwrap()
    }

    pub fn set_color_write_enable1(&mut self, channel: enums::ColorWriteChannels) {
        self.raw.colorWriteEnable1 = channel as u32;
    }

    pub fn color_write_enable2(&self) -> enums::ColorWriteChannels {
        enums::ColorWriteChannels::from_u32(self.raw.colorWriteEnable2).unwrap()
    }

    pub fn set_color_write_enable2(&mut self, channel: enums::ColorWriteChannels) {
        self.raw.colorWriteEnable2 = channel as u32;
    }

    pub fn color_write_enable3(&self) -> enums::ColorWriteChannels {
        enums::ColorWriteChannels::from_u32(self.raw.colorWriteEnable3).unwrap()
    }

    pub fn set_color_write_enable3(&mut self, channel: enums::ColorWriteChannels) {
        self.raw.colorWriteEnable3 = channel as u32;
    }
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
            raw: sys::FNA3D_BlendState {
                colorSourceBlend: enums::Blend::SourceAlpha as u32,
                colorDestinationBlend: enums::Blend::InverseSourceAlpha as u32,
                colorBlendFunction: enums::BlendFunction::Add as u32,
                //
                alphaSourceBlend: enums::Blend::SourceAlpha as u32,
                alphaDestinationBlend: enums::Blend::InverseSourceAlpha as u32,
                alphaBlendFunction: enums::BlendFunction::Add as u32,
                //
                colorWriteEnable: enums::ColorWriteChannels::All as u32,
                colorWriteEnable1: enums::ColorWriteChannels::All as u32,
                colorWriteEnable2: enums::ColorWriteChannels::All as u32,
                colorWriteEnable3: enums::ColorWriteChannels::All as u32,
                //
                blendFactor: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                // TODO: what does it mean??
                multiSampleMask: -1,
            },
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
