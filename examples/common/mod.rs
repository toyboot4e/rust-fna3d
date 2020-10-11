//! Common utilities among samples

use sdl2::{event::Event, EventPump};

pub type Result<T> = std::result::Result<T, String>;

/// Runs SDL2 + FNA3D game in a simple way
pub fn run(
    title: &str,
    size: (u32, u32),
    game_loop: impl FnOnce(EventPump, fna3d::Device) -> Result<()>,
) -> Result<()> {
    let (sdl, vid, win) = {
        let flags = fna3d::prepare_window_attributes();

        let sdl = sdl2::init()?;
        let vid = sdl.video()?;
        let win = vid
            .window(title, size.0, size.1)
            .set_window_flags(flags.0)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let size = fna3d::get_drawable_size(win.raw() as *mut _);
        log::info!("FNA3D drawable size: [{}, {}]", size.0, size.1);

        (sdl, vid, win)
    };

    let (params, device) = {
        let params = fna3d::utils::default_params_from_window_handle(win.raw() as *mut _);
        let device = fna3d::Device::from_params(params, true);

        let vp = fna3d::Viewport {
            x: 0,
            y: 0,
            w: params.backBufferWidth as i32,
            h: params.backBufferHeight as i32,
            minDepth: 0.0,
            maxDepth: 1.0, // TODO: what's this
        };
        device.set_viewport(&vp);

        let rst = fna3d::RasterizerState::default();
        device.apply_rasterizer_state(&rst);

        let bst = fna3d::BlendState::alpha_blend();
        device.set_blend_state(&bst);

        (params, device)
    };

    (game_loop)(sdl.event_pump()?, device)
}
