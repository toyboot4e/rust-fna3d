//! Wrapper of `FNA3D_Device`

use std::{
    // this should be `std::ffi::c_void` but `bindgen` uses:
    os::raw::c_void,
    ptr,
};

use fna3d_sys as sys;

use crate::{
    fna3d::{fna3d_enums as enums, fna3d_structs::*},
    utils::AsVec4,
};
use enum_primitive::*;

// TODO: i32 vs u32 (vs usize) for indices or width/height
// TODO: option vs raw pointer
// TODO: actually some `&mut` are semantically `&self`
// TODO: memory managenemt and lifetimes

// --------------------------------------------------------------------------------
// Helpers

trait AsMutPtr<T> {
    fn as_mut_ptr(self) -> *mut T;
}

impl<'a, T> AsMutPtr<T> for Option<&'a mut T> {
    fn as_mut_ptr(self) -> *mut T {
        match self {
            Some(value) => value as *mut T,
            None => ptr::null_mut(),
        }
    }
}

// --------------------------------------------------------------------------------
// Device

/// Used to implement `GraphicsDevice`
///
/// # Note
///
/// - Use disposing functions for:
///     - `Buffer`
///     - `Renderbuffer`
///     - `Effect`
///     - `Query`
///     - `Texture`
pub struct Device {
    raw: *mut sys::FNA3D_Device,
}

// TODO: is this correct?
impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            sys::FNA3D_DestroyDevice(self.raw);
        };
    }
}

impl Device {
    pub fn raw(&self) -> *mut sys::FNA3D_Device {
        self.raw
    }

    /// Creates a rendering context for use on the calling thread.
    ///
    /// * `params`:
    /// The initial device/backbuffer settings.
    /// * `do_debug`:
    /// Enable debugging and backend validation features at the cost of reduced overall
    /// performance.
    ///
    /// Returns a device ready for use. Be sure to only call device functions from
    /// the thread that it was created on!
    pub fn from_params(params: &mut PresentationParameters, do_debug: bool) -> Self {
        Self {
            // debug mode
            raw: unsafe { sys::FNA3D_CreateDevice(params, do_debug as u8) },
        }
    }
}

/// Begin/end frame
/// ---
impl Device {
    /// The first thing you call when rendering a frame
    pub fn begin_frame(&mut self) {
        unsafe {
            sys::FNA3D_BeginFrame(self.raw);
        }
    }

    /// Presents the backbuffer to the window.
    ///
    /// * `src`: The region of the buffer to present (or None).
    /// * `dest`: The region of the window to update (or None).
    /// * `override_window_handle`: The OS window handle (not really "overridden").
    pub fn swap_buffers(
        &mut self,
        // TODO: different function name for (None, None)?
        mut src: Option<Rect>,
        mut dest: Option<Rect>,
        // TODO: wrap it
        override_window_handle: *mut c_void,
    ) {
        let src = src.as_mut().as_mut_ptr();
        let dest = dest.as_mut().as_mut_ptr();
        unsafe {
            sys::FNA3D_SwapBuffers(self.raw, src, dest, override_window_handle);
        }
    }
}

/// Draw
/// ---
impl Device {
    /// Clears the active draw buffers of any previous contents.
    ///
    /// * `options`:
    ///   Bitflags to specify color/depth/stencil buffers for clearing.
    /// * `color`:
    ///   The new value of the cleared color buffer.
    /// * `depth`:
    ///   The new value of the cleared depth buffer.
    /// * `stencil`:
    ///   The new value of the cleared stencil buffer.
    pub fn clear(&mut self, options: enums::ClearOptions, color: Color, depth: f32, stencil: i32) {
        unsafe {
            sys::FNA3D_Clear(
                self.raw,
                options.bits(),
                &mut color.as_vec4() as *mut _,
                depth,
                stencil,
            );
        }
    }

    /// Draws data from vertex/index buffers.
    ///
    /// * `prim`:
    ///   The primitive topology of the vertex data.
    /// * `base_vertex`:
    ///   The starting offset to read from the vertex buffer.
    /// * `min_vertex_index`:
    ///   The lowest index value expected from the index buffer.
    /// * `num_vertices`:
    ///   The highest offset expected from the index buffer.
    /// * `start_index`:
    ///   The starting offset to read from the index buffer.
    /// * `prim_count`:
    ///   The number of primitives to draw.
    /// * `indices`:
    ///   The index buffer to bind for this draw call.
    /// * `idx_elem_size`:
    ///   The size of the index type for this index buffer.
    pub fn draw_indexed_primitives(
        &mut self,
        prim: enums::PrimitiveType,
        base_vertex: i32,
        min_vertex_index: i32,
        num_vertices: i32,
        start_index: i32,
        prim_count: i32,
        indices: &mut Buffer,
        idx_elem_size: enums::IndexElementSize,
    ) {
        unsafe {
            sys::FNA3D_DrawIndexedPrimitives(
                self.raw,
                prim as sys::FNA3D_PrimitiveType,
                base_vertex,
                min_vertex_index,
                num_vertices,
                start_index,
                prim_count,
                indices,
                idx_elem_size as u32,
            );
        }
    }

    /// Draws data from vertex/index buffers with instancing enabled.
    ///
    /// * `instance_count`:
    ///   The number of instances that will be drawn.
    pub fn draw_instanced_primitives(
        &mut self,
        prim: enums::PrimitiveType,
        base_vertex: i32,
        min_vertex_index: i32,
        num_vertices: i32,
        start_index: i32,
        prim_count: i32,
        instance_count: i32,
        indices: &mut Buffer,
        idx_elem_size: enums::IndexElementSize,
    ) {
        unsafe {
            sys::FNA3D_DrawInstancedPrimitives(
                self.raw,
                prim as sys::FNA3D_PrimitiveType,
                base_vertex,
                min_vertex_index,
                num_vertices,
                start_index,
                prim_count,
                instance_count,
                indices,
                idx_elem_size as u32,
            );
        }
    }

