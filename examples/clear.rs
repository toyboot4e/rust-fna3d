//! Creates a new window and clears the screen with cornflower-blue color

mod common;

use {
    sdl2::{event::Event, EventPump},
    std::time::Duration,
};

use crate::common::Result;

pub fn main() -> Result<()> {
    env_logger::init();

    log::info!("FNA3D linked version: {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let title = "Rust-FNA3D triangle example";
    let size = (640, 360);

    common::run(title, size, self::game_loop)
}

fn game_loop(mut pump: EventPump, device: fna3d::Device) -> Result<()> {
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
            fna3d::Color::cornflower_blue(),
            0.0,
            0,
        );
        self::tick_game(&device);
        device.swap_buffers(None, None, std::ptr::null_mut());
    }

    Ok(())
}

fn tick_game(device: &fna3d::Device) {
    //
}
