//! Creates a new window and clears the screen with cornflower-blue color. Then draws the texture.
//!
//! Rust-FNA3D is not so easy t use, unfortunatelly.
//!
//! Utilities came from `examples/common/mod.rs`.

mod common;

use {
    anyhow::{Error, Result},
    sdl2::{event::Event, EventPump},
    std::{mem, time::Duration},
};

use self::common::{
    embedded,
    gfx::{Texture2d, Vertex},
};

const W: u32 = 1280;
const H: u32 = 720;

pub fn main() -> Result<()> {
    env_logger::init();

    let title = "Rust-FNA3D triangle example";

    let init = common::init(title, (W, H))?;
    let pump = init.sdl.event_pump().map_err(Error::msg)?;

    self::run(pump, init)
}

fn run(mut pump: EventPump, init: common::Init) -> Result<()> {
    let mut game = self::GameData::new(init)?;

    'running: loop {
        // Rustified enums are the biggest benefit when using Rust-SDL2 (not Rust-SDL2-sys)!
        for ev in pump.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // something like 30 FPS. do not use it for real applications
        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 30));

        // update and render our game
        game.tick()?;
    }

    Ok(())
}

pub struct GameData {
    /// Lifetime of the application
    init: common::Init,
    /// GPU side of things: shader and GPU vertices with attributes
    draw: DrawData,
    /// CPU vertices
    verts: Vec<Vertex>,
    /// GPU texture decoded from `DeadlyStrike.png`
    texture: Texture2d,
}

impl Drop for GameData {
    fn drop(&mut self) {
        self.init.device.add_dispose_texture(self.texture.raw);
    }
}

impl GameData {
    pub fn new(init: common::Init) -> Result<Self> {
        // GPU texture
        let texture = Texture2d::from_encoded_bytes(&init.device, embedded::ICON);

        // CPU vertex buffer
        let color = fna3d::Color::rgb(255, 255, 255);
        let verts = {
            // 1/2 scale
            let pos = (100.0, 100.0);
            let size = (texture.w as f32 / 2.0, texture.h as f32 / 2.0);

            vec![
                // Vertex::new(destination, uv, color) (z values are not used actually)
                Vertex::new([pos.0, pos.1, 0.0], [0.0, 0.0], color),
                Vertex::new([pos.0 + size.0, pos.1, 0.0], [1.0, 0.0], color),
                Vertex::new([pos.0, pos.1 + size.1, 0.0], [0.0, 1.0], color),
                Vertex::new([pos.0 + size.0, pos.1 + size.1, 0.0], [1.0, 1.0], color),
            ]
        };

        let draw = DrawData::new(init.device.clone(), verts.len() as u32)?;

        Ok(Self {
            init,
            draw,
            verts,
            texture,
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        {
            let depth = 0.0;
            let stencil = 0;
            self.init.device.clear(
                fna3d::ClearOptions::TARGET,
                fna3d::Color::rgb(120, 180, 140).to_vec4(),
                depth,
                stencil,
            );
        }

        self.draw.draw_quads(&self.verts, self.texture.raw)?;

        self.init
            .device
            .swap_buffers(None, None, self.init.raw_window() as *mut _);

        Ok(())
    }
}

/// GPU side of things
#[derive(Debug)]
pub struct DrawData {
    /// FNA3D device
    device: fna3d::Device,
    /// Handle of FNA3D effect. The internals are opaque
    effect: *mut fna3d::Effect,
    /// Access to the internals of FNA3D effect.
    effect_data: *mut fna3d::mojo::Effect,
    /// GPU vertex buffer
    vbuf: *mut fna3d::Buffer,
    /// Vertex attributes of the vertex buffer that can be uploaded to GPU
    vbind: fna3d::VertexBufferBinding,
    /// GPU index buffer, setup only for quadliterals
    ibuf: *mut fna3d::Buffer,
}

impl Drop for DrawData {
    fn drop(&mut self) {
        // frees both `effect` and `effect_data`
        self.device.add_dispose_effect(self.effect);

        self.device.add_dispose_vertex_buffer(self.vbuf);
        self.device.add_dispose_index_buffer(self.ibuf);
    }
}

impl DrawData {
    pub fn new(device: fna3d::Device, n_verts: u32) -> Result<Self> {
        // create the `SpriteEffect` shader
        let (effect, effect_data) =
            fna3d::mojo::from_bytes(&device, embedded::SHADER).map_err(Error::msg)?;

        // set the matrix parameter of the SpriteEffect shader to orthographic projection matrix
        {
            let mat = fna3d::mojo::orthographic_off_center(0.0, W as f32, H as f32, 0.0, 1.0, 0.0);
            // the name is hardcoded to the original shader source file (`SpriteEffect.fx`)
            let name = "MatrixTransform";
            unsafe {
                let name = std::ffi::CString::new(name)?;
                if !fna3d::mojo::set_param(effect_data, &name, &mat) {
                    eprintln!("Failed to set MatrixTransform shader paramter. Probablly we're not using `SpriteEffect.fxb`");
                }
            };
        }

        // GPU vertex buffer (marked as "dynamic")
        let vbuf = device.gen_vertex_buffer(
            true,
            fna3d::BufferUsage::None,
            n_verts * mem::size_of::<Vertex>() as u32,
        );

        // GPU index buffer (marked as "static")
        let ibuf = device.gen_index_buffer(false, fna3d::BufferUsage::None, 16 * n_verts);
        {
            let data = [0 as i16, 1, 2, 3, 2, 1];
            device.set_index_buffer_data(ibuf, 0, &data, fna3d::SetDataOptions::None);
        }

        // vertex attributes
        // META: such types are just re-exported from FFI to FNA3D and don't have snake_case fields
        let vbind = fna3d::VertexBufferBinding {
            vertexBuffer: vbuf,
            vertexDeclaration: Vertex::DECLARATION,
            vertexOffset: 0,
            instanceFrequency: 0,
        };

        Ok(Self {
            device,
            effect,
            effect_data,
            vbuf,
            vbind,
            ibuf,
        })
    }

    pub fn draw_quads(&mut self, verts: &[Vertex], texture: *mut fna3d::Texture) -> Result<()> {
        // "effect" in XNA is an abstraction over shaders --  not so good I hear though)
        {
            let pass = 0;
            self.device
                .apply_effect(self.effect, pass, &fna3d::utils::no_change_effect());
        }

        // upload the CPU vertices to the GPU vertices (we don't have to do it every frame in though)
        {
            let offset = 0;
            self.device.set_vertex_buffer_data(
                self.vbuf,
                offset,
                &verts,
                fna3d::SetDataOptions::None,
            );
        }

        {
            // FIXME:
            // let sampler = fna3d::SamplerState::default();
            let sampler = {
                let mut s = fna3d::SamplerState::linear_clamp();
                s.set_address_u(fna3d::TextureAddressMode::Wrap);
                s.set_filter(fna3d::TextureFilter::Point);
                s
            };
            let slot = 0;
            self.device.verify_sampler(slot, texture, &sampler);
        }

        // let's make a draw call
        {
            let base_vtx = 0;
            let base_vtx_idx = 0;
            let n_verts = verts.len() as u32;
            let base_idx = 0;
            let n_triangles = n_verts / 2;

            self.device
                .apply_vertex_buffer_bindings(&[self.vbind], true, base_vtx);

            self.device.draw_indexed_primitives(
                fna3d::PrimitiveType::TriangleList,
                base_vtx,
                base_vtx_idx,
                n_verts,
                base_idx,
                n_triangles,
                self.ibuf,
                fna3d::IndexElementSize::Bits16,
            );
        }

        Ok(())
    }
}
