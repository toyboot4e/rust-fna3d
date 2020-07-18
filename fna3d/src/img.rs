//! `FNA3D_Image.h`

use fna3d_sys as sys;

/// Callback used to pull data from the stream
pub type ReadFunc = sys::FNA3D_Image_ReadFunc;

/// Callback used to seek around a stream
pub type SkipFunc = sys::FNA3D_Image_SkipFunc;

/// Callback used to check that we're reached the end of a stream
pub type EofFunc = sys::FNA3D_Image_EOFFunc;

use std::{
    io::{Read, Seek, SeekFrom},
    os::raw::c_void,
};

// TODO: detect error (nullptr returned)

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data from an arbitrary IO
///
/// FNA3D_Image uses `stb_image` (`stbi`) and it uses callback functions to enable arbitrary IO.
///
/// You may want to wrap a `Read` struct with `std::io::BufReader`. Be sure to free the memory with
/// `FNA3D_Image_Free` after use!
pub fn load_image_from_reader<R: Read + Seek>(
    reader: R,
    force_size: Option<[u32; 2]>,
    do_zoom: bool,
) -> (*mut u8, usize, [u32; 2]) {
    let context = StbiCallbackState {
        reader,
        is_end: false,
    };
    self::load(
        Some(StbiCallbacks::<R>::read),
        Some(StbiCallbacks::<R>::skip),
        Some(StbiCallbacks::<R>::eof),
        unsafe { std::mem::transmute(&context) },
        force_size,
        do_zoom,
    )
}

struct StbiCallbackState<R: Read + Seek> {
    reader: R,
    is_end: bool,
}

/// Callbacks for `stb_image` used via `FNA3D_Image`
struct StbiCallbacks<R: Read + Seek> {
    phantom: std::marker::PhantomData<R>,
}

impl<R: Read + Seek> StbiCallbacks<R> {
    fn transmute_cx(cx: *mut c_void) -> *mut StbiCallbackState<R> {
        // TODO: error if it's an invalid pointer
        unsafe { std::mem::transmute(cx) }
    }

    /// Reads up to `size` bytes
    unsafe extern "C" fn read(
        context: *mut ::std::os::raw::c_void,
        out_ptr: *mut ::std::os::raw::c_char,
        out_size: i32,
    ) -> i32 {
        let cx = &mut *Self::transmute_cx(context);

        let out = std::slice::from_raw_parts_mut(out_ptr as *mut u8, out_size as usize);
        // FIXME: this is a hack
        let len_read = if out_size > 8064 {
            cx.reader.read_exact(out).unwrap();
            out_size
        } else {
            cx.reader.read(out).unwrap() as i32
        };

        log::trace!("stbi readFunc: {} -> {}", out_size, len_read);

        len_read as i32
    }

    /// Skips `n` bytes
    unsafe extern "C" fn skip(context: *mut ::std::os::raw::c_void, n: i32) {
        println!("skip n: {}", n);
        let cx = &mut *Self::transmute_cx(context);
        cx.reader
            .seek(SeekFrom::Current(n as i64))
            .unwrap_or_else(|err| panic!("error in anf skip func {}", err));
    }

    // TODO: do we have to peek??
    unsafe extern "C" fn eof(context: *mut ::std::os::raw::c_void) -> i32 {
        let cx = &mut *Self::transmute_cx(context);
        println!("eof fn: {}", cx.is_end);
        cx.is_end as i32
    }
}

// --------------------------------------------------------------------------------
// impls

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
fn load(
    read_fn: ReadFunc,
    skip_fn: SkipFunc,
    eof_fn: EofFunc,
    context: *mut ::std::os::raw::c_void,
    force_size: Option<[u32; 2]>,
    do_zoom: bool, // TODO: need this? (or use force_size.is_some()?)
) -> (*mut u8, usize, [u32; 2]) {
    let (mut w, mut h, mut len) = (0, 0, 0);
    let force_size = if let Some([x, y]) = force_size {
        [x as i32, y as i32]
    } else {
        [-1, -1]
    };
    let pixels = unsafe {
        sys::FNA3D_Image_Load(
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
        )
    };
    (pixels, len as usize, [w as u32, h as u32])
}

// extern "C" {
//     pub fn FNA3D_Image_Free(mem: *mut u8);
// }

// pub type FNA3D_Image_WriteFunc = ::std::option::Option<
//     unsafe extern "C" fn(
//         context: *mut ::std::os::raw::c_void,
//         data: *mut ::std::os::raw::c_void,
//         size: i32,
//     ),
// >;

// extern "C" {
//     pub fn FNA3D_Image_SavePNG(
//         writeFunc: FNA3D_Image_WriteFunc,
//         context: *mut ::std::os::raw::c_void,
//         srcW: i32,
//         srcH: i32,
//         dstW: i32,
//         dstH: i32,
//         data: *mut u8,
//     );
// }

// extern "C" {
//     pub fn FNA3D_Image_SaveJPG(
//         writeFunc: FNA3D_Image_WriteFunc,
//         context: *mut ::std::os::raw::c_void,
//         srcW: i32,
//         srcH: i32,
//         dstW: i32,
//         dstH: i32,
//         data: *mut u8,
//         quality: i32,
//     );
// }
