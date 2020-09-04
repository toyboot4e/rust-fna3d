//! Wrapper of `FNA3D_Image.h`
//!
//! Iternally, `FNA3D_Image` uses [`stb_image`] (`stbi`) with callback functions and it enables
//! arbitrary IO. However, I'm limiting the input to `Path` because there's a problem if the
//! `Read` is `BufRead` (e.g. `BufReader`). I don't know how to deal with it for now. See
//! `self::StbiCallbackState::read` for details.
//!
//! You also can use [`stb_image`] directory without callbacks or other libraries such as RWops in
//! SDL2.
//!
//! [`stb_image`]: https://github.com/nothings/stb/blob/master/stb_image.h
//!
//! # Example (presudo code)
//!
//! ```no_run
//! pub struct MyTexture2D {
//!     raw: fna3d::Texture,
//!     w: u32,
//!     h: u32,
//! }
//!
//! impl MyTexture2D {
//!     pub fn from_path(
//!         device: &mut fna3d::Device,
//!         path: impl AsRef<std::path::Path>,
//!     ) -> Option<Self> {
//!         let (pixels_ptr, len, [w, h]) = fna3d::img::from_path(path, None);
//!         if pixels_ptr == std::ptr::null_mut() {
//!             return None;
//!         }
//!
//!         let texture: MyTexture2D = unimplemented!();
//!
//!         fna3d::img::free(pixels_ptr as *mut _);
//!
//!         return Some(texture);
//!     }
//! }
//! ```

use fna3d_sys as sys;

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    os::raw::c_void,
    path::Path,
};

/// Callback used to pull data from the stream
pub type ReadFunc = sys::FNA3D_Image_ReadFunc;

/// Callback used to seek around a stream
pub type SkipFunc = sys::FNA3D_Image_SkipFunc;

/// Callback used to check that we're reached the end of a stream
pub type EofFunc = sys::FNA3D_Image_EOFFunc;

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data from an path
///
/// The return type is not `Vec` becaue frees its memory when dropping. Be sure to free the memory
/// with `FNA3D_Image_Free` after use!
pub fn from_path(
    path: impl AsRef<Path>,
    force_size: Option<[u32; 2]>,
) -> (*const u8, u32, [u32; 2]) {
    let path = path.as_ref();
    let reader = File::open(path)
        .ok()
        .unwrap_or_else(|| panic!("failed to open file {}", path.display()));

    let context = StbiCallbackState {
        reader,
        is_end: false,
    };

    unsafe {
        self::load(
            Some(StbiCallbacks::<File>::read),
            Some(StbiCallbacks::<File>::skip),
            Some(StbiCallbacks::<File>::eof),
            std::mem::transmute(&context),
            force_size,
        )
    }
}

/// Free pixels loaded with [`from_reader`]
///
/// [`from_reader`]: ./fn.from_reader.html
pub fn free(mem: *mut u8) {
    unsafe {
        sys::FNA3D_Image_Free(mem);
    }
}

// --------------------------------------------------------------------------------
// impls

struct StbiCallbackState<R: Read + Seek> {
    reader: R,
    is_end: bool,
}

/// Callback functions for `FNA3D_Image.h`, i.e. `stb_image.h`
struct StbiCallbacks<R: Read + Seek> {
    phantom: std::marker::PhantomData<R>,
}

impl<R: Read + Seek> StbiCallbacks<R> {
    /// Casts unknown pointer to the state type
    ///
    /// If we want to remove this transumute function, we (only) have to modify the output of
    /// bindgen. But I prefered to leave it.
    unsafe fn transmute_cx(cx: *mut c_void) -> *mut StbiCallbackState<R> {
        std::mem::transmute(cx)
    }

    /// Reads up to `size` bytes
    unsafe extern "C" fn read(
        context: *mut ::std::os::raw::c_void,
        out_ptr: *mut ::std::os::raw::c_char,
        size: i32,
    ) -> i32 {
        let cx = &mut *Self::transmute_cx(context);

        let out = std::slice::from_raw_parts_mut(out_ptr as *mut u8, size as usize);

        let len_read = cx.reader.read(out).unwrap() as i32;

        // FIXME: This is a hack required for `BufReader`. Maybe it is not safe and I'm limiting
        //        `from_path`.
        // let len_read = if size > 8064 {
        //     cx.reader.read_exact(out).unwrap();
        //     size
        // } else {
        //     cx.reader.read(out).unwrap() as i32
        // };

        log::trace!("stbi readFunc: {} -> {}", size, len_read);

        len_read as i32
    }

    /// FIXME: is this OK?
    ///
    /// Skips `n` bytes
    unsafe extern "C" fn skip(context: *mut ::std::os::raw::c_void, n: i32) {
        log::trace!("stbi skipFunc: {}", n);
        let cx = &mut *Self::transmute_cx(context);
        cx.reader
            .seek(SeekFrom::Current(n as i64))
            .unwrap_or_else(|err| panic!("error in anf skip func {}", err));
    }

    /// FIXME: is this OK?
    unsafe extern "C" fn eof(context: *mut ::std::os::raw::c_void) -> i32 {
        let cx = &mut *Self::transmute_cx(context);
        log::trace!("stbi eofFunc: {}", cx.is_end);
        cx.is_end as i32
    }
}

/// Decodes PuncG/JPG/GIF data into raw RGBA8 texture data.
///
/// * `read_fn`
///   Callback used to pull data from the stream.
/// * `skip_fn`
///   Callback used to seek around a stream.
/// * `eof_fn`
///   Callback used to check that we're reached the end of a stream.
/// * `context`
///   User pointer passed back to the above callbacks.
/// * `force_size`
///   Forced size of the returned image
/// * `do_zoom`
///   When forcing dimensions, enable this to crop instead of stretch.
///
/// Returns a block of memory suitable for use with `FNA3D_SetTextureData2D`.
/// Be sure to free the memory with `FNA3D_Image_Free` after use!
unsafe fn load(
    read_fn: ReadFunc,
    skip_fn: SkipFunc,
    eof_fn: EofFunc,
    context: *mut ::std::os::raw::c_void,
    force_size: Option<[u32; 2]>,
) -> (*const u8, u32, [u32; 2]) {
    let do_zoom = force_size.is_some();
    let force_size = if let Some([x, y]) = force_size {
        [x as i32, y as i32]
    } else {
        [-1, -1]
    };

    let (mut w, mut h, mut len) = (0, 0, 0);
    let pixels = sys::FNA3D_Image_Load(
        read_fn,
        skip_fn,
        eof_fn,
        context,
        &mut w,
        &mut h,
        &mut len,
        force_size[0],
        force_size[1],
        do_zoom as u8,
    );

    (pixels, len as u32, [w as u32, h as u32])
}
