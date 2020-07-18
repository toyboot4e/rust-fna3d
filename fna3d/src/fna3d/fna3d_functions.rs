use fna3d_sys as sys;

// this should be replaced with `std::ffi::c_void`
use std::os::raw::c_void;

// from line 2301
pub fn linked_version() -> u32 {
    unsafe { sys::FNA3D_LinkedVersion() }
}

pub fn hook_log_functions_default() {
    unsafe {
        sys::FNA3D_HookLogFunctions(Some(log), Some(log), Some(log));
    }
    // ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>;
    unsafe extern "C" fn log(msg: *const ::std::os::raw::c_char) {
        let slice = ::std::ffi::CStr::from_ptr(msg);
        let string = slice.to_string_lossy().into_owned();
        println!("string buffer size without nul terminator: {}", string);
    }
}

/// [SDL_WindowFlags](https://wiki.libsdl.org/SDL_WindowFlags), which is required by
/// [SDL_CreateWindow](https://wiki.libsdl.org/SDL_CreateWindow)
pub struct SdlWindowFlags(u32);

pub fn prepare_window_attributes() -> SdlWindowFlags {
    SdlWindowFlags(unsafe { sys::FNA3D_PrepareWindowAttributes() })
}

pub fn get_drawable_size(window: *mut c_void) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    unsafe {
        sys::FNA3D_GetDrawableSize(window, &mut x, &mut y);
    }
    (x, y)
}