    /// Draws data from vertex buffers.
    ///
    /// * `prim`:
    ///   The primitive topology of the vertex data.
    /// * `vertexStart`:
    ///   The starting offset to read from the vertex buffer.
    /// * `primitiveCount`:
    ///   The number of primitives to draw.
    pub fn draw_primitives(
        &mut self,
        prim: enums::PrimitiveType,
        vertex_start: u32,
        prim_count: u32,
    ) {
        let vertex_start = vertex_start as i32;
        let prim_count = prim_count as i32;
        unsafe {
            sys::FNA3D_DrawPrimitives(self.raw, prim as u32, vertex_start, prim_count);
        }
    }
}

/// Mutable render states
/// ---
impl Device {
    /// Sets the view dimensions for rendering, relative to the active render target.
    /// It is required to call this at least once after calling `set_render_targets`, as
    /// the renderer may need to adjust these dimensions to fit the backend's
    /// potentially goofy coordinate systems.
    ///
    /// * `viewport`: The new view dimensions for future draw calls.
    pub fn set_viewport(&mut self, viewport: &mut Viewport) {
        unsafe {
            sys::FNA3D_SetViewport(self.raw, viewport);
        }
    }

    /// Sets the scissor box for rendering, relative to the active render target.
    /// It is required to call this at least once after calling `set_render_targets`, as
    /// the renderer may need to adjust these dimensions to fit the backend's
    /// potentially goofy coordinate systems.
    ///
    /// * `scissor`: The new scissor box for future draw calls.
    pub fn set_scissor_rect(&mut self, mut scissor: Option<Rect>) {
        unsafe {
            sys::FNA3D_SetScissorRect(self.raw, scissor.as_mut().as_mut_ptr());
        }
    }

    /// Gets the blending factor used for current draw calls.
    ///
    /// * `blend_factor`: Filled with color being used as the device blend factor.
    pub fn get_blend_factor(&mut self, mut blend_factor: Color) {
        unsafe {
            sys::FNA3D_GetBlendFactor(self.raw, &mut blend_factor as *mut _);
        }
    }

    /// Sets the blending factor used for future draw calls.
    ///
    /// * `blend_factor`: The color to use as the device blend factor.
    pub fn set_blend_factor(&mut self, mut blend_factor: Color) {
        unsafe {
            sys::FNA3D_SetBlendFactor(self.raw, &mut blend_factor as *mut _);
        }
    }

    /// Gets the mask from which multisample fragment data is sampled from.
    ///
    /// Returns the coverage mask used to determine sample locations.
    pub fn get_multi_sample_mask(&self) -> i32 {
        unsafe { sys::FNA3D_GetMultiSampleMask(self.raw) }
    }

    /// Sets the reference value used for certain types of stencil testing.
    ///
    /// * `ref`: The new stencil reference value.
    pub fn set_multi_sample_mask(&mut self, mask: i32) {
        unsafe {
            sys::FNA3D_SetMultiSampleMask(self.raw, mask);
        }
    }

    /// Gets the reference value used for certain types of stencil testing.
    ///
    /// Returns the stencil reference value.
    pub fn get_reference_stencil(&self) -> i32 {
        unsafe { sys::FNA3D_GetReferenceStencil(self.raw) }
    }

    /// Sets the reference value used for certain types of stencil testing.
    ///
    /// * `ref`: The new stencil reference value.
    pub fn set_reference_stencil(&mut self, ref_: i32) {
        unsafe {
            sys::FNA3D_SetReferenceStencil(self.raw, ref_);
        }
    }
}

/// Immutable render states
/// ---
impl Device {
    /// Applies a blending state to use for future draw calls. This only needs to be
    /// called when the state actually changes. Redundant calls may negatively affect
    /// performance!
    ///
    /// * `blend_state`: The new parameters to use for color blending.
    pub fn set_blend_state(&mut self, blend_state: &mut BlendState) {
        unsafe {
            sys::FNA3D_SetBlendState(self.raw, blend_state.raw() as *mut _);
        }
    }

    /// Applies depth/stencil states to use for future draw calls. This only needs to
    /// be called when the states actually change. Redundant calls may negatively
    /// affect performance!
    ///
    /// * `depth_stencil_state`: The new parameters to use for depth/stencil work.
    pub fn set_depth_stencil_state(&mut self, depth_stencil_state: &mut DepthStencilState) {
        unsafe {
            sys::FNA3D_SetDepthStencilState(self.raw, depth_stencil_state.raw() as *mut _);
        }
    }

    /// Applies the rasterizing state to use for future draw calls.
    /// It's generally a good idea to call this for each draw call, but if you really
    /// wanted to you could try reducing it to when the state changes and when the
    /// render target state changes.
    ///
    /// * `rasterizer_state`: The new parameters to use for rasterization work.
    pub fn apply_rasterizer_state(&mut self, rst: &mut RasterizerState) {
        unsafe {
            sys::FNA3D_ApplyRasterizerState(self.raw, rst.raw() as *mut _);
        }
    }

