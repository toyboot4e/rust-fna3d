//! Wrappers of enum variants defined as constants by `bindgen`
//!
//! Because C enums are loosely typed, `bindgen` defines each variant of an enum as a constant.
//! Here we wrap them into `enum` s using the `enum_primitive` crate to implement `from_xxx`
//! methods.
//!
//! # References
//!
//! * https://github.com/rust-lang/rust-bindgen/issues/1096
//!
//! # Example
//!
//! ```
//! // Constants generated with `bindgen`:
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET: FNA3D_ClearOptions = 1;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER: FNA3D_ClearOptions = 2;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL: FNA3D_ClearOptions = 4;
//! pub type FNA3D_ClearOptions = u32;
//!
//! // We wrap them into an enum:
//! use enum_primitive::*;
//! enum_from_primitive! {
//!     #[derive(Debug, Copy, Clone, PartialEq)]
//!     #[repr(u32)]
//!     pub enum ClearOptions {
//!         Target = FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET,
//!         DepthBuffer = FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER,
//!         Stencil = FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL,
//!     }
//! }
//! ```

// TODO: make a macro to wrap u32 as enum

// TODO: cast to underlying type
// pub trait ToRepr {
//     type Output;
//     fn to_repr(&self) -> Self::Output;
// }

// TODO: should we use u8 or stick with u32?
// TODO: do we not need to use enum_primitive?
// TODO: was it possible to make such enums automatically?
// TODO: MOJOSHADER_effect?

