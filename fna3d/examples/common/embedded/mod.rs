//! Embedded files

/// SpriteEffect shader
///
/// It has `MatrixTransform` uniform, i.e. orthograpihc projection matrix.
pub const SHADER: &[u8] = include_bytes!("SpriteEffect.fxb");

pub const ICON: &[u8] = include_bytes!("deadly-strike.png");
pub const CASTLE: &[u8] = include_bytes!("castle.png");