    /// Updates a sampler slot with new texture/sampler data for future draw calls.
    /// This should only be called on slots that have modified texture/sampler state.
    /// Redundant calls may negatively affect performance!
    ///
    /// * `index`:
    ///   The sampler slot to update.
    /// * `texture`:
    ///   The texture bound to this sampler.
    /// * `sampler`:
    ///   The new parameters to use for this slot's texture sampling.
    pub fn verify_sampler(
        &mut self,
        index: i32,
        // TODO: unnecessary mutable references
        texture: *mut Texture,
        sampler: &mut SamplerState,
    ) {
        // let texture = match texture {
        //     Some(t) => t as *mut _,
        //     None => ptr::null_mut(),
        // };
        unsafe {
            sys::FNA3D_VerifySampler(self.raw, index, texture, sampler.raw_mut() as *mut _);
        }
    }

    /// Updates a vertex sampler slot with new texture/sampler data for future draw
    /// calls. This should only be called on slots that have modified texture/sampler
    /// state. Redundant calls may negatively affect performance!
    ///
    /// * `index`:	The vertex sampler slot to update.
    /// * `texture`:	The texture bound to this sampler.
    /// * `sampler`:	The new parameters to use for this slot's texture sampling.
    pub fn verify_vertex_sampler(
        &mut self,
        index: i32,
        texture: &mut Texture,
        sampler: &mut SamplerState,
    ) {
        unsafe {
            sys::FNA3D_VerifyVertexSampler(self.raw, index, texture, sampler.raw_mut() as *mut _);
        }
    }

    /// Updates the vertex attribute state to read from a set of vertex buffers. This
    /// should be the very last thing you call before making a draw call, as this
    /// does all the final prep work for the shader program before it's ready to use.
    ///
    /// * `bindings`:
    ///   The vertex buffers and their attribute data.
    /// * `num_bindings`:
    ///   The number of elements in the bindings array.
    /// * `bindings_updated`:
    ///   If the bindings array hasn't changed since the last update, this can be false. We'll only
    ///   update the shader state, updating vertex attribute data only if we 100% have to, for a
    ///   tiny performance improvement.
    /// * `base_vertex`:
    ///   This should be the same as the `baseVertex` parameter from your Draw*Primitives call, if
    ///   applicable. Not every rendering backend has native base vertex support, so we work
    ///   around it by passing this here.
    pub fn apply_vertex_buffer_bindings(
        &mut self,
        bindings: &mut VertexBufferBinding,
        num_bindings: usize,
        bindings_updated: bool,
        base_vertex: usize,
    ) {
        unsafe {
            sys::FNA3D_ApplyVertexBufferBindings(
                self.raw,
                bindings,
                num_bindings as i32,
                bindings_updated as u8,
                base_vertex as i32,
            );
        }
    }
}

/// Render targets
/// ---
impl Device {
    /// Sets the color/depth/stencil buffers to write future draw calls to.
    ///
    /// * `render_targets`:
    ///    The targets to write to, or `None` for the backbuffer.
    /// * `num_render_targets`:
    ///    The size of the renderTargets array (can be 0).
    /// * `depth_stencil_buffer`:
    ///    The depth/stencil renderbuffer (can be `None`).
    /// * `depth_format`:
    ///    The format of the depth/stencil renderbuffer.
    pub fn set_render_targets(
        &mut self,
        render_targets: Option<&mut RenderTargetBinding>,
        num_render_targets: u32,
        depth_stencil_buffer: Option<&mut Renderbuffer>,
        depth_format: enums::DepthFormat,
    ) {
        unsafe {
            sys::FNA3D_SetRenderTargets(
                self.raw,
                render_targets.as_mut_ptr(),
                num_render_targets as i32,
                depth_stencil_buffer.as_mut_ptr(),
                depth_format as u32,
            );
        }
    }

    /// After unsetting a render target, call this to resolve multisample targets or
    /// generate mipmap data for the final texture.
    ///
    /// * `target`: The render target to resolve once rendering is complete.
    pub fn resolve_target(&mut self, target: &mut RenderTargetBinding) {
        unsafe {
            sys::FNA3D_ResolveTarget(self.raw, target);
        }
    }

    /// After modifying the OS window state, call this to reset the backbuffer to
    /// match your window changes.
    ///
    /// * `params`: The new settings for the backbuffer.
    pub fn reset_backbuffer(&mut self, params: &mut PresentationParameters) {
        unsafe {
            sys::FNA3D_ResetBackbuffer(self.raw, params as *mut _);
        }
    }

    ///  Read the backbuffer's contents directly into client memory. This function is
    ///  basically one giant CPU/GPU sync point, do NOT ever call this during any
    ///  performance-critical situation! Just use it for screenshots.
    ///
    /// * `x`:		The x offset of the backbuffer region to read.
    /// * `y`:		The y offset of the backbuffer region to read.
    /// * `w`:		The width of the backbuffer region to read.
    /// * `h`:		The height of the backbuffer region to read.
    /// * `data`:	The pointer to read the backbuffer data into.
    /// * `data_len`:	The size of the backbuffer data in bytes.
    pub fn read_backbuffer(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        // TODO: what is data??
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_ReadBackbuffer(self.raw, x, y, w, h, data, data_len);
        }
    }

    /// Gets the current dimensions of the backbuffer.
    ///
    /// * `w`:	Filled with the backbuffer's width.
    /// * `h`:	Filled with the backbuffer's height.
    pub fn get_backbuffer_size(&mut self) -> (i32, i32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            sys::FNA3D_GetBackbufferSize(self.raw, &mut w, &mut h);
        }
        (w, h)
    }

    /// Gets the current pixel format of the backbuffer.
    ///
    /// Returns the backbuffer's pixel format.
    pub fn get_backbuffer_surface_format(&self) -> enums::SurfaceFormat {
        let prim = unsafe { sys::FNA3D_GetBackbufferSurfaceFormat(self.raw) };
        // FIXME: is it ok to unwrap??
        enums::SurfaceFormat::from_u32(prim).unwrap()
    }

    /// Gets the format of the backbuffer's depth/stencil buffer.
    ///
    /// Returns the backbuffer's depth/stencil format.
    pub fn get_backbuffer_depth_format(&self) -> enums::DepthFormat {
        let prim = unsafe { sys::FNA3D_GetBackbufferDepthFormat(self.raw) };
        // FIXME: is it ok to unwrap??
        enums::DepthFormat::from_u32(prim).unwrap()
    }

    /// Gets the multisample sample count of the backbuffer.
    ///
    /// Returns the backbuffer's multisample sample count.
    pub fn get_backbuffer_multi_sample_count(&self) -> i32 {
        unsafe { sys::FNA3D_GetBackbufferMultiSampleCount(self.raw) }
    }
}

