use fna3d_sys as sys;

// this should be replaced with `std::ffi::c_void`
use std::os::raw::c_void;

// from line 2301
pub fn linked_version() -> u32 {
    unsafe { sys::FNA3D_LinkedVersion() }
}

// TODO: read Rust nomicon and do with function pointers?
// pub type LogFunc = sys::FNA3D_LogFunc;
// extern "C" {
//     pub fn FNA3D_HookLogFunctions(info: FNA3D_LogFunc, warn: FNA3D_LogFunc, error: FNA3D_LogFunc);
// }

pub fn prepare_window_attributes() -> u32 {
    unsafe { sys::FNA3D_PrepareWindowAttributes() }
}

pub fn get_drawable_size(window: *mut c_void) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    unsafe {
        sys::FNA3D_GetDrawableSize(window, &mut x, &mut y);
    }
    (x, y)
}
