//! Grapics data types

use std::mem;

/// The vertex data
///
/// `#[repr(C)]` is required.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    /// Destination position in pixels
    ///
    /// We don't need the z coordinate but the shader (`SpriteEffect.fxb`) requires it.
    ///
    /// TODO: really? setup 2D only vertices
    dst: [f32; 3],
    /// Color of the vertex
    color: fna3d::Color,
    /// Texture coordinates in normalized range [0, 1] (or wraps if it's out of the range)
    uv: [f32; 2],
}

mod test {
    #[test]
    fn test__() {
        println!("{}", std::mem::align_of::<super::Vertex>());
    }
}

impl Vertex {
    pub fn new(dst: [f32; 3], uv: [f32; 2], color: fna3d::Color) -> Self {
        Self { dst, uv, color }
    }

    /// Vertex attribute elements
    const ELEMS: &'static [fna3d::VertexElement; 3] = &[
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
    pub const DECLARATION: fna3d::VertexDeclaration = fna3d::VertexDeclaration {
        // byte length of the vertex
        vertexStride: mem::size_of::<Vertex>() as i32,
        elementCount: 3,
        elements: Self::ELEMS as *const _ as *mut _,
    };
}

#[derive(Debug, Clone)]
pub struct Texture2d {
    /// Consider using `Rc<TextureDrop>` in real applications
    pub raw: *mut fna3d::Texture,
    pub w: u32,
    pub h: u32,
}

impl Texture2d {
    /// For use with `include_bytes!`
    pub fn from_encoded_bytes(device: &fna3d::Device, bytes: &[u8]) -> Self {
        let (ptr, len, [w, h]) = fna3d::img::from_encoded_bytes(bytes);

        if ptr == std::ptr::null_mut() {
            panic!("Unable to read the encoded bytes as an image!");
        }

        // setup a GPU texture
        let raw = {
            let texture = device.create_texture_2d(fna3d::SurfaceFormat::Color, w, h, 0, false);

            let pixels: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            let level = 0; // mipmap level
            device.set_texture_data_2d(texture, 0, 0, w, h, level, pixels);

            texture
        };

        // free the CPU texture
        fna3d::img::free(ptr);

        Self { raw, w, h }
    }
}