/// Textures
/// ---
impl Device {
    /// Creates a 2D texture to be applied to verify_sampler.
    ///
    /// * `fmt`:
    ///   The pixel format of the texture data.
    /// * `w`:
    ///   The width of the texture image.
    /// * `h`:
    ///   The height of the texture image.
    /// * `level_count`:
    ///   The number of mipmap levels to allocate.
    /// * `is_render_target`:
    ///   Set this to 1 when using this with `set_render_targets`.
    ///
    /// Returns an allocated `FNA3D_Texture*` object. Note that the contents of the
    /// texture are undefined, so you must call `SetData` at least once before drawing!
    pub fn create_texture_2d(
        &mut self,
        fmt: enums::SurfaceFormat,
        w: u32,
        h: u32,
        level_count: u32,
        is_render_target: bool,
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTexture2D(
                self.raw,
                fmt as u32,
                w as i32,
                h as i32,
                level_count as i32,
                is_render_target as u8,
            )
        }
    }

    /// Creates a 3D texture to be applied to `verify_sampler`.
    ///
    /// * `fmt`:
    ///   The pixel format of the texture data.
    /// * `width`:
    ///   The width of the texture image.
    /// * `height`:
    ///   The height of the texture image.
    /// * `depth`:
    ///   The depth of the texture image.
    /// * `level_count`:
    ///   The number of mipmap levels to allocate.
    ///
    /// Returns an allocated FNA3D_Texture* object. Note that the contents of the
    /// texture are undefined, so you must call `SetData` at least once before drawing!
    pub fn create_texture_3d(
        &mut self,
        fmt: enums::SurfaceFormat,
        width: i32,
        height: i32,
        depth: i32,
        level_count: i32,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTexture3D(self.raw, fmt as u32, width, height, depth, level_count)
        }
    }

    /// Creates a texture cube to be applied to `verify_sampler`.
    ///
    /// * `fmt`:
    ///   The pixel format of the texture data.
    /// * `size`:
    ///   The length of a single edge of the texture cube.
    /// * `level_count`:
    ///   The number of mipmap levels to allocate.
    /// * `is_render_target`:
    ///   Set this to 1 when using this with `set_render_targets`.
    ///
    /// Returns an allocated FNA3D_Texture* object. Note that the contents of the
    ///  texture are undefined, so you must call `SetData` at least once before drawing!
    pub fn create_texture_cube(
        &mut self,
        fmt: enums::SurfaceFormat,
        size: i32,
        level_count: i32,
        is_render_target: bool,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTextureCube(
                self.raw,
                fmt as u32,
                size,
                level_count,
                is_render_target as u8,
            )
        }
    }

    /// Sends a texture to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `texture`: The FNA3D_Texture to be destroyed.
    pub fn add_dispose_texture(&mut self, texture: &mut Texture) {
        unsafe {
            sys::FNA3D_AddDisposeTexture(self.raw, texture);
        }
    }

    /// Uploads image data to a 2D texture object.
    ///
    /// * `texture`:
    ///   The texture to be updated.
    /// * `fmt`:
    ///   Should match the format provided to `create_texture_2D`.
    /// * `x`:
    ///   The x offset of the subregion being updated.
    /// * `y`:
    ///   The y offset of the subregion being updated.
    /// * `w`:
    ///   The width of the subregion being updated.
    /// * `h`:
    ///   The height of the subregion being updated.
    /// * `level`:
    ///   The mipmap level being updated.
    /// * `data`:
    ///    A pointer to the image data.
    /// * `data_len`:
    ///   The size of the image data in bytes.
    pub fn set_texture_data_2d(
        &mut self,
        texture: *mut Texture,
        fmt: enums::SurfaceFormat,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        level: u32,
        data: *mut c_void,
        data_len: u32,
    ) {
        unsafe {
            sys::FNA3D_SetTextureData2D(
                self.raw,
                texture,
                fmt as u32,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                level as i32,
                data,
                data_len as i32,
            );
        }
    }

    /// Uploads image data to a 3D texture object.
    ///
    /// * `texture`:
    ///   The texture to be updated.
    /// * `fmt`:
    ///   Should match the format provided to CreateTexture3D.
    /// * `x`:
    ///   The x offset of the subregion being updated.
    /// * `y`:
    ///   The y offset of the subregion being updated.
    /// * `z`:
    ///   The z offset of the subregion being updated.
    /// * `w`:
    ///   The width of the subregion being updated.
    /// * `h`:
    ///   The height of the subregion being updated.
    /// * `d`:
    ///   The depth of the subregion being updated.
    /// * `level`:
    ///   The mipmap level being updated.
    /// * `data`:
    ///   A pointer to the image data.
    /// * `data_len`:
    ///   The size of the image data in bytes.
    pub fn set_texture_data_3d(
        &mut self,
        texture: &mut Texture,
        fmt: enums::SurfaceFormat,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
        h: i32,
        d: i32,
        level: i32,
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_SetTextureData3D(
                self.raw, texture, fmt as u32, x, y, z, w, h, d, level, data, data_len,
            );
        }
    }

    /// Uploads image data to a single face of a texture cube object.
    ///
    /// * `texture`:
    ///   The texture to be updated.
    /// * `fmt`:
    ///   Should match the format provided to CreateTextureCube.
    /// * `x`:
    ///   The x offset of the subregion being updated.
    /// * `y`:
    ///   The y offset of the subregion being updated.
    /// * `w`:
    ///   The width of the subregion being updated.
    /// * `h`:
    ///   The height of the subregion being updated.
    /// * `cube_map_face`:
    ///   The face of the cube being updated.
    /// * `level`:
    ///   The mipmap level being updated.
    /// * `data`:
    ///   A pointer to the image data.
    /// * `data_len`:
    ///   The size of the image data in bytes.
    pub fn set_texture_data_cube(
        &mut self,
        texture: &mut Texture,
        fmt: enums::SurfaceFormat,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        cube_map_face: enums::CubeMapFace,
        level: i32,
        data: *mut ::std::os::raw::c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_SetTextureDataCube(
                self.raw,
                texture,
                fmt as u32,
                x,
                y,
                w,
                h,
                cube_map_face as u32,
                level,
                data,
                data_len,
            );
        }
    }

    /// Uploads YUV image data to three ALPHA8 texture objects.
    ///
    /// * `y`:
    ///   The texture storing the Y data.
    /// * `u`:
    ///   The texture storing the U (Cb) data.
    /// * `v`:
    ///   The texture storing the V (Cr) data.
    /// * `y_width`:
    ///   The width of the Y plane.
    /// * `y_height`:
    ///   The height of the Y plane.
    /// * `uv_width`:
    ///   The width of the U/V planes.
    /// * `uv_height`:
    ///   The height of the U/V planes.
    /// * `data`:
    ///   A pointer to the raw YUV image data.
    /// * `data_len`:
    ///   The size of the image data in bytes.
    pub fn set_texture_data_yuv(
        &mut self,
        y: &mut Texture,
        u: &mut Texture,
        v: &mut Texture,
        y_width: i32,
        y_height: i32,
        uv_width: i32,
        uv_height: i32,
        data: *mut ::std::os::raw::c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_SetTextureDataYUV(
                self.raw, y, u, v, y_width, y_height, uv_width, uv_height, data, data_len,
            );
        }
    }

    //// Pulls image data from a 2D texture into client memory. Like any GetData,
    /// this is generally asking for a massive CPU/GPU sync point, don't call this
    /// unless there's absolutely no other way to use the image data!
    ///
    /// * `texture`:
    ///   The texture object being read.
    /// * `fmt`:
    ///   Should match the format provided to CreateTexture2D.
    /// * `x`:
    ///   The x offset of the subregion being read.
    /// * `y`:
    ///   The y offset of the subregion being read.
    /// * `w`:
    ///   The width of the subregion being read.
    /// * `h`:
    ///   The height of the subregion being read.
    /// * `level`:
    ///   The mipmap level being read.
    /// * `data`:
    ///   The pointer being filled with the image data.
    /// * `data_len`:
    ///   The size of the image data in bytes.
    pub fn get_texture_data_2d(
        &mut self,
        texture: &mut Texture,
        fmt: enums::SurfaceFormat,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        level: i32,
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_GetTextureData2D(
                self.raw, texture, fmt as u32, x, y, w, h, level, data, data_len,
            );
        }
    }

    /// Pulls image data from a 3D texture into client memory. Like any GetData,
    /// this is generally asking for a massive CPU/GPU sync point, don't call this
    /// unless there's absolutely no other way to use the image data!
    ///
    /// * `texture`:	The texture object being read.
    /// * `fmt`:	Should match the format provided to CreateTexture3D.
    /// * `x`:		The x offset of the subregion being read.
    /// * `y`:		The y offset of the subregion being read.
    /// * `z`:		The z offset of the subregion being read.
    /// * `w`:		The width of the subregion being read.
    /// * `h`:		The height of the subregion being read.
    /// * `d`:		The depth of the subregion being read.
    /// * `level`:	The mipmap level being read.
    /// * `data`:	The pointer being filled with the image data.
    /// * `data_len`:	The size of the image data in bytes.
    pub fn get_texture_data_3d(
        &mut self,
        texture: &mut Texture,
        fmt: enums::SurfaceFormat,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
        h: i32,
        d: i32,
        level: i32,
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_GetTextureData3D(
                self.raw, texture, fmt as u32, x, y, z, w, h, d, level, data, data_len,
            );
        }
    }

    /// Pulls image data from a single face of a texture cube object into client
    /// memory. Like any GetData, this is generally asking for a massive CPU/GPU sync
    /// point, don't call this unless there's absolutely no other way to use the
    /// image data!
    ///
    /// * `texture`:	The texture object being read.
    /// * `fmt`:	Should match the format provided to CreateTextureCube.
    /// * `x`:		The x offset of the subregion being read.
    /// * `y`:		The y offset of the subregion being read.
    /// * `w`:		The width of the subregion being read.
    /// * `h`:		The height of the subregion being read.
    /// * `cubeMapFace`:	The face of the cube being read.
    /// * `level`:	The mipmap level being read.
    /// * `data`:	The pointer being filled with the image data.
    /// * `dataLength`:	The size of the image data in bytes.
    pub fn get_texture_data_cube(
        &mut self,
        texture: *mut Texture,
        fmt: enums::SurfaceFormat,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        cube_map_face: enums::CubeMapFace,
        level: i32,
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_GetTextureDataCube(
                self.raw,
                texture,
                fmt as u32,
                x,
                y,
                w,
                h,
                cube_map_face as u32,
                level,
                data,
                data_len,
            );
        }
    }
}

