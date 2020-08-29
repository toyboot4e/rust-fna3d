use fna3d_sys as sys;
use std::ffi::c_void;

pub fn linked_version() -> u32 {
    unsafe { sys::FNA3D_LinkedVersion() }
}

pub fn hook_log_functions_default() {
    unsafe {
        // info, warn, error respectively
        sys::FNA3D_HookLogFunctions(Some(log), Some(log), Some(log));
    }
    // ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>;
    unsafe extern "C" fn log(msg: *const ::std::os::raw::c_char) {
        let slice = ::std::ffi::CStr::from_ptr(msg);
        let string = slice.to_string_lossy().into_owned();
        println!("{}", string);
    }
}

/// [SDL_WindowFlags](https://wiki.libsdl.org/SDL_WindowFlags), which is used for
/// [SDL_CreateWindow](https://wiki.libsdl.org/SDL_CreateWindow)
pub struct SdlWindowFlags(pub u32);

// Init/Quit

/// Selects the most suitable graphics rendering backend for the system, then provides the
/// application with context-sensitive bitflags for the OS window.
///
/// Returns a bitflag value, typically SDL_WindowFlags masks.
pub fn prepare_window_attributes() -> SdlWindowFlags {
    SdlWindowFlags(unsafe { sys::FNA3D_PrepareWindowAttributes() })
}

/// After your window is created, call this to check for high-DPI support.
pub fn get_drawable_size(window: *mut c_void) -> (i32, i32) {
    let (mut w, mut h) = (0, 0);
    unsafe {
        sys::FNA3D_GetDrawableSize(window, &mut w, &mut h);
    }
    (w, h)
}
