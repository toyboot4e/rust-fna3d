//! MojoShader types and some helpers
//!
//! Types are re-exported from MojoShader headers beucause `FNA3D.h` does not provide concrete
//! definition
//!
//! * TODO: wrap shader and provide with uniform accessors

use fna3d_sys as sys;

pub type Effect = sys::mojo::MOJOSHADER_effect;
pub type EffectTechnique = sys::mojo::MOJOSHADER_effectTechnique;
pub type EffectStateChanges = sys::mojo::MOJOSHADER_effectStateChanges;

pub const ORTHOGRAPIHCS_MATRIX: [f32; 16] = [
    0.0015625,
    0.0,
    0.0,
    -1.0,
    0.0,
    -0.00277777785,
    0.0,
    1.0,
    0.0,
    0.0,
    1.0,
    0.0,
    0.0,
    0.0,
    0.0,
    1.0,
];

/// Sets a row-major projection matrix to MojoShader
///
/// I don't know the details but it's working.
pub fn set_projection_uniform(data: *mut crate::mojo::Effect, mat: &[f32; 16]) {
    unsafe {
        // cast the matrix to `&[u8]`
        let len = std::mem::size_of::<f32>() * 16;
        let src: &[u8] = std::slice::from_raw_parts_mut(mat.as_ptr() as *mut u8, len);

        // FIXME: do not allocate a new string
        let target_name = std::ffi::CString::new("MatrixTransform").unwrap();

        use std::io::Write;
        for i in 0..(*data).param_count as isize {
            // filter parameters
            let name = (*(*data).params.offset(i)).value.name;
            let name = std::ffi::CStr::from_ptr(name);
            if name != target_name.as_c_str() {
                continue;
            }

            let mut dest = std::slice::from_raw_parts_mut(
                (*(*data).params.offset(i)).value.__bindgen_anon_1.values as *mut u8,
                len,
            );

            dest.write(src)
                .expect("failed to write universal effect data");

            break; // why do we break? is there only one "MatrixTransform"?
        }
    }
}