/// Renderbuffers
/// ---
impl Device {
    /// Creates a color buffer to be used by `set_render_targets` / `resolve_target`.
    ///
    /// * `width`:		The width of the color buffer.
    /// * `height`:		The height of the color buffer.
    /// * `fmt`:		The pixel format of the color buffer.
    /// * `multi_sample_count`:	The MSAA value for the color buffer.
    /// * `texture`:		The texture that this buffer will be resolving to.
    ///
    /// Returns a color FNA3D_Renderbuffer object.
    pub fn gen_color_renderbuffer(
        &mut self,
        width: i32,
        height: i32,
        fmt: enums::SurfaceFormat,
        multi_sample_count: i32,
        texture: &mut Texture,
    ) -> *mut Renderbuffer {
        unsafe {
            sys::FNA3D_GenColorRenderbuffer(
                self.raw,
                width,
                height,
                fmt as u32,
                multi_sample_count,
                texture,
            )
        }
    }

    /// Creates a depth/stencil buffer to be used by `set_render_targets`.
    ///
    /// * `width`:		The width of the depth/stencil buffer.
    /// * `height`:		The height of the depth/stencil buffer.
    /// * `fmt`:		The storage format of the depth/stencil buffer.
    /// * `multi_sample_count`:	The MSAA value for the depth/stencil buffer.
    ///
    /// Returns a depth/stencil FNA3D_Renderbuffer object.
    pub fn gen_depth_stencil_renderbuffer(
        &mut self,
        width: i32,
        height: i32,
        fmt: enums::DepthFormat,
        multi_sample_count: i32,
    ) -> *mut Renderbuffer {
        unsafe {
            sys::FNA3D_GenDepthStencilRenderbuffer(
                self.raw,
                width,
                height,
                fmt as u32,
                multi_sample_count,
            )
        }
    }