use enum_primitive::*;
use fna3d_sys as sys;

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum PresentInterval {
        Defalt = sys::FNA3D_PresentInterval_FNA3D_PRESENTINTERVAL_DEFAULT,
        One = sys::FNA3D_PresentInterval_FNA3D_PRESENTINTERVAL_ONE,
        Two = sys::FNA3D_PresentInterval_FNA3D_PRESENTINTERVAL_TWO,
        Immediate = sys::FNA3D_PresentInterval_FNA3D_PRESENTINTERVAL_IMMEDIATE,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum DisplayOrientation {
        Defaut = sys::FNA3D_DisplayOrientation_FNA3D_DISPLAYORIENTATION_DEFAULT,
        LandscapeLeft = sys::FNA3D_DisplayOrientation_FNA3D_DISPLAYORIENTATION_LANDSCAPELEFT,
        LandscapeRight = sys::FNA3D_DisplayOrientation_FNA3D_DISPLAYORIENTATION_LANDSCAPERIGHT,
        Portrait = sys::FNA3D_DisplayOrientation_FNA3D_DISPLAYORIENTATION_PORTRAIT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum RenderTargetUsage {
        DiscardContents = sys::FNA3D_RenderTargetUsage_FNA3D_RENDERTARGETUSAGE_DISCARDCONTENTS,
        PreserveContents = sys::FNA3D_RenderTargetUsage_FNA3D_RENDERTARGETUSAGE_PRESERVECONTENTS,
        PlatformContents = sys::FNA3D_RenderTargetUsage_FNA3D_RENDERTARGETUSAGE_PLATFORMCONTENTS,
    }
}

bitflags::bitflags! {
    /// Specifies the buffers for clearing when calling `Device::clear`
    pub struct ClearOptions: u32 {
        /// Color buffer
        const Target = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET;
        /// Depth buffer
        const DepthBuffer = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER;
        /// Stencil buffer
        const Stencil = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL;
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// How vertex data is ordered
    pub enum PrimitiveType {
        /// Renders the specified vertices as a sequence of isolated triangles. Each group of three
        /// vertices defines a separate triangle. Back-face culling is affected by the current
        /// winding-order render state.
        TriangleList = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_TRIANGLELIST,
        /// Renders the vertices as a triangle strip. The back-face culling flag is flipped
        /// automatically on even-numbered triangles.
        TriangleStrip = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_TRIANGLESTRIP,
        /// Renders the vertices as a list of isolated straight line segments; the count may be any
        /// positive integer.
        LineList = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_LINELIST,
        /// Renders the vertices as a single polyline; the count may be any positive integer.
        LineStrip = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_LINESTRIP,
        /// Treats each vertex as a single point. Vertex n defines point n. N points are drawn.
        PointListExt = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_POINTLIST_EXT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// Size for index in `IndexBuffer` or `DynamicIndexBuffer`
    pub enum IndexElementSize {
        /// `i16` will be used as index type
        Bits16 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT,
        /// `i32` will be used as index type
        Bits32 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SurfaceFormat {
        Color = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_COLOR,
        Bgr565 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGR565,
        Bgra5551 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGRA5551,
        Bgra4444 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGRA4444,
        Dxt1 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT1,
        Dxt3 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT3,
        Dxt5 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT5,
        NormalizedByte2 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_NORMALIZEDBYTE2,
        NormalizedByte4 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_NORMALIZEDBYTE4,
        Rgba1010102 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_RGBA1010102,
        Rg32 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_RG32,
        Rgba64 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_RGBA64,
        Alpha8 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_ALPHA8,
        Single = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_SINGLE,
        Vector2 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_VECTOR2,
        Vector4 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_VECTOR4,
        HalfSingle = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_HALFSINGLE,
        HalfVector2 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_HALFVECTOR2,
        HalfVector4 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_HALFVECTOR4,
        HdrBlendable = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_HDRBLENDABLE,
        ColorBgraExt = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_COLORBGRA_EXT,
    }
}

impl SurfaceFormat {
    pub fn size(&self) -> usize {
        match self {
            SurfaceFormat::Dxt1 => 8,
            SurfaceFormat::Dxt3 | SurfaceFormat::Dxt5 => 16,
            SurfaceFormat::Alpha8 => 1,
            SurfaceFormat::Bgr565
            | SurfaceFormat::Bgra4444
            | SurfaceFormat::Bgra5551
            | SurfaceFormat::HalfSingle
            | SurfaceFormat::NormalizedByte2 => 2,
            SurfaceFormat::Color
            | SurfaceFormat::Single
            | SurfaceFormat::Rg32
            | SurfaceFormat::HalfVector2
            | SurfaceFormat::NormalizedByte4
            | SurfaceFormat::Rgba1010102
            | SurfaceFormat::ColorBgraExt => 4,
            SurfaceFormat::HalfVector4 | SurfaceFormat::Rgba64 | SurfaceFormat::Vector2 => 8,
            SurfaceFormat::Vector4 => 16,
            SurfaceFormat::HdrBlendable => panic!("SurfaceFormat::HdrBlendable is only used for RenderTarget and should not get size (?)"),
        }
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum DepthFormat {
        None = sys::FNA3D_DepthFormat_FNA3D_DEPTHFORMAT_NONE,
        D16 = sys::FNA3D_DepthFormat_FNA3D_DEPTHFORMAT_D16,
        D24 = sys::FNA3D_DepthFormat_FNA3D_DEPTHFORMAT_D24,
        D24S8 = sys::FNA3D_DepthFormat_FNA3D_DEPTHFORMAT_D24S8,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum CubeMapFace {
        PositiveX = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_POSITIVEX,
        NegativeX = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_NEGATIVEX,
        PositiveY = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_POSITIVEY,
        NegativeY = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_NEGATIVEY,
        PositiveZ = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_POSITIVEZ,
        NegativeZ = sys::FNA3D_CubeMapFace_FNA3D_CUBEMAPFACE_NEGATIVEZ,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// Hint for optimizing memory placement of graphics buffers
    pub enum BufferUsage {
        None = sys::FNA3D_BufferUsage_FNA3D_BUFFERUSAGE_NONE,
        WriteOnly = sys::FNA3D_BufferUsage_FNA3D_BUFFERUSAGE_WRITEONLY,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// How vertex or index buffer data will be flushed during a SetData operation.
    pub enum SetDataOptions {
        /// The SetData operation can overwrite the portions of existing data.
        None = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_NONE,
        /// The SetData operation will discard the entire buffer. A pointer to a new memory area is
        /// returned and rendering from the previous area do notstall
        Discard = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_DISCARD,
        /// The SetData operation will not overwrite existing data. This allows the driver to
        /// return immediately from a SetData operation and continue rendering.
        NoOverwrite = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_NOOVERWRITE,
    }
}

enum_from_primitive! {
    /// Blend mode. Component of `BlendState`
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum Blend {
        /// Each component of the color is multiplied by {1, 1, 1, 1}.
        One = sys::FNA3D_Blend_FNA3D_BLEND_ONE,
        /// Each component of the color is multiplied by {0, 0, 0, 0}.
        Zero = sys::FNA3D_Blend_FNA3D_BLEND_ZERO,
        /// Each component of the color is multiplied by the source color.
        /// {Rs, Gs, Bs, As}, where Rs, Gs, Bs, As are color source values.
        SourceColor = sys::FNA3D_Blend_FNA3D_BLEND_SOURCECOLOR,
        /// Each component of the color is multiplied by the inverse of the source color.
        /// {1 - Rs, 1 - Gs, 1 - Bs, 1 - As}, where Rs, Gs, Bs, As are color source values.
        InverseSourceColor = sys::FNA3D_Blend_FNA3D_BLEND_INVERSESOURCECOLOR,
        /// Each component of the color is multiplied by the alpha value of the source.
        /// {As, As, As, As}, where As is the source alpha value.
        SourceAlpha = sys::FNA3D_Blend_FNA3D_BLEND_SOURCEALPHA,
        /// Each component of the color is multiplied by the inverse of the alpha value of the source.
        /// {1 - As, 1 - As, 1 - As, 1 - As}, where As is the source alpha value.
        InverseSourceAlpha = sys::FNA3D_Blend_FNA3D_BLEND_INVERSESOURCEALPHA,
        /// Each component color is multiplied by the destination color.
        /// {Rd, Gd, Bd, Ad}, where Rd, Gd, Bd, Ad are color destination values.
        DestinationColor = sys::FNA3D_Blend_FNA3D_BLEND_DESTINATIONCOLOR,
        /// Each component of the color is multiplied by the inversed destination color.
        /// {1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad}, where Rd, Gd, Bd, Ad are color destination values.
        InveseDestinationColor = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEDESTINATIONCOLOR,
        /// Each component of the color is multiplied by the alpha value of the destination.
        /// {Ad, Ad, Ad, Ad}, where Ad is the destination alpha value.
        DestinaitonAlpha = sys::FNA3D_Blend_FNA3D_BLEND_DESTINATIONALPHA,
        /// Each component of the color is multiplied by the inversed alpha value of the destination.
        /// {1 - Ad, 1 - Ad, 1 - Ad, 1 - Ad}, where Ad is the destination alpha value.
        InverseDetinationAlpha = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEDESTINATIONALPHA,
        /// Each component of the color is multiplied by a constant in the <see cref="GraphicsDevice.BlendFactor"/>.
        BlendFactor = sys::FNA3D_Blend_FNA3D_BLEND_BLENDFACTOR,
        /// Each component of the color is multiplied by a inversed constant in the <see cref="GraphicsDevice.BlendFactor"/>.
        InverseBlendFactor = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEBLENDFACTOR,
        /// Each component of the color is multiplied by either the alpha of the source color, or the inverse of the alpha of the source color, whichever is greater.
        /// {f, f, f, 1}, where f = min(As, 1 - As), where As is the source alpha value.
        SourceAlphaSaturation = sys::FNA3D_Blend_FNA3D_BLEND_SOURCEALPHASATURATION,
    }
}

enum_from_primitive! {
    /// Function for color blending. Component of `BlendState`
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum BlendFunction {
        /// `(src_color * src_blend) + (dest_color * dest_blend)`
        Add = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_ADD,
        /// `(src_color * src_blend) - (dest_color * dest_blend)`
        Substract = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_SUBTRACT,
        /// `(dest_color * dest_blend) - (src_color * src_blend)`
        ReverseSubstract = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_REVERSESUBTRACT,
        /// `min((src_color * src_blend),(dest_color * dest_blend))`
        Max = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_MAX,
        /// `max((src_color * src_blend),(dest_color * dest_blend))`
        Min = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_MIN,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// Specifies color channels for render target blending operations
    pub enum ColorWriteChannels {
        None = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_NONE,
        Red = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_RED,
        Green = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_GREEN,
        Blue = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_BLUE,
        Alpha = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_ALPHA,
        All = sys::FNA3D_ColorWriteChannels_FNA3D_COLORWRITECHANNELS_ALL,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum StencilOperation {
        Keep = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_KEEP,
        Zero = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_ZERO,
        Replace = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_REPLACE,
        Increment = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_INCREMENT,
        Decrement = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_DECREMENT,
        IncrementSaturation = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_INCREMENTSATURATION,
        DecrementSaturation = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_DECREMENTSATURATION,
        Invert = sys::FNA3D_StencilOperation_FNA3D_STENCILOPERATION_INVERT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum CompareFunction {
        Always = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_ALWAYS,
        Never = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_NEVER,
        Less = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_LESS,
        LessEqual = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_LESSEQUAL,
        Equal = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_EQUAL,
        GreaterEqual = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_GREATEREQUAL,
        Greater = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_GREATER,
        NonEqual = sys::FNA3D_CompareFunction_FNA3D_COMPAREFUNCTION_NOTEQUAL,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum CullMode {
        None = sys::FNA3D_CullMode_FNA3D_CULLMODE_NONE,
        CullClockWiseFace = sys::FNA3D_CullMode_FNA3D_CULLMODE_CULLCLOCKWISEFACE,
        CullCounterClockwiseFace = sys::FNA3D_CullMode_FNA3D_CULLMODE_CULLCOUNTERCLOCKWISEFACE,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum FillMode {
        Solid = sys::FNA3D_FillMode_FNA3D_FILLMODE_SOLID,
        WireFrame = sys::FNA3D_FillMode_FNA3D_FILLMODE_WIREFRAME,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// Modes for addressing texels (texture pixels) using texture coordinates that are outside of
    /// the range of `0.0` to `1.0`.
    pub enum TextureAddressMode {
        /// Texels outside range will form the tile at every integer junction.
        Wrap = sys::FNA3D_TextureAddressMode_FNA3D_TEXTUREADDRESSMODE_WRAP,
        /// Texels outside range will be setted to color of `0.0` or `1.0` texel.
        Clamp = sys::FNA3D_TextureAddressMode_FNA3D_TEXTUREADDRESSMODE_CLAMP,
        /// Same as `TextureAddressMode::Wrap` but tiles will also flipped at every integer
        /// junction.
        Mirror = sys::FNA3D_TextureAddressMode_FNA3D_TEXTUREADDRESSMODE_MIRROR,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum TextureFilter {
        Linear = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_LINEAR,
        Point = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_POINT,
        Anisotropic = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_ANISOTROPIC,
        LinearMipPoint = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_LINEAR_MIPPOINT,
        PointMipLinear = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_POINT_MIPLINEAR,
        MinLinearMagPointMipLinear = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_MINLINEAR_MAGPOINT_MIPLINEAR,
        MinLinearMagPointMipPoint = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_MINLINEAR_MAGPOINT_MIPPOINT,
        MinPointMagLinearMipLinear = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_MINPOINT_MAGLINEAR_MIPLINEAR,
        MinPointMagLinearMipPoint = sys::FNA3D_TextureFilter_FNA3D_TEXTUREFILTER_MINPOINT_MAGLINEAR_MIPPOINT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum VertexElementFormat {
        Single = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_SINGLE,
        Vector2 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_VECTOR2,
        Vector3 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_VECTOR3,
        Vector4 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_VECTOR4,
        Color = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_COLOR,
        Byte4 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_BYTE4,
        Short2 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_SHORT2,
        Short4 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_SHORT4,
        NormalizedShort2 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_NORMALIZEDSHORT2,
        NormalizedShort4 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_NORMALIZEDSHORT4,
        HalfVector2 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_HALFVECTOR2,
        HalfVector4 = sys::FNA3D_VertexElementFormat_FNA3D_VERTEXELEMENTFORMAT_HALFVECTOR4,
    }
}

impl VertexElementFormat {
    pub fn size(&self) -> u8 {
        match self {
            VertexElementFormat::Single => 4,
            VertexElementFormat::Vector2 => 8,
            VertexElementFormat::Vector3 => 12,
            VertexElementFormat::Vector4 => 16,
            VertexElementFormat::Color => 4,
            VertexElementFormat::Byte4 => 4,
            VertexElementFormat::Short2 => 4,
            VertexElementFormat::Short4 => 8,
            VertexElementFormat::NormalizedShort2 => 4,
            VertexElementFormat::NormalizedShort4 => 8,
            VertexElementFormat::HalfVector2 => 4,
            VertexElementFormat::HalfVector4 => 8,
        }
    }
}

// line 854
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum VertexElementUsage {
        Position = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_POSITION,
        Color = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_COLOR,
        TextureCoordinate = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_TEXTURECOORDINATE,
        Nornal = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_NORMAL,
        BinNormal = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_BINORMAL,
        Tangent = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_TANGENT,
        BlendIndices = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_BLENDINDICES,
        BendWeight = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_BLENDWEIGHT,
        Depth = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_DEPTH,
        Fog = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_FOG,
        PointSize = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_POINTSIZE,
        Sample = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_SAMPLE,
        TesselateFactor = sys::FNA3D_VertexElementUsage_FNA3D_VERTEXELEMENTUSAGE_TESSELATEFACTOR,
    }
}
