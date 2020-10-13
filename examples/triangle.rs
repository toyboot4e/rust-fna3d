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

use crate::common::{
    gfx::{Texture2d, Vertex},
    Result,
};

const W: u32 = 1280;
const H: u32 = 720;

pub fn main() -> Result<()> {
    env_logger::init();

    let title = "Rust-FNA3D triangle example";
    common::run(title, (W, H), self::game_loop)
}

fn game_loop(mut pump: EventPump, device: fna3d::Device) -> Result<()> {
    let mut game = self::GameData::new(device.clone())?;

    'running: loop {
        // Rustified enums are the biggest benefit when using Rust-SDL2 (not Rust-SDL2-sys)!
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
    /// GPU side of things: shader and GPU vertices with attributes
    draw: DrawData,
    /// CPU vertices
    verts: Vec<Vertex>,
    /// GPU texture decoded from `DeadlyStrike.png`
    texture: Texture2d,
}

impl GameData {
    pub fn new(device: fna3d::Device) -> Result<Self> {
        // GPU texture
        let texture = Texture2d::from_undecoded_bytes(&device, common::ICON);

        // CPU vertex buffer
        let color = fna3d::Color::rgb(255, 255, 255);
        let verts = {
            let pos = (100.0, 100.0);
            // 1/2 scale
            let size = (texture.w as f32 / 2.0, texture.h as f32 / 2.0);
            vec![
                // Vertex::new(destination, uv, color) (z values are actually not used)
                Vertex::new([pos.0, pos.1, 0.0], [0.0, 0.0], color),
                Vertex::new([pos.0 + size.0, pos.1, 0.0], [1.0, 0.0], color),
                Vertex::new([pos.0, pos.1 + size.1, 0.0], [0.0, 1.0], color),
                Vertex::new([pos.0 + size.0, pos.1, 0.0], [1.0, 0.0], color),
                Vertex::new([pos.0, pos.1 + size.1, 0.0], [0.0, 1.0], color),
                Vertex::new([pos.0 + size.0, pos.1 + size.1, 0.0], [1.0, 1.0], color),
            ]
        };

        let len_bytes = mem::size_of::<Vertex>() as u32 * verts.len() as u32;
        let draw = DrawData::new(device, len_bytes)?;

        Ok(Self {
            draw,
            verts,
            texture,
        })
    }

    pub fn tick(&mut self, _device: &fna3d::Device) -> Result<()> {
        self.draw.draw_verts(&self.verts, self.texture.raw)?;

        Ok(())
    }
}

/// GPU side of things
pub struct DrawData {
    /// The FNA3D device. Reference counted and disposed automatically
    device: fna3d::Device,
    /// Handle of FNA3D effect. The internals are opaque
    effect: *mut fna3d::Effect,
    /// Access to the internals of FNA3D effect
    effect_data: *mut fna3d::mojo::Effect,
    /// GPU vertices
    vbuf: *mut fna3d::Buffer,
    /// Vertex attributes
    vbind: fna3d::VertexBufferBinding,
}

impl Drop for DrawData {
    fn drop(&mut self) {
        self.device.add_dispose_effect(self.effect);
        self.device.add_dispose_vertex_buffer(self.vbuf);
    }
}

impl DrawData {
    pub fn new(device: fna3d::Device, len_bytes: u32) -> Result<Self> {
        // create SpriteEffect shader (the matrix is not set yet)
        let (effect, effect_data) =
            fna3d::mojo::from_bytes(&device, crate::common::SHADER).map_err(Error::msg)?;

        // GPU vertex buffer (marked as "dynamic")
        let vbuf = device.gen_vertex_buffer(true, fna3d::BufferUsage::None, len_bytes);
        // device.set_vertex_buffer_data(vbuf, 0, &verts, fna3d::SetDataOptions::None);

        // vertex attributes
        // META: such types are just re-exported from FFI to FNA3D and don't have snake_case fields
        let vbind = fna3d::VertexBufferBinding {
            vertexBuffer: vbuf,
            vertexDeclaration: Vertex::declaration(),
            vertexOffset: 0,
            instanceFrequency: 0,
        };

        Ok(Self {
            device,
            effect,
            effect_data,
            vbuf,
            vbind,
        })
    }

    pub fn draw_verts(&mut self, verts: &[Vertex], texture: *mut fna3d::Texture) -> Result<()> {
        // upload CPU vertices to GPU vertices
        // (we don't have to do it every frame though)
        self.device
            .set_vertex_buffer_data(self.vbuf, 0, &verts, fna3d::SetDataOptions::None);

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
            self.device.apply_effect(self.effect, 0, &changes)
        }

        let sampler = fna3d::SamplerState::default();
        let slot = 0;
        self.device.verify_sampler(slot, texture, &sampler);

        // let's make a draw call
        let offset = 0;
        // self.vbind.vertexBuffer = self.vbuf;
        self.vbind.vertexOffset = offset;
        self.device
            .apply_vertex_buffer_bindings(&[self.vbind], true, 0);
        self.device.draw_primitives(
            fna3d::PrimitiveType::TriangleList,
            offset as u32,
            verts.len() as u32,
        );

        Ok(())
    }
}