    /// Sends a renderbuffer to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `renderbuffer`: The FNA3D_Renderbuffer to be destroyed.
    pub fn add_dispose_renderbuffer(&mut self, renderbuffer: &mut Renderbuffer) {
        unsafe {
            sys::FNA3D_AddDisposeRenderbuffer(self.raw, renderbuffer);
        }
    }

    /// Creates a vertex buffer to be used by Draw*Primitives.
    ///
    /// * `dynamic`:
    ///   Set to 1 if this buffer will be updated frequently.
    /// * `usage`:
    ///   Set to WRITEONLY if you do not intend to call GetData.
    /// * `size_in_bytes`:
    ///   The length of the vertex buffer.
    ///
    /// Returns an allocated FNA3D_Buffer* object. Note that the contents of the
    /// buffer are undefined, so you must call `SetData` at least once before drawing!
    pub fn gen_vertex_buffer(
        &mut self,
        is_dynamic: bool,
        usage: enums::BufferUsage,
        size_in_bytes: u32,
    ) -> *mut Buffer {
        unsafe {
            sys::FNA3D_GenVertexBuffer(
                self.raw,
                is_dynamic as u8,
                usage as u32,
                size_in_bytes as i32,
            )
        }
    }

    /// Sends a vertex buffer to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `buffer`: The FNA3D_Buffer to be destroyed.
    pub fn add_dispose_vertex_buffer(&mut self, buffer: &mut Buffer) {
        unsafe {
            sys::FNA3D_AddDisposeVertexBuffer(self.raw, buffer);
        }
    }

    // TODO: refine vertex buffer function API as index buffer functions
    /// Sets a region of the vertex buffer with client data.
    ///
    /// * `buffer`:
    ///   The vertex buffer to be updated.
    /// * `offsetInBytes`:
    ///   The starting offset of the buffer to write into.
    /// * `data`:
    ///   The client data to write into the buffer.
    /// * `elementSizeInBytes`:
    ///   The size of each element in the client buffer.
    /// * `options`:
    ///   Try not to call NONE if this is a dynamic buffer!
    pub fn set_vertex_buffer_data(
        &mut self,
        buffer: *mut Buffer,
        offset_in_bytes: u32,
        data: *mut c_void,
        // element_count: u32,
        elem_size_in_bytes: u32,
        // vertex_stride: u32,
        options: enums::SetDataOptions,
    ) {
        unsafe {
            sys::FNA3D_SetVertexBufferData(
                self.raw,
                buffer,
                offset_in_bytes as i32,
                data,
                elem_size_in_bytes as i32,
                1, // see `FNA3D.h` for details (XNA compatibility)
                1, // see `FNA3D.h` for details (XNA compatibility)
                options as u32,
            );
        }
    }

    /// Pulls data from a region of the vertex buffer into a client pointer.
    ///
    /// * `buffer`:
    ///   The vertex buffer to be read from.
    /// * `offset_in_bytes`:
    ///   The starting offset of the buffer to write into.
    /// * `data`:
    ///   The client data to write into from the buffer.
    /// * `elem_size_in_bytes`:
    ///   The size of each element in the client buffer.
    pub fn get_vertex_buffer_data(
        &mut self,
        buffer: &mut Buffer,
        offset_in_bytes: i32,
        data: *mut ::std::os::raw::c_void,
        // element_count: i32,
        elem_size_in_bytes: u32,
        // vertex_stride: i32,
    ) {
        unsafe {
            sys::FNA3D_GetVertexBufferData(
                self.raw,
                buffer,
                offset_in_bytes,
                data,
                // element_count,
                1,
                elem_size_in_bytes as i32,
                // vertex_stride,
                1,
            );
        }
    }
}

