use ::sdl2::{event::Event, video::Window};

use crate::{
    fna3d_renderer::{ImGuiRenderer, RcTexture2d, TextureData2d},
    sdl2_backend::ImguiSdl2,
};

/// Just holds both ImGUI context and backend/renderer of it
pub struct Fna3dImgui {
    /// Call `io_mut` or `ui`
    icx: imgui::Context,
    part: Fna3dImguiPart,
}

pub struct Fna3dImguiPart {
    backend: ImguiSdl2,
    renderer: ImGuiRenderer,
}

impl Fna3dImguiPart {
    pub fn render(
        &mut self,
        ui: imgui::Ui,
        window: &Window,
        device: &fna3d::Device,
    ) -> crate::Result<()> {
        self.backend.prepare_render(&ui, window);
        self.renderer.render(ui.render(), device)
    }
}

impl Fna3dImgui {
    pub fn quick_start(
        device: &fna3d::Device,
        window: &Window,
        display_size: [f32; 2],
        font_size: f32,
        hidpi_factor: f32,
    ) -> crate::Result<Self> {
        let (mut icx, renderer) =
            ImGuiRenderer::quick_start(device, display_size, font_size, hidpi_factor)?;
        let backend = ImguiSdl2::new(&mut icx, window);
        Ok(Self {
            icx,
            part: Fna3dImguiPart { backend, renderer },
        })
    }

    pub fn io_mut(&mut self) -> &mut imgui::Io {
        self.icx.io_mut()
    }

    pub fn font_texture(&self) -> &TextureData2d {
        self.part.renderer.font_texture()
    }

    pub fn textures_mut(&mut self) -> &mut imgui::Textures<RcTexture2d> {
        self.part.renderer.textures_mut()
    }

    pub fn handle_event(&mut self, ev: &Event) -> bool {
        self.part.backend.handle_event(&mut self.icx, ev)
    }

    pub fn frame(
        &mut self,
        window: &impl AsRef<Window>,
        size: [f32; 2],
        scale: [f32; 2],
        dt: f32,
    ) -> (imgui::Ui, &mut Fna3dImguiPart) {
        let mut io = self.icx.io_mut();
        io.display_size = size;
        io.display_framebuffer_scale = scale;
        io.delta_time = dt;

        self.part.backend.prepare_frame(self.icx.io_mut(), window);

        let ui = self.icx.frame();
        (ui, &mut self.part)
    }
}
