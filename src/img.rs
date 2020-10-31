//! `FNA3D_Image.h` with some helpers
//!
//! Iternally, `FNA3D_Image` uses [`stb_image`] (`stbi`) with callback functions where arbitrary IO
//! is allowed.
//!
//! Other options than `fna3d::img` are raw [`stb_image`] (with or without callbacks), [`SDL_RWops`]
//! (SDL2) or else.
//!
//! [`stb_image`]: https://github.com/nothings/stb/blob/master/stb_image.h
//! [`SDL_RWops`]: https://wiki.libsdl.org/SDL_RWops
//!
//! # Example (presudo code)
//!
//! ```no_run
//! pub struct MyTexture2d {
//!     raw: *mut fna3d::Texture,
//!     w: u32,
//!     h: u32,
//! }
//!
//! impl MyTexture2d {
//!     pub fn from_path(
//!         device: &fna3d::Device,
//!         path: impl AsRef<std::path::Path>,
//!     ) -> Option<Self> {
//!         let (pixels_ptr, len, [w, h]) = fna3d::img::from_path(path, None);
//!
//!         if pixels_ptr == std::ptr::null_mut() {
//!             return None;
//!         }
//!
//!         let pixels: &[u8] = unsafe { std::slice::from_raw_parts(pixels_ptr, len as usize) };
//!         let raw = device.create_texture_2d(
//!             fna3d::SurfaceFormat::Color,
//!             w,
//!             h,
//!             0,
//!             false,
//!         );
//!         let texture = MyTexture2d { raw, w, h };
//!
//!         fna3d::img::free(pixels_ptr as *mut _);
//!
//!         Some(texture)
//!     }
//! }
//! ```

use ::{
    fna3d_sys as sys,
    std::{
        fs::File,
        io::{self, BufReader, Read, Seek, SeekFrom, Write},
        os::raw::{c_char, c_void},
        path::Path,
    },
};

use crate::Texture;

/// Callback used to pull data from the stream
type ReadFunc = sys::FNA3D_Image_ReadFunc;

/// Callback used to seek around a stream
type SkipFunc = sys::FNA3D_Image_SkipFunc;

/// Callback used to check that we're reached to the end of a stream
type EofFunc = sys::FNA3D_Image_EOFFunc;

// /// Callback used to check that we're reached to the end of a stream
// type WriteFunc = sys::FNA3D_Image_WriteFunc;

/// Frees pixels loaded with a helper method in this module
pub fn free(mem: *const u8) {
    unsafe {
        sys::FNA3D_Image_Free(mem as *mut _);
    }
}

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data
///
/// Mainly for `include_bytes!`.
pub fn from_encoded_bytes(bytes: &[u8]) -> (*const u8, u32, [u32; 2]) {
    let reader = std::io::Cursor::new(bytes);
    self::from_reader(reader, None)
}

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data
///
/// Be sure to [`free`] the returned memory after use!
///
/// The return type is not [`Vec`] because it frees its content when dropping.
///
/// [`Vec`]: std::vec::Vec
pub fn from_path(
    path: impl AsRef<Path>,
    force_size: Option<[u32; 2]>,
) -> (*const u8, u32, [u32; 2]) {
    let path = path.as_ref();
    let reader = File::open(path)
        .ok()
        .unwrap_or_else(|| panic!("failed to open file {}", path.display()));
    let reader = BufReader::new(reader); // FIXME: is this good?
    self::from_reader(reader, force_size)
}

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data
///
/// Be sure to [`free`] the returned memory after use!
///
/// The return type is not [`Vec`] because it frees its content when dropping.
///
/// [`Vec`]: std::vec::Vec
pub fn from_reader<R: Read + Seek>(
    reader: R,
    force_size: Option<[u32; 2]>,
) -> (*const u8, u32, [u32; 2]) {
    let context = LoadContext {
        reader,
        is_end: false,
    };

    unsafe {
        self::load_impl(
            Some(LoadCallbacks::<R>::read),
            Some(LoadCallbacks::<R>::skip),
            Some(LoadCallbacks::<R>::eof),
            std::mem::transmute(&context),
            force_size,
        )
    }
}

