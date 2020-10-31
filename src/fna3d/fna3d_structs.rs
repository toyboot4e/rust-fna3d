//! Structure types in FNA3D other than `Device`
//!
//! We _could_ use macors to define field accessors. Probablly is usefule for that. However, I
//! prefered explicit definitions this time.
//!
//! * TODO: wrap "masks" with newtype struct?
//! * TODO: wrap more structs
//!
//! [paste]: https://github.com/dtolnay/paste

use ::{fna3d_sys as sys, num_traits::FromPrimitive};

use crate::fna3d::fna3d_enums as enums;

// for documentation (types in scope are automatically linked with [`TypeName`])
#[allow(unused_imports)]
use crate::fna3d::fna3d_device::Device;
#[allow(unused_imports)]
use crate::fna3d::fna3d_enums::*;

// --------------------------------------------------------------------------------
// Disposed types
//
// Those types have corresponding disposing functions in `Device`

/// Opaque struct that represents index or vertex buffer
///
/// It is not strictly typed and more information have to be supplied with [`BufferUsage`],
/// [`VertexDeclaration`] and [`IndexElementSize`].
///
/// # Dispose
///
/// This type has to be disposed with a corresponding function in [`Device`]
pub type Buffer = sys::FNA3D_Buffer;

/// Opaque struct that represents FNA3D render buffer
///
/// Disposed with a corresponding function in [`Device`]
pub type Renderbuffer = sys::FNA3D_Renderbuffer;

/// Opaque struct that represents FNA3D effect
///
/// Disposed with a corresponding function in [`Device`]
pub type Effect = sys::FNA3D_Effect;

/// Opaque struct that represents FNA3D query
///
/// Disposed with a corresponding function in [`Device`]
pub type Query = sys::FNA3D_Query;

/// Opaque struct that represents 2D or 3D texture data stored in GPU memory
///
/// # Dispose
///
/// This type has to be disposed with a corresponding function in [`Device`]
pub type Texture = sys::FNA3D_Texture;

/// [`Device::apply_vertex_buffer_bindings`] parameter, which describes vertex memory location and
/// attributes
pub type VertexBufferBinding = sys::FNA3D_VertexBufferBinding;

pub struct RenderTargetBinding {
    raw: sys::FNA3D_RenderTargetBinding,
}

/// 2D | Cube
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderTargetType {
    /// w, h
    TwoD = 0,
    /// size, face
    Cube = 1,
}

impl RenderTargetBinding {
    pub fn raw(&self) -> &sys::FNA3D_RenderTargetBinding {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_RenderTargetBinding {
        &mut self.raw
    }

    // two constructors handling the union

    pub fn new_2d(
        type_: RenderTargetType,
        level_count: u32,
        multi_sample_count: u32,
        texture: *mut Texture,
        w: u32,
        h: u32,
        color_buffer: *mut Renderbuffer,
    ) -> Self {
        Self {
            raw: sys::FNA3D_RenderTargetBinding {
                type_: type_ as u8,
                __bindgen_anon_1: sys::FNA3D_RenderTargetBinding__bindgen_ty_1 {
                    twod: sys::FNA3D_RenderTargetBinding__bindgen_ty_1__bindgen_ty_1 {
                        width: w as i32,
                        height: h as i32,
                    },
                },
                levelCount: level_count as i32,
                multiSampleCount: multi_sample_count as i32,
                texture,
                colorBuffer: color_buffer,
            },
        }
    }

    pub fn new_cube(
        type_: enums::RenderTargetUsage,
        level_count: u32,
        multi_sample_count: u32,
        texture: *mut Texture,
        size: u32,
        face: enums::CubeMapFace,
        color_buffer: *mut Renderbuffer,
    ) -> Self {
        Self {
            raw: sys::FNA3D_RenderTargetBinding {
                type_: type_ as u8,
                __bindgen_anon_1: sys::FNA3D_RenderTargetBinding__bindgen_ty_1 {
                    cube: sys::FNA3D_RenderTargetBinding__bindgen_ty_1__bindgen_ty_2 {
                        size: size as i32,
                        face: face as u32,
                    },
                },
                levelCount: level_count as i32,
                multiSampleCount: multi_sample_count as i32,
                texture,
                colorBuffer: color_buffer,
            },
        }
    }
}

// /// 2D | Cube with access to internals
// #[derive(Debug)]
// pub enum RenderTargetBindingTypeDataAcecss<'a> {
//     /// w, h
//     TwoD(&'a mut sys::FNA3D_RenderTargetBinding__bindgen_ty_1__bindgen_ty_1),
//     /// size, face
//     Cube(&'a mut sys::FNA3D_RenderTargetBinding__bindgen_ty_1__bindgen_ty_2),
// }

// /// Accessors
// impl RenderTargetBinding {
//     pub fn type_data_mut(&mut self) -> RenderTargetBindingTypeDataAcecss<'_> {
//         unsafe {
//             match self.raw.type_ {
//                 0 => RenderTargetBindingTypeDataAcecss::TwoD(&mut self.raw.__bindgen_anon_1.twod),
//                 1 => RenderTargetBindingTypeDataAcecss::Cube(&mut self.raw.__bindgen_anon_1.cube),
//                 _ => unreachable!(),
//             }
//         }
//     }
// }

// --------------------------------------------------------------------------------
// Type aliases

// TODO: maybe wrap those types

/// The view bounds for render-target surface
pub type Viewport = sys::FNA3D_Viewport;

/// 24 bits RGBA color
///
/// [`Color::to_vec4`] is available.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    raw: sys::FNA3D_Color,
    // another approach would be implementing `std::ops::Deref` but I think this is OK
}

impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
        self.raw.r == other.raw.r
            && self.raw.g == other.raw.g
            && self.raw.b == other.raw.b
            && self.raw.a == other.raw.a
    }
}

impl Eq for Color {}

impl Color {
    pub fn raw(&self) -> sys::FNA3D_Color {
        self.raw
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.raw.r as f32 / 255.0,
            y: self.raw.g as f32 / 255.0,
            z: self.raw.b as f32 / 255.0,
            w: self.raw.a as f32 / 255.0,
        }
    }

    // TODO pre-multiplied alpha or not
    pub fn multiply(&self, f: f32) -> Self {
        Self {
            raw: sys::FNA3D_Color {
                r: (self.raw.r as f32 * f) as u8,
                g: (self.raw.g as f32 * f) as u8,
                b: (self.raw.b as f32 * f) as u8,
                a: (self.raw.a as f32 * f) as u8,
            },
        }
    }
}

/// Constructors
impl Color {
    /// Normalized [`Vec4`] -> [`Color`]
    pub fn from_vec4(v: Vec4) -> Self {
        fn clamp(v: f32, min: f32, max: f32) -> f32 {
            if v <= min {
                min
            } else if v >= max {
                max
            } else {
                v
            }
        }

        Self {
            raw: sys::FNA3D_Color {
                r: clamp(v.x * 255.0, 0.0, 255.0) as u8,
                g: clamp(v.y * 255.0, 0.0, 255.0) as u8,
                b: clamp(v.z * 255.0, 0.0, 255.0) as u8,
                a: clamp(v.w * 255.0, 0.0, 255.0) as u8,
            },
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            raw: sys::FNA3D_Color {
                r: r,
                g: g,
                b: b,
                a: a,
            },
        }
    }

    /// Returns premultiplied alpha
    pub fn from_non_premultiplied(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::rgba((r * a) / 255, (g * a) / 255, (b * a) / 255, a)
    }
}

/// Predefined colors
impl Color {
    pub fn transparent() -> Self {
        Self::rgba(0, 0, 0, 0)
    }

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub fn cornflower_blue() -> Self {
        Self::rgb(100, 149, 237)
    }
}

/// Used to represent scissors rectangle
pub type Rect = sys::FNA3D_Rect;
/// Used to represent color
pub type Vec4 = sys::FNA3D_Vec4;
pub type PresentationParameters = sys::FNA3D_PresentationParameters;

// MOJOSHADER_effect?

// --------------------------------------------------------------------------------
// Vertex

// ----------------------------------------
// VertexDeclaration

/// [`VertexBufferBinding`] component that declares memory layout of a vertex data
///
/// Users can use custom vertex data with declaration.
///
/// Composed of [`VertexElement`]s
pub type VertexDeclaration = sys::FNA3D_VertexDeclaration;

/// [`VertexDeclaration`] component that specifies an element of vertex data
///
/// Needs to be related with [`VertexElementFormat`] and [`VertexElementUsage`]
///
/// [`VertexElementFormat`]: crate::VertexElement
/// [`VertexElementUsage`]: crate::VertexElementUsage
pub type VertexElement = sys::FNA3D_VertexElement;

// --------------------------------------------------------------------------------
// States

// ----------------------------------------
// RasterizerState

/// Pipeline
#[derive(Debug, Clone)]
pub struct RasterizerState {
    raw: sys::FNA3D_RasterizerState,
}

impl Default for RasterizerState {
    fn default() -> Self {
        Self {
            raw: sys::FNA3D_RasterizerState {
                fillMode: enums::FillMode::Solid as u32,
                // FIXME: should I use None?
                cullMode: enums::CullMode::CullCounterClockwiseFace as u32,
                // cullMode: enums::CullMode::None as u32,
                depthBias: 0.0,
                slopeScaleDepthBias: 0.0,
                scissorTestEnable: false as u8,
                multiSampleAntiAlias: true as u8,
            },
        }
    }
}

