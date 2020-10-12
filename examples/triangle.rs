//! Creates a new window and clears the screen with cornflower-blue color. Then draws some
//! triangles.
//!
//! Utilities came from `examples/common/mod.rs`.

mod common;

use {
    anyhow::Error,
    sdl2::{event::Event, EventPump},
    std::{mem, time::Duration},
};

use crate::common::Result;

const W: u32 = 1280;
const H: u32 = 720;

pub fn main() -> Result<()> {
    env_logger::init();

    let title = "Rust-FNA3D triangle example";
    common::run(title, (W, H), self::game_loop)
}

fn game_loop(mut pump: EventPump, device: fna3d::Device) -> Result<()> {
    let mut game = GameData::new(device.clone())?;

    // Rustified enums are the biggest benefit when using Rust-SDL2 (not Rust-SDL2-sys)!
    'running: loop {
        for ev in pump.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // something like 30 FPS
        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 30));

        device.clear(
            fna3d::ClearOptions::TARGET,
            fna3d::Color::rgb(120, 180, 140).as_vec4(),
            0.0,
            0,
        );

        // update and render
        game.tick(&device)?;

        device.swap_buffers(None, None, std::ptr::null_mut());
    }

    Ok(())
}

pub struct GameData {
    /// The FNA3D device. Reference counted and disposed automatically
    device: fna3d::Device,
    /// Handle of FNA3D effect. The internals are opaque
    effect: *mut fna3d::Effect,
    /// Access to the internals of FNA3D effect
    effect_data: *mut fna3d::mojo::Effect,
    /// CPU vertices
    verts: Vec<Vertex>,
    /// GPU vertices
    vbuf: *mut fna3d::Buffer,
    /// Vertex attributes
    vbind: fna3d::VertexBufferBinding,
    /// Deadily strike
    texture: *mut fna3d::Texture,
}

impl Drop for GameData {
    fn drop(&mut self) {
        self.device.add_dispose_effect(self.effect);
        self.device.add_dispose_vertex_buffer(self.vbuf);
    }
}

impl GameData {
    pub fn new(device: fna3d::Device) -> Result<Self> {
        let (effect, effect_data) =
            fna3d::mojo::from_bytes(&device, crate::common::SHADER).map_err(Error::msg)?;

        // CPU vertex buffer
        let color = fna3d::Color::rgb(255, 255, 255);
        let verts = vec![
            // Vertex::new(destination, uv, color) (z values are actually not used)
            Vertex::new([100.0, 100.0, 0.0], [0.0, 0.0], color),
            Vertex::new([484.0, 100.0, 0.0], [1.0, 0.0], color),
            Vertex::new([100.0, 484.0, 0.0], [0.0, 1.0], color),
        ];

        // GPU vertex buffer (marked as "dynamic")
        let vbuf = device.gen_vertex_buffer(
            true,
            fna3d::BufferUsage::None,
            mem::size_of::<Vertex>() as u32 * verts.len() as u32,
        );
        device.set_vertex_buffer_data(vbuf, 0, &verts, fna3d::SetDataOptions::None);

        // vertex attributes
        // META: such types are just re-exported from FFI to FNA3D and don't have snake_case fields
        let vbind = fna3d::VertexBufferBinding {
            vertexBuffer: vbuf,
            vertexDeclaration: Vertex::declaration(),
            vertexOffset: 0,
            instanceFrequency: 0,
        };

        let texture = {
            let (ptr, len, [w, h]) = fna3d::img::from_undecoded_bytes(common::ICON);

            let texture = device.create_texture_2d(fna3d::SurfaceFormat::Color, w, h, 0, false);
            let data: &[u8] = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            let level = 0; // mipmap level
            device.set_texture_data_2d(texture, 0, 0, w, h, level, data);

            fna3d::img::free(ptr);
            texture
        };

        Ok(Self {
            device,
            effect,
            effect_data,
            verts,
            vbuf,
            vbind,
            texture,
        })
    }

    pub fn tick(&mut self, device: &fna3d::Device) -> Result<()> {
        // device.set_vertex_buffer_data(self.vbuf, 0, &self.verts, fna3d::SetDataOptions::None);

        // set the matrix parameter of the SpriteEffect shader to orthographic projection matrix
        let mat = fna3d::mojo::orthographic_off_center(0.0, W as f32, H as f32, 0.0, 1.0, 0.0);
        let name = "MatrixTransform";
        unsafe {
            let name = std::ffi::CString::new(name)?;
            if !fna3d::mojo::set_param(self.effect_data, &name, &mat) {
                eprintln!("failed to set MatrixTransform shader paramter. maybe not using SpriteEffect.fxb");
            }
        };

        // apply "effect" (shaders in XNA abstraction --  not so good I hear though)
        {
            // no change
            let changes = fna3d::mojo::EffectStateChanges {
                render_state_change_count: 0,
                render_state_changes: std::ptr::null(),
                sampler_state_change_count: 0,
                sampler_state_changes: std::ptr::null(),
                vertex_sampler_state_change_count: 0,
                vertex_sampler_state_changes: std::ptr::null(),
            };
            device.apply_effect(self.effect, 0, &changes)
        }

        let sampler = fna3d::SamplerState::default();
        let slot = 0;
        device.verify_sampler(slot, self.texture, &sampler);

        // let's make a draw call
        let offset = 0;
        // self.vbind.vertexBuffer = self.vbuf;
        self.vbind.vertexOffset = offset;
        device.apply_vertex_buffer_bindings(&[self.vbind], true, 0);
        device.draw_primitives(fna3d::PrimitiveType::TriangleList, offset as u32, 3);

        Ok(())
    }
}

struct Vertex {
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
