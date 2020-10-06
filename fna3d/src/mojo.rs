//! MojoShader types and some helpers
//!
//! This module has some helpers in addition to FNA3D items.
//!
//! # Column-major
//!
//! MojoShader uses column-major matrices, where position vectors are considered as column vectors.
//! If you're using a row-major framework, you have to transpose your matrix when you set it to the
//! projection matrix of MojoShader.
//!
//! # Example
//!
//! [Orthographic projection] matrix loading:
//!
//! ```no_run
//! use std::path::Path;
//!
//! pub fn load_shader_with_orthographic_projection(
//!     device: &fna3d::Device,
//!     shader_path: impl AsRef<Path>,
//! ) -> fna3d::mojo::Result<(*mut fna3d::Effect, *mut fna3d::mojo::Effect)> {
//!     let (effect, data) = fna3d::mojo::from_file(device, shader_path)?;
//!     fna3d::mojo::set_projection_matrix(data, &fna3d::mojo::ORTHOGRAPHICAL_MATRIX);
//!     Ok((effect, data))
//! }
//! ```
//!
//! [`SpriteEffect.fxb`] can be used for the `shader_path`.
//!
//! [Orthographic projection]: https://en.wikipedia.org/wiki/Orthographic_projection
//! [`SpriteEffect.fxb`]: https://github.com/FNA-XNA/FNA/blob/d3d5840d9f42d109413b9c489af12e5642b336b9/src/Graphics/Effect/StockEffects/FXB/SpriteEffect.fxb
//!
//! # Dispose
//!
//! Effect data loaded with helpers in this modules have to be disposed with
//! [`Device::add_dispose_effect`](crate::Device::add_dispose_effect).

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

pub type Result<T> = std::result::Result<T, LoadShaderError>;

#[derive(Debug)]
pub enum LoadShaderError {
    Io(io::Error),
    EffectError(String),
}

/// Helper for loading shader. Be sure to set projection matrix after loading!
pub fn from_file(
    device: &crate::Device,
    shader_path: impl AsRef<Path>,
) -> Result<(*mut crate::Effect, *mut crate::mojo::Effect)> {
    let data = fs::read(shader_path).map_err(|e| LoadShaderError::Io(e))?;
    self::from_bytes(device, &data)
}

/// Helper for loading shader. Be sure to set projection matrix after loading!
///
/// If ok, returns (effect_handle, effect_data_access). The latter is automatically disposed after
/// calling [`fna3d::Device::add_dispose_effect`].
pub fn from_bytes(
    device: &crate::Device,
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

/// Predefined [orthograpihc projection] matrix (column-major)
///
/// [orthograpihc projection]: https://en.wikipedia.org/wiki/Orthographic_projection
pub const ORTHOGRAPHIC_MATRIX: [f32; 16] = [
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
/// Works only for `SpriteEffect.fxb`.
///
/// The matrix considers position vectors as row vectors. So it is often transposed from examples
/// in mathmatical textbooks.
pub fn set_projection_matrix(data: *mut Effect, mat: &[f32; 16]) {
    // FIXME: do not allocate a new string
    let name = std::ffi::CString::new("MatrixTransform").unwrap();

    unsafe {
        if !set_param(data, &name, mat) {
            panic!("could not find MatrixTransform parameter in shader");
        }
    }
}

/// Tries to find a shader parameter with name
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

/// Returns true if the parameter is found
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
