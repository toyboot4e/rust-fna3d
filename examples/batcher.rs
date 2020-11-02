//! `SpriteBatch` in FNA
//!
//! Not that tested but the idea is there.

mod common;

use {
    anyhow::{Error, Result},
    fna3d::Color,
    sdl2::{event::Event, EventPump},
    std::time::Duration,
};

use self::common::{
    batch::{Batcher, QuadData},
    embedded,
    gfx::{Shader2d, Texture2dDrop, Vertex},
};

const W: u32 = 1280;
const H: u32 = 720;

pub fn main() -> Result<()> {
    env_logger::init();

    let title = "Rust-FNA3D batcher example";

    let init = common::init(title, (W, H))?;
    let pump = init.sdl.event_pump().map_err(Error::msg)?;

    self::run(pump, init)
}

fn run(mut pump: EventPump, init: common::Init) -> Result<()> {
    let shader = Shader2d::new(&init.device, W, H)?;
    let mut game = self::GameData::new(init, shader)?;

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
        println!("---- new frame");
        game.tick()?;
    }

    Ok(())
}

pub struct GameData {
    /// Lifetime of the application
    init: common::Init,
    /// Batcher of draw calls
    batcher: Batcher,
    deadly_strike: Texture2dDrop,
    castle: Texture2dDrop,
}

impl GameData {
    pub fn new(init: common::Init, shader: Shader2d) -> Result<Self> {
        // GPU texture
        let deadly_strike = Texture2dDrop::from_encoded_bytes(&init.device, embedded::ICON);
        let castle = Texture2dDrop::from_encoded_bytes(&init.device, embedded::CASTLE);

        let batcher = Batcher::new(&init.device, shader)?;

        Ok(Self {
            init,
            batcher,
            deadly_strike,
            castle,
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        self.init.device.clear(
            fna3d::ClearOptions::TARGET,
            Color::rgb(120, 180, 140).to_vec4(),
            0.0, // depth
            0,   // stencil
        );

        self.render();

        self.batcher.flush();

        self.init
            .device
            .swap_buffers(None, None, self.init.raw_window() as *mut _);

        Ok(())
    }

    fn render(&mut self) {
        let size = [200.0, 200.0];

        for i in 0..4 {
            let tex = if i % 2 == 0 {
                self.deadly_strike.raw
            } else {
                self.castle.raw
            };

            // push 5 quadliterals
            for j in 0..5 {
                let pos = [
                    100.0 + 5.0 * j as f32 + 150.0 * i as f32,
                    100.0 + 40.0 * j as f32,
                ];

                let color = Color::rgba(255, 255, 255, 255);
                let quad = QuadData([
                    Vertex {
                        dst: [pos[0], pos[1], 0.0],
                        color,
                        uv: [0.0, 0.0],
                    },
                    Vertex {
                        dst: [pos[0] + size[0], pos[1], 0.0],
                        color,
                        uv: [1.0, 0.0],
                    },
                    Vertex {
                        dst: [pos[0], pos[1] + size[1], 0.0],
                        color,
                        uv: [0.0, 1.0],
                    },
                    Vertex {
                        dst: [pos[0] + size[0], pos[1] + size[1], 0.0],
                        color,
                        uv: [1.0, 1.0],
                    },
                ]);

                self.batcher.push_quad(&quad, tex);
            }
        }
    }
}
