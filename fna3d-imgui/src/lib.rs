//! imgui-rs renderer in Rust-FNA3D
//!
//! Based on the [glium renderer] in the imgui-rs [repository].
//!
//! [glium renderer]: https://github.com/Gekkio/imgui-rs/tree/master/imgui-glium-renderer
//! [repository]: https://github.com/Gekkio/imgui-rs

// TODO: refactoring
// TODO: make examples

mod fna3d_renderer;
mod helper;
mod sdl2_backend;

pub use crate::{
    fna3d_renderer::{ImGuiRendererError, RcTexture2d, Result, TextureData2d},
    helper::Fna3dImgui,
};

/// `SpriteEffect.fxb`
pub const SHARDER: &[u8] = include_bytes!("embedded/SpriteEffect.fxb");

/// `mplus-1p-regular.ttf`
pub const JP_FONT: &[u8] = include_bytes!("embedded/mplus-1p-regular.ttf");
