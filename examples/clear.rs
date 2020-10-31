//! Creates a new window and clears the screen with cornflower-blue color
//!
//! Utilities came from `examples/common/mod.rs`.

mod common;

use {
    anyhow::{Error, Result},
    sdl2::{event::Event, EventPump},
    std::time::Duration,
};

pub fn main() -> Result<()> {
    env_logger::init();

    let title = "Rust-FNA3D triangle example";
    let size = (640, 360);

    let init = common::init(title, size)?;
    let pump = init.sdl.event_pump().map_err(Error::msg)?;

    self::run(pump, init)
}

fn run(mut pump: EventPump, init: common::Init) -> Result<()> {
    let device = init.device.clone();

    'running: loop {
        // Rustified enums are the biggest benefit when using Rust-SDL2 (not Rust-SDL2-sys)!
        for ev in pump.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // something like 30 FPS. do not use it for real applications.
        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 30));

        // clear the screen (the back frame buffer)
        device.clear(
            fna3d::ClearOptions::TARGET,
            fna3d::Color::cornflower_blue().to_vec4(),
            0.0,
            0,
        );

        // process your game here

        // present the back frame buffer onto the screen
        device.swap_buffers(None, None, init.raw_window() as *mut _);
    }

    Ok(())
}