impl RasterizerState {
    pub fn raw(&self) -> &sys::FNA3D_RasterizerState {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_RasterizerState {
        &mut self.raw
    }

    pub fn from_cull_mode(mode: enums::CullMode) -> Self {
        let mut me = Self::default();
        me.set_cull_mode(mode);
        me
    }
}

/// Accessors
impl RasterizerState {
    pub fn fill_mode(&self) -> enums::FillMode {
        enums::FillMode::from_u32(self.raw.fillMode).unwrap()
    }

    pub fn set_fill_mode(&mut self, fill_mode: enums::FillMode) {
        self.raw.fillMode = fill_mode as u32;
    }

    pub fn cull_mode(&self) -> enums::CullMode {
        enums::CullMode::from_u32(self.raw.cullMode).unwrap()
    }

    pub fn set_cull_mode(&mut self, value: enums::CullMode) {
        self.raw.cullMode = value as u32;
    }

    pub fn depth_bias(&self) -> f32 {
        self.raw.depthBias
    }

    pub fn set_depth_bias(&mut self, value: f32) {
        self.raw.depthBias = value;
    }

    pub fn slope_scale_depth_bias(&self) -> f32 {
        self.raw.slopeScaleDepthBias
    }

    pub fn set_slope_scale_depth_bias(&mut self, value: f32) {
        self.raw.slopeScaleDepthBias = value;
    }

    pub fn scissor_test_enable(&self) -> u8 {
        self.raw.scissorTestEnable
    }

    pub fn set_scissor_test_enable(&mut self, value: u8) {
        self.raw.scissorTestEnable = value;
    }

    pub fn multi_sample_anti_alias(&self) -> u8 {
        self.raw.multiSampleAntiAlias
    }

    pub fn set_multi_sample_anti_alias(&mut self, value: u8) {
        self.raw.multiSampleAntiAlias = value;
    }
}

// ----------------------------------------
// SamplerState

/// Specifies texture sampling method
///
/// Wrap, mirror, etc.
#[derive(Debug, Clone)]
pub struct SamplerState {
    raw: sys::FNA3D_SamplerState,
}

impl Default for SamplerState {
    fn default() -> Self {
        Self {
            raw: sys::FNA3D_SamplerState {
                filter: enums::TextureFilter::Linear as u32,
                // texture coordinates u, v, and w
                addressU: enums::TextureAddressMode::Wrap as u32,
                addressV: enums::TextureAddressMode::Wrap as u32,
                addressW: enums::TextureAddressMode::Wrap as u32,
                mipMapLevelOfDetailBias: 0.0,
                maxAnisotropy: 4,
                maxMipLevel: 0,
            },
        }
    }
}

impl SamplerState {
    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_SamplerState {
        &mut self.raw
    }

    pub fn filter(&self) -> enums::TextureFilter {
        enums::TextureFilter::from_u32(self.raw.filter).unwrap()
    }

    pub fn set_filter(&mut self, filter: enums::TextureFilter) {
        self.raw.filter = filter as u32;
    }

    pub fn address_u(&self) -> enums::TextureAddressMode {
        enums::TextureAddressMode::from_u32(self.raw.addressU).unwrap()
    }

    pub fn set_address_u(&mut self, address: enums::TextureAddressMode) {
        self.raw.addressU = address as u32;
    }

    pub fn address_v(&self) -> enums::TextureAddressMode {
        enums::TextureAddressMode::from_u32(self.raw.addressV).unwrap()
    }

    pub fn set_address_v(&mut self, address: enums::TextureAddressMode) {
        self.raw.addressV = address as u32;
    }

    pub fn address_w(&self) -> enums::TextureAddressMode {
        enums::TextureAddressMode::from_u32(self.raw.addressW).unwrap()
    }

    pub fn set_address_w(&mut self, address: enums::TextureAddressMode) {
        self.raw.addressW = address as u32;
    }

    pub fn mip_map_level_of_detail_bias(&self) -> f32 {
        self.raw.mipMapLevelOfDetailBias
    }

    pub fn set_mip_map_level_of_detail_bias(&mut self, value: f32) {
        self.raw.mipMapLevelOfDetailBias = value;
    }

    pub fn max_anisotropy(&self) -> i32 {
        self.raw.maxAnisotropy
    }

    pub fn set_max_anisotropy(&mut self, value: i32) {
        self.raw.maxAnisotropy = value;
    }

    pub fn max_mip_level(&self) -> i32 {
        self.raw.maxMipLevel
    }

