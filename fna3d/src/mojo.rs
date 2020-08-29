//! MojoShader types re-exported

use fna3d_sys as sys;

// FNA3D.h does NOT provide concrete definition
// pub type Effect = sys::MOJOSHADER_effect;
// pub type EffectTechnique = sys::MOJOSHADER_effectTechnique;
// pub type EffectStateChanges = sys::MOJOSHADER_effectStateChanges;

// So look at `mojoshader.h`
pub type Effect = sys::mojo::MOJOSHADER_effect;
pub type EffectTechnique = sys::mojo::MOJOSHADER_effectTechnique;
pub type EffectStateChanges = sys::mojo::MOJOSHADER_effectStateChanges;