/// Index buffers
impl Device {
    /// Creates an index buffer to be used by Draw*Primitives.
    ///
    /// * `dynamic`:
    ///   Set to 1 if this buffer will be updated frequently.
    /// * `usage`:
    ///   Set to WRITEONLY if you do not intend to call GetData.
    /// * `size_in_bytes`:
    ///   The length of the vertex buffer.
    ///
    /// Returns an allocated FNA3D_Buffer* object. Note that the contents of the
    /// buffer are undefined, so you must call `SetData` at least once before drawing!
    pub fn gen_index_buffer(
        &mut self,
        is_dynamic: bool,
        usage: enums::BufferUsage,
        size_in_bytes: u32,
    ) -> *mut Buffer {
        unsafe {
            sys::FNA3D_GenIndexBuffer(
                self.raw,
                is_dynamic as u8,
                usage as u32,
                size_in_bytes as i32,
            )
        }
    }

    /// Sends an index buffer to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `buffer`: The FNA3D_Buffer to be destroyed.
    pub fn add_dispose_index_buffer(&mut self, buf: &mut Buffer) {
        unsafe {
            sys::FNA3D_AddDisposeIndexBuffer(self.raw, buf);
        }
    }

    /// Sets a region of the index buffer with client data.
    ///
    /// * `buf`:
    ///   The index buffer to be updated.
    /// * `buf_offset_in_bytes`:
    ///   The starting offset of the buffer to write into.
    /// * `data`:
    ///   The client data to write into the buffer.
    // * `data_len`:
    //   The size (in bytes) of the client data.
    /// * `opts`:
    ///   Try not to call NONE if this is a dynamic buffer!
    pub fn set_index_buffer_data<T>(
        &mut self,
        buf: *mut Buffer,
        buf_offset_in_bytes: u32,
        data: &[T],
        // data: *mut c_void,
        // data_len: i32,
        opts: enums::SetDataOptions,
    ) {
        let data_len_in_bytes = data.len() * std::mem::size_of::<T>();
        unsafe {
            sys::FNA3D_SetIndexBufferData(
                self.raw,
                buf,
                buf_offset_in_bytes as i32,
                data.as_ptr() as *mut _,
                data_len_in_bytes as i32,
                opts as u32,
            );
        }
    }

    /// Pulls data from a region of the index buffer into a client pointer.
    ///
    /// * `buf`:
    ///   The index buffer to be read from.
    /// * `buf_offset_in_bytes`:
    ///   The starting offset of the buffer to read from.
    /// * `data`:
    ///   The pointer to read buffer data into.
    // * `data_len`:
    //   The size (in bytes) of the client data.
    pub fn get_index_buffer_data<T>(
        &mut self,
        buf: &mut Buffer,
        buf_offset_in_bytes: i32,
        data: &[T],
        // data: *mut c_void,
        // data_len: i32,
    ) {
        let len_in_bytes = data.len() * std::mem::size_of::<T>();
        unsafe {
            sys::FNA3D_GetIndexBufferData(
                self.raw,
                buf,
                buf_offset_in_bytes,
                data.as_ptr() as *mut _,
                len_in_bytes as i32,
            );
        }
    }
}

/// Effects
/// ---
impl Device {
    /// Parses and compiles a Direct3D 9 Effects Framework binary.
    ///
    /// * `effect_code`:
    ///   The D3D9 Effect binary blob.
    /// * `effect_code_length`:
    ///   The size (in bytes) of the blob.
    /// * `effect`:
    ///    Filled with the compiled FNA3D_Effect*.
    /// * `effect_data`:
    ///    Filled with the parsed Effect Framework data. This pointer is valid until
    ///    the effect is disposed.
    pub fn create_effect(
        &mut self,
        effect_code: *mut u8,
        effect_code_length: u32,
        // FIXME: I'm really not sure about tihs
        effect: *mut *mut Effect,
        effect_data: *mut *mut mojo::Effect,
    ) {
        unsafe {
            sys::FNA3D_CreateEffect(
                self.raw,
                effect_code,
                effect_code_length,
                effect,
                effect_data,
            );
        }
    }

    /// Copies a compiled Effect, including its current technique/parameter data.
    ///
    /// * `clone_source`:	The FNA3D_Effect to copy.
    /// * `effect`:	Filled with the new compiled FNA3D_Effect*.
    /// * `effect_data`:	Filled with the copied Effect Framework data.
    pub fn clone_effect(
        &mut self,
        clone_source: *mut Effect,
        effect: *mut *mut Effect,
        // FIXME: where sho
        effect_data: *mut *mut mojo::Effect,
    ) {
        unsafe {
            sys::FNA3D_CloneEffect(self.raw, clone_source, effect, effect_data);
        }
    }

    /// Sends an Effect to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `effect`: The FNA3D_Effect to be destroyed.
    pub fn add_dispose_effect(&mut self, effect: *mut Effect) {
        unsafe {
            sys::FNA3D_AddDisposeEffect(self.raw, effect);
        }
    }

    /// Sets the active technique on the Effect.
    ///
    /// * `effect`:	The Effect to be modified.
    /// * `technique`:	The technique to be used by future ApplyEffect calls.
    pub fn set_effect_technique(
        &mut self,
        effect: *mut Effect,
        technique: *mut mojo::EffectTechnique,
    ) {
        unsafe {
            sys::FNA3D_SetEffectTechnique(self.raw, effect, technique);
        }
    }

    /// Applies an effect pass from a given Effect, setting the active shader program
    /// and committing any parameter data changes to be used by future draw calls.
    ///
    /// * `effect`:		The Effect to be applied.
    /// * `pass`:		The current technique's pass index to be applied.
    /// * `state_changes`:	Structure to be filled with any render state changes
    ///			made by the Effect. This must be valid for the entire
    /// 			duration that this Effect is being applied.
    pub fn apply_effect(
        &mut self,
        effect: *mut Effect,
        pass: u32,
        state_changes: *mut mojo::EffectStateChanges,
    ) {
        unsafe {
            sys::FNA3D_ApplyEffect(self.raw, effect, pass, state_changes);
        }
    }

