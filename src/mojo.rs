//! MojoShader types and some helpers
//!
//! This module has some helpers in addition to the original items.
//!
//! # Effect
//!
//! Effect is an abstraction over shaders in XNA. It's actually not so good but we have to stick
//! with it if we use FNA3D.
//!
//! For compiling `fx_2_0` on macOS Catalina, see [this] repository.
//!
//! [this]: https://github.com/toyboot4e/fxc
//!
//! # Column-major
//!
//! MojoShader uses column-major matrices, where position vectors are considered as column vectors.
//!
//! FNA is using row-major matrices while MojoShader is column-major.
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
//! /// SpriteEffect.fxb with orthographic projection matrix
//! pub fn load_2d_shader(
//!     device: &fna3d::Device,
//!     shader_path: impl AsRef<Path>,
//! ) -> fna3d::mojo::Result<(*mut fna3d::Effect, *mut fna3d::mojo::Effect)> {
//!     let (effect, effect_data) = fna3d::mojo::from_file(device, shader_path)?;
//!     let mat = fna3d::mojo::orthographic_off_center(0.0, 1280.0, 720.0, 0.0, 1.0, 0.0);
//!     let name = std::ffi::CString::new("MatrixTransform").unwrap();
//!     unsafe {
//!         assert!(fna3d::mojo::set_param(effect_data, &name, &mat));
//!     }
//!     Ok((effect, effect_data))
//! }
//! ```
//!
//! [`SpriteEffect.fxb`] could be used for the `shader_path`.
//!
//! [Orthographic projection]: https://en.wikipedia.org/wiki/Orthographic_projection
//! [`SpriteEffect.fxb`]: https://github.com/FNA-XNA/FNA/blob/d3d5840d9f42d109413b9c489af12e5642b336b9/src/Graphics/Effect/StockEffects/FXB/SpriteEffect.fxb
//!
//! # Dispose
//!
//! [`crate::Effect`] loaded with a helper in this modules have to be disposed with
//! [`Device::add_dispose_effect`](crate::Device::add_dispose_effect). Then [`crate::mojo::Effect`]
//! is also disposed.

// `FNA3D.h` does not provide concrete MojoShader type definitions e.g. `fna3d_sys::MJOSHADER_Effect`.
// So some types are re-exported from MojoShader headers.

pub type Effect = sys::mojo::MOJOSHADER_effect;
pub type EffectTechnique = sys::mojo::MOJOSHADER_effectTechnique;
pub type EffectStateChanges = sys::mojo::MOJOSHADER_effectStateChanges;
pub type EffectParam = sys::mojo::MOJOSHADER_effectParam;

// --------------------------------------------------------------------------------
// Helpers

use ::{
    fna3d_sys as sys,
    std::{
        ffi::{c_void, CStr},
        fmt, fs,
        io::{self, prelude::*},
        path::Path,
    },
};

pub type Result<T> = std::result::Result<T, LoadShaderError>;

#[derive(Debug)]
pub enum LoadShaderError {
    Io(io::Error),
    EffectError(String),
}

impl fmt::Display for LoadShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadShaderError::Io(err) => write!(f, "{}", err),
            LoadShaderError::EffectError(err) => write!(f, "Shader loading errors: {}", err),
        }
    }
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

/// Column-major orthographic matrix
///
/// `fna3d::mojo::orthographic_off_center(0.0, width, height, 0.0, 1.0, 0.0);`
///
/// * bottom is down and top is up, so `bottom` > `top`
/// * z axis goes from the screen to your face, so `near` > `far`
pub fn orthographic_off_center(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [f32; 16] {
    // TODO: does it make sense to calculate in f64 and then cast to f32
    [
        (2.0 / (right as f64 - left as f64)) as f32,
        0.0,
        0.0,
        -((right as f64 + left as f64) / (right as f64 - left as f64)) as f32,
        //
        0.0,
        (2.0 / (top as f64 - bottom as f64)) as f32,
        0.0,
        -((top as f64 + bottom as f64) / (top as f64 - bottom as f64)) as f32,
        //
        0.0,
        0.0,
        // FNA (TODO: which is correct FNA or Wiki)
        -(1.0 / (far as f64 - near as f64)) as f32,
        (near as f64 / (near as f64 - far as f64)) as f32,
        // wiki
        // -(2.0 / (far as f64 - near as f64)) as f32,
        // -((far as f64 + near as f64) / (far as f64 - near as f64)) as f32,
        //
        0.0,
        0.0,
        0.0,
        1.0,
    ]
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
    let ptr = match self::find_param(data, name) {
        Some(ptr) => ptr,
        None => return false,
    };

    // memcpy
    let n_bytes = std::mem::size_of::<T>();
    let src: &[u8] = std::slice::from_raw_parts_mut(value as *const _ as *mut u8, n_bytes);
    let mut dest = std::slice::from_raw_parts_mut(ptr as *mut u8, n_bytes);
    dest.write(src)
        .expect("failed to write universal effect data");

    true
}