    pub fn set_max_mip_level(&mut self, value: i32) {
        self.raw.maxMipLevel = value;
    }
}

/// Preset values
impl SamplerState {
    fn new_(
        filter: enums::TextureFilter,
        address_u: enums::TextureAddressMode,
        address_v: enums::TextureAddressMode,
        address_w: enums::TextureAddressMode,
    ) -> Self {
        let mut me = Self::default();
        me.set_filter(filter);
        me.set_address_u(address_u);
        me.set_address_v(address_v);
        me.set_address_w(address_w);
        me
    }

    pub fn anisotropic_clamp() -> Self {
        Self::new_(
            enums::TextureFilter::Anisotropic,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
        )
    }

    pub fn anisotropic_wrap() -> Self {
        Self::new_(
            enums::TextureFilter::Anisotropic,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
        )
    }

    pub fn linear_clamp() -> Self {
        Self::new_(
            enums::TextureFilter::Linear,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
        )
    }

    pub fn linear_wrap() -> Self {
        Self::new_(
            enums::TextureFilter::Linear,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
        )
    }

    pub fn point_clamp() -> Self {
        Self::new_(
            enums::TextureFilter::Point,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
            enums::TextureAddressMode::Clamp,
        )
    }

    pub fn point_wrap() -> Self {
        Self::new_(
            enums::TextureFilter::Point,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
            enums::TextureAddressMode::Wrap,
        )
    }
}

// ----------------------------------------
// BlendState

#[derive(Debug, Clone)]
pub struct BlendState {
    raw: sys::FNA3D_BlendState,
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
            raw: sys::FNA3D_BlendState {
                // FIXME: should I use Blend::One?
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
                blendFactor: Color::rgba(0xff, 0xff, 0xff, 0xff).raw(),
                // TODO: what does it mean??
                multiSampleMask: -1,
            },
        }
    }
}

impl BlendState {
    pub fn raw(&self) -> &sys::FNA3D_BlendState {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_BlendState {
        &mut self.raw
    }
}

/// Constructors (taken from FNA)
impl BlendState {
    pub fn with_blend(
        color_src: enums::Blend,
        alpha_src: enums::Blend,
        color_dest: enums::Blend,
        alpha_dest: enums::Blend,
    ) -> Self {
        let mut me = Self::default();
        me.raw.colorSourceBlend = color_src as u32;
        me.raw.alphaSourceBlend = alpha_src as u32;
        me.raw.colorDestinationBlend = color_dest as u32;
        me.raw.alphaDestinationBlend = alpha_dest as u32;
        me
    }

    pub fn additive() -> Self {
        Self::with_blend(
            enums::Blend::SourceAlpha,
            enums::Blend::SourceAlpha,
            enums::Blend::One,
            enums::Blend::One,
        )
    }

    pub fn alpha_blend() -> Self {
        Self::with_blend(
            enums::Blend::One,
            enums::Blend::One,
            enums::Blend::InverseSourceAlpha,
            enums::Blend::InverseSourceAlpha,
        )
    }

    /// ImGUI font texture uses this blending function
    pub fn non_premultiplied() -> Self {
        Self::with_blend(
            enums::Blend::SourceAlpha,
            enums::Blend::SourceAlpha,
            enums::Blend::InverseSourceAlpha,
            enums::Blend::InverseSourceAlpha,
        )
    }

    pub fn opaque() -> Self {
        Self::with_blend(
            enums::Blend::Zero,
            enums::Blend::Zero,
            enums::Blend::One,
            enums::Blend::One,
        )
    }
}

/// Accessors
impl BlendState {
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

// ----------------------------------------
// DepthStencilState

/// Pipeline
#[derive(Debug, Clone)]
pub struct DepthStencilState {
    raw: sys::FNA3D_DepthStencilState,
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

/// Wrap enums and booleans
impl DepthStencilState {
    pub fn raw(&self) -> &sys::FNA3D_DepthStencilState {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_DepthStencilState {
        &mut self.raw
    }

    pub fn none() -> Self {
        let mut me = Self::default();
        me.raw.depthBufferEnable = false as u8;
        me.raw.depthBufferWriteEnable = false as u8;
        me
    }

    // ----------------------------------------
    // depth buffer

    pub fn is_depth_buffer_enabled(&self) -> bool {
        self.raw.depthBufferEnable != 0
    }

    pub fn set_is_depth_buffer_enabled(&mut self, b: bool) {
        self.raw.depthBufferEnable = b as u8;
    }

    pub fn is_depth_buffer_write_enabled(&self) -> bool {
        self.raw.depthBufferWriteEnable != 0
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
        self.raw.stencilEnable != 0
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
        self.raw.twoSidedStencilMode != 0
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