    /// Applies an effect pass from a given Effect, setting the active shader program
    /// and committing and parameter data changes to be used by future draw calls,
    /// while also caching the current program object to be stored once this Effect's
    /// pass has been completed.
    ///
    /// * `effect`:		The Effect to be applied.
    /// * `state_changes`:	Structure to be filled with any render state changes
    ///			made by the Effect. This must be valid for the entire
    /// 			duration that this Effect is being applied.
    pub fn begin_pass_restore(
        &mut self,
        effect: *mut Effect,
        state_changes: *mut mojo::EffectStateChanges,
    ) {
        unsafe {
            sys::FNA3D_BeginPassRestore(self.raw, effect, state_changes);
        }
    }

    /// Ends a pass started by BeginPassRestore, unsetting the current Effect and
    /// restoring the previous shader state from before BeginPassRestore was called.
    ///
    /// * `effect`: The Effect that was applied at BeginPassRestore.
    pub fn end_pass_restore(&mut self, effect: *mut Effect) {
        unsafe {
            sys::FNA3D_EndPassRestore(self.raw, effect);
        }
    }

    /// Creates an object used to run occlusion queries.
    ///
    /// Returns an FNA3D_Query object.
    pub fn create_query(&mut self) -> *mut Query {
        unsafe { sys::FNA3D_CreateQuery(self.raw) }
    }

    /// Sends a query object to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `query`: The FNA3D_Query to be destroyed.
    pub fn add_dispose_query(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_AddDisposeQuery(self.raw, query);
        }
    }

    /// Marks the start of when a query object should count pixels written.
    ///
    /// * `query`: The FNA3D_Query to start.
    pub fn query_begin(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_QueryBegin(self.raw, query);
        }
    }

    /// Marks the end of when a query object should count pixels written. Note that
    /// this does NOT mean the query has finished executing, you will need to poll
    /// QueryComplete before checking the pixel count.
    ///
    /// * `query`: The FNA3D_Query to stop.
    pub fn query_end(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_QueryEnd(self.raw, query);
        }
    }

    /// Call this until the function returns 1 to safely query for pixel counts.
    ///
    /// * `query`: The FNA3D_Query to sync with.
    ///
    /// Returns 1 when complete, 0 when still in execution.
    pub fn query_complete(&mut self, query: *mut Query) -> bool {
        unsafe { sys::FNA3D_QueryComplete(self.raw, query) != 0 }
    }

    /// Query the pixels counted between the begin/end markers set for the object.
    ///
    /// query: The FNA3D_Query to poll for pixel count
    ///
    /// Returns the pixels written during the begin/end period.
    pub fn query_pixel_count(&mut self, query: *mut Query) -> i32 {
        unsafe { sys::FNA3D_QueryPixelCount(self.raw, query) }
    }
}

/// Feature queries
/// ---
impl Device {
    /// Returns 1 if the renderer natively supports DXT1 texture data.
    pub fn supports_dxt1(&self) -> bool {
        unsafe { sys::FNA3D_SupportsDXT1(self.raw) != 0 }
    }

    /// Returns 1 if the renderer natively supports S3TC texture data.
    pub fn supports_s3_tc(&self) -> bool {
        unsafe { sys::FNA3D_SupportsS3TC(self.raw) != 0 }
    }

    /// Returns 1 if the renderer natively supports hardware instancing.
    pub fn supports_hardware_instancing(&self) -> bool {
        unsafe { sys::FNA3D_SupportsHardwareInstancing(self.raw) != 0 }
    }

    ///  Returns 1 if the renderer natively supports asynchronous buffer writing.
    pub fn supports_no_overwrite(&self) -> bool {
        unsafe { sys::FNA3D_SupportsNoOverwrite(self.raw) != 0 }
    }

    /// Returns the number of sampler slots supported by the renderer.
    pub fn get_max_texture_slots(
        &mut self,
        // FIXME: this..
    ) -> (usize, usize) {
        let (mut textures, mut vertex_textures): (i32, i32) = (0, 0);
        unsafe {
            sys::FNA3D_GetMaxTextureSlots(
                self.raw,
                &mut textures as *mut _,
                &mut vertex_textures as *mut _,
            );
        }
        (
            usize::from_i32(textures).unwrap(),
            usize::from_i32(vertex_textures).unwrap(),
        )
    }

    /// Returns the highest multisample count supported for anti-aliasing.
    ///
    /// * `fmt`:
    ///    The pixel format to query for MSAA support.
    /// * `multi_sample_count`:
    ///   The max MSAA value requested for this format.
    ///
    /// Returns a hardware-specific version of min(preferred, possible).
    pub fn get_max_multi_sample_count(
        &mut self,
        fmt: enums::SurfaceFormat,
        multi_sample_count: i32,
    ) -> i32 {
        unsafe { sys::FNA3D_GetMaxMultiSampleCount(self.raw, fmt as u32, multi_sample_count) }
    }
}

/// Debug
impl Device {
    /// Sets an arbitrary string constant to be stored in a rendering API trace,
    /// useful for labeling call streams for debugging purposes.
    ///
    /// * `text`: The string constant to mark in the API call stream.
    // FIXME: C string wrapper?? I have to read Rust nomicon
    pub fn set_string_marker(&mut self, text: *const ::std::os::raw::c_char) {
        unsafe {
            sys::FNA3D_SetStringMarker(self.raw, text);
        }
    }
}
