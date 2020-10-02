//! MojoShader types and some helpers
//!
//! This module has some helpers in addition to FNA3D items.
//!
//! # Example
//!
//! [Orthograpihcal projection] matrix loading:
//!
//! ```
//! pub fn load_shader_with_orthograpihcal_projection(
//!     device: &mut fna3d::Device,
//!     shader_path: impl AsRef<Path>,
//! ) -> io::Result<(*mut fna3d::Effect, *mut fna3d::mojo::MOJOSHADER_Effect)> {
//!     let (effect, data) = fna3d::mojo::load_shader_path(device, shader_path)?;
//!     fna3d::mojo::set_projection_matrix(fna3d::mojo::ORTHOGRAPHICAL_MATRIX);
//!     (effect, data)
//! }
//! ```
//!
//! [`SpriteEffect.fx`] can be used for the `shader_path`.
//!
//! # Dispose
//!
//! Effect data loaded with helpers in this modules have to be disposed with
//! [`Device::add_dispose_effect`](crate::Device::add_dispose_effect).
//!
//! [orthograpihcal projection]: https://en.wikipedia.org/wiki/Orthographic_projection
//! [`SpriteEffect.fx`]: https://github.com/FNA-XNA/FNA/blob/d3d5840d9f42d109413b9c489af12e5642b336b9/src/Graphics/Effect/StockEffects/HLSL/SpriteEffect.fx

// `FNA3D.h` does not provide concrete MojoShader type definitions e.g. `fna3d_sys::MJOSHADER_Effect`.
// So some types are re-exported from MojoShader headers.

pub type Effect = sys::mojo::MOJOSHADER_effect;
pub type EffectTechnique = sys::mojo::MOJOSHADER_effectTechnique;
pub type EffectStateChanges = sys::mojo::MOJOSHADER_effectStateChanges;
pub type EffectParam = sys::mojo::MOJOSHADER_effectParam;

// --------------------------------------------------------------------------------
// Helpers

use std::{
    ffi::{c_void, CStr},
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fna3d_sys as sys;

#[derive(Debug)]
pub enum LoadShaderError {
    Io(io::Error),
    EffectError(String),
}

pub type Result<T> = std::result::Result<T, LoadShaderError>;

/// Helper for loading shader. Set projection matrix after loading
pub fn load_shader_from_file(
    device: &mut crate::Device,
    shader_path: impl AsRef<Path>,
) -> Result<(*mut crate::Effect, *mut crate::mojo::Effect)> {
    let data = fs::read(shader_path).map_err(|e| LoadShaderError::Io(e))?;
    self::load_shader_from_bytes(device, &data)
}

/// Helper for loading shader. Set projection matrix after loading
///
/// If ok, returns (effect_handle, effect_data_access). The latter is automatically disposed after
/// calling [`fna3d::Device::add_dispose_effect`].
pub fn load_shader_from_bytes(
    device: &mut crate::Device,
    bytes: &[u8],
) -> Result<(*mut crate::Effect, *mut crate::mojo::Effect)> {
    let (effect, mojo_effect) =
        device.create_effect(bytes as *const _ as *mut _, bytes.len() as u32);

    let techniques = unsafe { (*mojo_effect).techniques };
    device.set_effect_technique(effect, techniques);

    // detect error
    let mojo_effect: &mut crate::mojo::Effect = unsafe { &mut *mojo_effect };
    if mojo_effect.error_count <= 0 {
        Ok((effect, mojo_effect))
    } else {
        let errs = unsafe {
            std::slice::from_raw_parts(mojo_effect.techniques, mojo_effect.technique_count as usize)
        };
        let message = format!("{:?}", errs);
        Err(LoadShaderError::EffectError(message))
    }
}

/// Predefined [orthograpihcal projection] matrix
///
/// [orthograpihcal projection]: https://en.wikipedia.org/wiki/Orthographic_projection
pub const ORTHOGRAPHICAL_MATRIX: [f32; 16] = [
    0.0015625, // 2.0 / viewport.w (?)
    0.0,
    0.0,
    -1.0,
    //
    0.0,
    -0.00277777785, // -2.0 / viewport.h (?)
    0.0,
    1.0,
    //
    0.0,
    0.0,
    1.0, // FIXME: sign
    0.0,
    //
    0.0,
    0.0,
    0.0,
    1.0,
];

/// Helper to set projection matrix in **COLUMN-MAJOR** representation
///
/// The matrix considers position vectors as row vectors. So it is often transposed from examples
/// in mathmatical textbooks.
pub fn set_projection_matrix(data: *mut Effect, mat: &[f32; 16]) {
    // FIXME: do not allocate a new string
    let name = std::ffi::CString::new("MatrixTransform").unwrap();

    unsafe {
        assert!(set_param(data, &name, mat));
    }
}

/// Tries to find shader parameter
pub fn find_param(data: *mut Effect, name: &CStr) -> Option<*mut c_void> {
    unsafe {
        for i in 0..(*data).param_count as isize {
            let target_name = (*(*data).params.offset(i)).value.name;
            let target_name = std::ffi::CStr::from_ptr(target_name);
            if target_name != name {
                continue;
            }

            return Some((*(*data).params.offset(i)).value.__bindgen_anon_1.values);
        }
        None
    }
}

/// Returns true if set
pub unsafe fn set_param<T>(data: *mut Effect, name: &CStr, value: &T) -> bool {
    if let Some(ptr) = find_param(data, name) {
        // memcpy
        let n_bytes = std::mem::size_of::<T>();
        let src: &[u8] = std::slice::from_raw_parts_mut(value as *const _ as *mut u8, n_bytes);
        let mut dest = std::slice::from_raw_parts_mut(ptr as *mut u8, n_bytes);
        dest.write(src)
            .expect("failed to write universal effect data");
        true
    } else {
        false
    }
}
