//! Common utilities among samples

pub mod embedded;
pub mod gfx;

use {anyhow::Error, sdl2::EventPump};

pub type Result<T> = anyhow::Result<T>;

pub struct Init {
    pub sdl: sdl2::Sdl,
    pub vid: sdl2::VideoSubsystem,
    pub win: sdl2::video::Window,
    pub params: fna3d::PresentationParameters,
    pub device: fna3d::Device,
}

impl Init {
    /// Use it when calling [`fna3d::Device::swap_buffers`]
    pub fn raw_window(&self) -> *mut sdl2::sys::SDL_Window {
        self.win.raw()
    }
}

pub fn init(title: &str, size: (u32, u32)) -> Result<Init> {
    log::info!("FNA3D linked version: {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let (sdl, vid, win) = {
        let flags = fna3d::prepare_window_attributes();

        // `map_err(Error:msg)` came from `anyhow`
        let sdl = sdl2::init().map_err(Error::msg)?;
        let vid = sdl.video().map_err(Error::msg)?;
        let win = vid
            .window(title, size.0, size.1)
            .set_window_flags(flags.0)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .map_err(Error::msg)?;

        let size = fna3d::get_drawable_size(win.raw() as *mut _);
        log::info!("FNA3D drawable size: [{}, {}]", size.0, size.1);

        (sdl, vid, win)
    };

    let (params, device) = {
        let params = fna3d::utils::default_params_from_window_handle(win.raw() as *mut _);
        let device = fna3d::Device::from_params(params, true);

        {
            let (max_tx, max_v_tx) = device.get_max_texture_slots();
            log::info!("device max textures: {}", max_tx);
            log::info!("device max vertex textures: {}", max_v_tx);
        }

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

        // let dsst = fna3d::DepthStencilState::default();
        // device.set_depth_stencil_state(&dsst);

        let bst = fna3d::BlendState::alpha_blend();
        device.set_blend_state(&bst);

        (params, device)
    };

    Ok(Init {
        sdl,
        vid,
        win,
        params,
        device,
    })
}
