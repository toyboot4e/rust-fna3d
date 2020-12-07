//! Font book on font stash

// FIXME: all

pub use fontstash::{self, FontStash};

use {
    fontstash::FonsTextIter,
    std::os::raw::{c_int, c_uchar, c_void},
};

/// The shared ownership of [`FontBookInternal`]
///
/// It is required to use the internal variable so that the memory position is fixed.
pub struct FontBook {
    /// Keeps memory position of the renderer
    inner: Box<FontBookInternal>,
}

impl std::ops::Deref for FontBook {
    type Target = FontBookInternal;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FontBook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FontBook {
    pub fn new(device: fna3d::Device, w: u32, h: u32) -> Self {
        let mut inner = Box::new(FontBookInternal {
            stash: FontStash::uninitialized(),
            device,
            texture: std::ptr::null_mut(),
            w,
            h,
            is_dirty: true,
        });

        let inner_ptr = inner.as_ref() as *const _ as *mut FontBookInternal;
        inner.stash.init_mut(w, h, inner_ptr);
        fontstash::set_error_callback(
            inner.stash().raw(),
            fons_error_callback,
            inner_ptr as *mut _,
        );

        return FontBook { inner };

        unsafe extern "C" fn fons_error_callback(
            _uptr: *mut c_void,
            error_code: c_int,
            _val: c_int,
        ) {
            match fontstash::ErrorCode::from_u32(error_code as u32) {
                Some(error) => {
                    log::warn!("fons error: {:?}", error);
                }
                None => {
                    log::warn!("fons error error: given broken erroor code");
                }
            }
        }
    }
}

/// The internals of [`FontBook`]
///
/// It is required to use the internal variable so that the memory position is fixed.
pub struct FontBookInternal {
    stash: fontstash::FontStash,
    device: fna3d::Device,
    /// The texture is always valid
    texture: *mut fna3d::Texture,
    /// The texture size is always synced with the fontstash size
    w: u32,
    /// The texture size is always synced with the fontstash size
    h: u32,
    /// Shall we update the texture data?
    is_dirty: bool,
}

impl Drop for FontBookInternal {
    fn drop(&mut self) {
        log::trace!("fontbook: drop");

        if !self.texture.is_null() {
            self.device.add_dispose_texture(self.texture);
        }
    }
}

/// Lifecycle
impl FontBookInternal {
    /// * TODO: render_update vs update
    pub fn update(&mut self) {
        self.is_dirty = true;
    }
}

/// Interface
impl FontBookInternal {
    pub fn texture(&self) -> *mut fna3d::Texture {
        self.texture
    }

    pub fn stash(&self) -> FontStash {
        self.stash.clone()
    }

    pub fn text_iter(&mut self, text: &str) -> fontstash::Result<FonsTextIter> {
        self.stash.text_iter(text)
    }
}

// --------------------------------------------------------------------------------
// Callback and texture updating

/// Renderer implementation
///
/// Return `1` to represent success.
unsafe impl fontstash::Renderer for FontBookInternal {
    /// Creates font texture
    unsafe extern "C" fn create(uptr: *mut c_void, width: c_int, height: c_int) -> c_int {
        log::trace!("fontbook: create [{}, {}]", width, height);

        let me = &mut *(uptr as *const _ as *mut Self);

        if !me.texture.is_null() {
            log::trace!("fontbook: create -- dispose old texture");
            me.device.add_dispose_texture(me.texture);
        }

        me.texture = me.device.create_texture_2d(
            fna3d::SurfaceFormat::Color,
            width as u32,
            height as u32,
            1,
            false,
        );
        me.w = width as u32;
        me.h = height as u32;

        me.is_dirty = true;

        true as c_int // success
    }

    unsafe extern "C" fn resize(uptr: *mut c_void, width: c_int, height: c_int) -> c_int {
        log::trace!("fontbook: resize");

        Self::create(uptr, width, height);
        true as c_int // success
    }

    /// Try to double the texture size while the atlas is full
    unsafe extern "C" fn expand(uptr: *mut c_void) -> c_int {
        log::trace!("fontbook: expand");

        let me = &mut *(uptr as *const _ as *mut Self);

        // Self::create(uptr, (me.w * 2) as i32, (me.h * 2) as i32);

        if let Err(why) = me.stash.expand_atlas(me.w * 2, me.h * 2) {
            log::warn!("fontstash: error on resize: {:?}", why);
            false as c_int // fail
        } else {
            true as c_int // success
        }
    }

    unsafe extern "C" fn update(
        uptr: *mut c_void,
        // TODO: what is the dirty rect
        _rect: *mut c_int,
        _data: *const c_uchar,
    ) -> c_int {
        let me = &mut *(uptr as *const _ as *mut Self);
        me.maybe_update_texture();
        true as c_int // success
    }
}

impl FontBookInternal {
    /// Updates GPU texure. Call it whenever drawing text
    fn maybe_update_texture(&mut self) {
        if !self.is_dirty {
            // TODO: this looks very odd but works
            self.is_dirty = true;
            return;
        }
        self.is_dirty = false;

        self.stash.with_pixels(|pixels, w, h| {
            let data = {
                log::trace!("fontbook: [{}, {}] update GPU texture", w, h);

                // FIXME: address boundary error
                let area = (w * h) as usize;
                // four channels (RGBA)
                let mut data = Vec::<u8>::with_capacity(4 * area);
                for i in 0..area {
                    data.push(255);
                    data.push(255);
                    data.push(255);
                    data.push(pixels[i]);
                }
                data
            };

            self.device
                .set_texture_data_2d(self.texture, 0, 0, w, h, 0, &data);

            log::trace!("<after upload>");
        });
    }
}
