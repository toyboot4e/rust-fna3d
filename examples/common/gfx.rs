//! Grapics data types

use {
    anyhow::{Error, Result},
    std::mem,
};

use super::embedded;

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
    pub dst: [f32; 3],
    /// Color of the vertex
    pub color: fna3d::Color,
    /// Texture coordinates in normalized range [0, 1] (or wraps if it's out of the range)
    pub uv: [f32; 2],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            dst: [0.0, 0.0, 0.0],
            color: fna3d::Color::rgba(255, 255, 255, 255),
            uv: [0.0, 0.0],
        }
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

/// GPU texture
///
/// # Safety
///
/// It's NOT disposed automatically. Very unsafe!
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
            let texture = device.create_texture_2d(fna3d::SurfaceFormat::Color, w, h, 1, false);
            let pixels: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            device.set_texture_data_2d(texture, 0, 0, w, h, 0, pixels);

            texture
        };

        // free the CPU texture
        fna3d::img::free(ptr);

        Self { raw, w, h }
    }
}

/// `SpriteEffect`, one of "Effects" in XNA
///
/// It's a combination of vertex/fragment shaders. I hear that it's not a good abstraction though
#[derive(Debug)]
pub struct Shader2d {
    device: fna3d::Device,
    effect: *mut fna3d::Effect,
    effect_data: *mut fna3d::mojo::Effect,
}

impl Drop for Shader2d {
    fn drop(&mut self) {
        // frees both `effect` and `effect_data`
        self.device.add_dispose_effect(self.effect);
    }
}

impl Shader2d {
    /// Create SpriteEffect from FNA3D device and the screen size
    pub fn new(device: &fna3d::Device, w: u32, h: u32) -> Result<Self> {
        // create the `SpriteEffect` shader
        let (effect, effect_data) =
            fna3d::mojo::from_bytes(&device, embedded::SHADER).map_err(Error::msg)?;

        // set the matrix parameter of the SpriteEffect shader to orthographic projection matrix
        {
            let mat = fna3d::mojo::orthographic_off_center(0.0, w as f32, h as f32, 0.0, 1.0, 0.0);
            // the name is hardcoded to the original shader source file (`SpriteEffect.fx`)
            let name = "MatrixTransform";
            unsafe {
                let name = std::ffi::CString::new(name)?;
                if !fna3d::mojo::set_param(effect_data, &name, &mat) {
                    eprintln!("Failed to set MatrixTransform shader paramter. Probablly we're not using `SpriteEffect.fxb`");
                }
            };
        }

        Ok(Self {
            device: device.clone(),
            effect,
            effect_data,
        })
    }

    pub fn apply_to_device(&self) {
        let pass = 0;
        self.device
            .apply_effect(self.effect, pass, &fna3d::utils::no_change_effect());
    }
}
