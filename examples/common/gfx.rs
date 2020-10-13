//! Grapics data types

use std::mem;

pub struct Vertex {
    /// Destination position in pixels
    ///
    /// We don't need the z coordinate but the shader (`SpriteEffect.fxb`) requires it.
    ///
    /// * TODO: set up 2D only shader
    dst: [f32; 3],
    /// Color of the vertex
    color: fna3d::Color,
    /// Texture coordinates in normalized range [0, 1] (or wraps if it's out of the range)
    uv: [f32; 2],
}

impl Vertex {
    pub fn new(dst: [f32; 3], uv: [f32; 2], color: fna3d::Color) -> Self {
        Self { dst, uv, color }
    }

    /// Vertex attributes (elements)
    const ELEMS: [fna3d::VertexElement; 3] = [
        // offsets are in bytes
        fna3d::VertexElement {
            offset: 0,
            vertexElementFormat: fna3d::VertexElementFormat::Vector3 as u32,
            vertexElementUsage: fna3d::VertexElementUsage::Position as u32,
            usageIndex: 0,
        },
        fna3d::VertexElement {
            offset: 12,
            vertexElementFormat: fna3d::VertexElementFormat::Color as u32,
            vertexElementUsage: fna3d::VertexElementUsage::Color as u32,
            usageIndex: 0,
        },
        fna3d::VertexElement {
            offset: 16,
            vertexElementFormat: fna3d::VertexElementFormat::Vector2 as u32,
            vertexElementUsage: fna3d::VertexElementUsage::TextureCoordinate as u32,
            usageIndex: 0,
        },
    ];

    /// Vertex attributes
    pub fn declaration() -> fna3d::VertexDeclaration {
        fna3d::VertexDeclaration {
            // byte length of the vertex
            vertexStride: mem::size_of::<Vertex>() as i32,
            elementCount: 3,
            elements: Self::ELEMS.as_ptr() as *mut _,
        }
    }
}

pub struct Texture2d {
    pub raw: *mut fna3d::Texture,
    pub w: u32,
    pub h: u32,
}

impl Texture2d {
    pub fn from_undecoded_bytes(device: &fna3d::Device, bytes: &[u8]) -> Self {
        let (ptr, len, [w, h]) = fna3d::img::from_undecoded_bytes(bytes);

        let texture = device.create_texture_2d(fna3d::SurfaceFormat::Color, w, h, 0, false);
        let level = 0; // mipmap level
        let data: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
        device.set_texture_data_2d(texture, 0, 0, w, h, level, data);

        // unload pixel data in CPU side
        fna3d::img::free(ptr);

        Self { raw: texture, w, h }
    }
}
