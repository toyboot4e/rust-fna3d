//! Wrappers of enum variants defined as constants by `bindgen`
//!
//! Because C enums are loosely typed, `bindgen` defines each variant of an enum as a constant.
//! Here we wrap them into an `enum` using the `enum_primitive` crate. It provides a macro to
//! convert types between each `enum` and primitive.
//!
//! # References
//!
//! * https://github.com/rust-lang/rust-bindgen/issues/1096

// TODO: cast to underlying type
// pub trait ToRepr {
//     type Output;
//     fn to_repr(&self) -> Self::Output;
// }

// TODO: should we use u8 or stick with u32?
// TODO: was it possible to make such enums automatically?
// TODO: MONOSHADER_effect?

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

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum ClearOptions {
        Target = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET,
        DepthBuffer = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER,
        Stencil = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum PrimitiveType {
        TriangleList = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_TRIANGLELIST,
        TriangleStrip = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_TRIANGLESTRIP,
        LineList = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_LINELIST,
        LineStrip = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_LINESTRIP,
        PointListExt = sys::FNA3D_PrimitiveType_FNA3D_PRIMITIVETYPE_POINTLIST_EXT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum IndexElementSize {
        Bit16 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT,
        Bit32 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SurfaceFormat {
        Color = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_COLOR,
        Rgb565 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGR565,
        Rgba5551 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGRA5551,
        Rgba4444 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_BGRA4444,
        DXT1 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT1,
        DXT3 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT3,
        DXT5 = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_DXT5,
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
        ColorRgbaEx = sys::FNA3D_SurfaceFormat_FNA3D_SURFACEFORMAT_COLORBGRA_EXT,
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
    pub enum BufferUsage {
        None = sys::FNA3D_BufferUsage_FNA3D_BUFFERUSAGE_NONE,
        WhiteOnly = sys::FNA3D_BufferUsage_FNA3D_BUFFERUSAGE_WRITEONLY,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SetDataOptions {
        None = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_NONE,
        Discard = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_DISCARD,
        NoOverwrite = sys::FNA3D_SetDataOptions_FNA3D_SETDATAOPTIONS_NOOVERWRITE,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum Blend {
        One = sys::FNA3D_Blend_FNA3D_BLEND_ONE,
        Zero = sys::FNA3D_Blend_FNA3D_BLEND_ZERO,
        SourceColor = sys::FNA3D_Blend_FNA3D_BLEND_SOURCECOLOR,
        InverseSourceColor = sys::FNA3D_Blend_FNA3D_BLEND_INVERSESOURCECOLOR,
        SourceAlpha = sys::FNA3D_Blend_FNA3D_BLEND_SOURCEALPHA,
        InverseSourceAlpha = sys::FNA3D_Blend_FNA3D_BLEND_INVERSESOURCEALPHA,
        DestinationColor = sys::FNA3D_Blend_FNA3D_BLEND_DESTINATIONCOLOR,
        InveseDestinationColo = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEDESTINATIONCOLOR,
        DestinaitonAlpha = sys::FNA3D_Blend_FNA3D_BLEND_DESTINATIONALPHA,
        InverseDetinationAlpha = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEDESTINATIONALPHA,
        BlendFactor = sys::FNA3D_Blend_FNA3D_BLEND_BLENDFACTOR,
        InverseBlendFactor = sys::FNA3D_Blend_FNA3D_BLEND_INVERSEBLENDFACTOR,
        SourceAlphaSaturation = sys::FNA3D_Blend_FNA3D_BLEND_SOURCEALPHASATURATION,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum BlendFunction {
        Add = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_ADD,
        Substract = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_SUBTRACT,
        ReverseSubstract = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_REVERSESUBTRACT,
        Max = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_MAX,
        Min = sys::FNA3D_BlendFunction_FNA3D_BLENDFUNCTION_MIN,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
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
    pub enum TextureAddresMode {
        Wrap = sys::FNA3D_TextureAddressMode_FNA3D_TEXTUREADDRESSMODE_WRAP,
        Clamp = sys::FNA3D_TextureAddressMode_FNA3D_TEXTUREADDRESSMODE_CLAMP,
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
    pub enum VertextElementFormat {
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

// line 854
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum VertextElementUsage {
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