/// Encodes RGBA8 image data into PNG data with a writer
pub fn save_png<T: Write>(
    writer: T,
    data: *mut Texture,
    src_w: u32,
    src_h: u32,
    dst_w: u32,
    dst_h: u32,
) {
    let mut cx = SaveContext { writer };

    unsafe {
        fna3d_sys::FNA3D_Image_SavePNG(
            Some(SaveContext::<T>::write),
            &mut cx as *mut _ as _,
            src_w as i32,
            src_h as i32,
            dst_w as i32,
            dst_h as i32,
            data as *mut u8,
        );
    }
}

/// Encodes RGBA8 image data into PNG data to some path
pub fn save_png_to(
    path: impl AsRef<Path>,
    data: *mut u8,
    src_w: u32,
    src_h: u32,
    dst_w: u32,
    dst_h: u32,
) -> io::Result<()> {
    let file = File::create(path)?;
    let mut cx = SaveContext { writer: file };

    unsafe {
        fna3d_sys::FNA3D_Image_SavePNG(
            Some(SaveContext::<File>::write),
            &mut cx as *mut _ as _,
            src_w as i32,
            src_h as i32,
            dst_w as i32,
            dst_h as i32,
            data,
        );
    }

    Ok(())
}

struct SaveContext<T: Write> {
    writer: T,
}

impl<T: Write> SaveContext<T> {
    unsafe extern "C" fn write(
        context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        size: i32,
    ) {
        let cx = &mut *(context as *mut Self);
        let buf = std::slice::from_raw_parts(data as *mut u8, size as usize);
        // TODO: don't unwrap
        cx.writer.write(buf).unwrap();
    }
}

// --------------------------------------------------------------------------------
// Internal implementation

/// Context passed around callback functions
struct LoadContext<R: Read + Seek> {
    reader: R,
    is_end: bool, // FIXME: is this right?
}

/// Callback functions for `FNA3D_Image.h`, i.e. `stb_image.h`
struct LoadCallbacks<R: Read + Seek> {
    phantom: std::marker::PhantomData<R>,
}

/// Tries to fill the buffer as much as possible. `Ok` if we read any byte
///
/// We need this helper because both [`read`] and [`read_exact`] don't match our needs. [`read`]
/// does not always try to read as much as possible, while `read_exact` returns error when it is
/// not able to fullfill the given buffer.
///
/// The implementation is based on [`read_exact`].
///
/// [`read`]: std::io::Read::read
/// [`read_exact`]: std::io::Read::read_exact
fn read_as_much(mut reader: impl Read, mut buf: &mut [u8]) -> io::Result<usize> {
    let mut read = 0;
    while !buf.is_empty() {
        match reader.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
                read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(read)
}

impl<R: Read + Seek> LoadCallbacks<R> {
    /// Reads up to `size` bytes
    unsafe extern "C" fn read(context: *mut c_void, out_ptr: *mut c_char, size: i32) -> i32 {
        let cx = &mut *(context as *mut LoadContext<R>);

        let out = std::slice::from_raw_parts_mut(out_ptr as *mut u8, size as usize);
        let len_read = self::read_as_much(&mut cx.reader, out).unwrap();

        len_read as i32
    }

    /// Skips `n` bytes
    unsafe extern "C" fn skip(context: *mut c_void, n: i32) {
        // log::warn!("FNA3D_Image stbi skipFunc called: {}", n);
        let cx = &mut *(context as *mut LoadContext<R>);
        cx.reader
            .seek(SeekFrom::Current(n as i64))
            .unwrap_or_else(|err| panic!("error in anf skip func {}", err));
    }

    /// FIXME: is this OK? I've never seen it's called and I'm really not confident
    unsafe extern "C" fn eof(context: *mut c_void) -> i32 {
        let cx = &mut *(context as *mut LoadContext<R>);
        log::warn!("FNA3D_Image stbi eofFunc called: is_end={}", cx.is_end);
        cx.is_end as i32
    }
}

/// Decodes PNG/JPG/GIF data into raw RGBA8 texture data.
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
unsafe fn load_impl(
    read_fn: ReadFunc,
    skip_fn: SkipFunc,
    eof_fn: EofFunc,
    context: *mut c_void,
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
