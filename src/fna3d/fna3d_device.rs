//! Wrapper of `FNA3D_Device`

use std::{
    // this should be `std::ffi::c_void` but `bindgen` uses:
    os::raw::c_void,
    ptr,
    rc::Rc,
};

use fna3d_sys::*;
use num_traits::FromPrimitive;

use crate::{
    fna3d::{fna3d_enums as enums, fna3d_structs::*},
    mojo,
};

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

/// The graphics device
///
/// # Functionalities
///
/// See the sidebar as list of methods.
///
/// * [Init/Quit](#initquit)
/// * [Presentation](#presentation)
/// * [Drawing](#drawing)
/// * [Mutable render states](#mutable-render-states)
/// * [Immutable render states](#immutable-render-states)
/// * [Render targets](#render-targets)
/// * [Textures](#textures)
/// * [Renderbuffers](#renderbuffers)
/// * [Vertex buffers](#vertex-buffers)
/// * [Index buffers](#index-buffers)
/// * [Effects](#effects)
/// * [Queries](#queris)
/// * [Feature queries](#feature-queries)
///
/// # Drop / dispose
///
/// `Device` destories the FNA3D device when it goes out of scope.
///
/// These types have to be disposed with corresponding methods in [`Device`]:
///
/// - [`Buffer`]
/// - [`Renderbuffer`]
/// - [`Effect`]
/// - [`Query`]
/// - [`Texture`]
///
/// If you'd like to automate disposing these resources with `Device` via `Drop`, you have to cheat
/// the borrow rules with pointers. This design might changes.
///
/// # Initialization
///
/// It's required to set viewport/rasterizer/blend state. **If this is skipped, we can't draw
/// anything** (we only can clear the screen):
///
/// * [`FNA3D_SetViewport`]
/// * [`FNA3D_ApplyRasterizerState`]
/// * [`FNA3D_SetBlendState`]
///
/// We also have to setup our shader used in the renderling pipeline. See [`crate::mojo`] for example.
///
/// # Rendering cycle
///
/// For each frame:
///
/// * [`FNA3D_Clear`]
/// * for each draw call:
///     * [`FNA3D_ApplyEffect`]
///     * [`FNA3D_SetVertexBufferData`]
///     * [`FNA3D_VerifySampler`]
///     * [`FNA3D_ApplyVertexBufferBindings`]
///     * [`FNA3D_DrawIndexedPrimitives`]
/// * [`FNA3D_SwapBuffers`]
#[derive(Debug, Clone)]
pub struct Device {
    lifetime: Rc<DeviceDrop>,
}

impl Device {
    pub fn raw(&self) -> *mut FNA3D_Device {
        self.lifetime.raw
    }
}

#[derive(Debug)]
struct DeviceDrop {
    raw: *mut FNA3D_Device,
}

impl Drop for DeviceDrop {
    fn drop(&mut self) {
        unsafe {
            FNA3D_DestroyDevice(self.raw);
        };
    }
}

/// Init/Quit
/// ---
impl Device {
    /// Creates a rendering context for use on the calling thread.
    ///
    /// * `params`:
    ///   The initial device/backbuffer settings.
    /// * `do_debug`:
    ///   Enable debugging and backend validation features at the cost of reduced overall
    ///   performance.
    ///
    /// Returns a device ready for use. Be sure to only call device functions from
    /// the thread that it was created on!
    ///
    /// See [initialization](./struct.Device.html#initialization)
    pub fn from_params(mut params: PresentationParameters, do_debug: bool) -> Self {
        Self {
            lifetime: Rc::new(DeviceDrop {
                raw: unsafe { FNA3D_CreateDevice(&mut params, do_debug as u8) },
            }),
        }
    }
}

/// Presentation
/// ---
impl Device {
    /// Presents the backbuffer to the window. `None` represents the full region
    ///
    /// * `override_window_handle`:
    ///   The OS window handle (not really "overridden").
    pub fn swap_buffers(
        &self,
        mut src: Option<Rect>,
        mut dest: Option<Rect>,
        override_window_handle: *mut c_void,
    ) {
        let src = src.as_mut().as_mut_ptr();
        let dest = dest.as_mut().as_mut_ptr();
        unsafe {
            FNA3D_SwapBuffers(self.raw(), src, dest, override_window_handle);
        }
    }
}

/// Drawing
/// ---
///
/// A "draw call" is actually a call of a drawing function, which is often
/// [`FNA3D_DrawIndexedPrimitives`] in FNA3D.
///
/// Set vertex/index data and sampler state before making draw calls.
///
/// See [rendering cycle](./struct.Device.html#rendering-cycle)
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
    pub fn clear(&self, options: enums::ClearOptions, color: Color, depth: f32, stencil: i32) {
        unsafe {
            FNA3D_Clear(
                self.raw(),
                options.bits(),
                &mut color.as_vec4() as *mut _,
                depth,
                stencil,
            );
        }
    }

    /// Draws data from vertex/index buffers
    ///
    /// This is good for reducing duplicate vertices.
    pub fn draw_indexed_primitives(
        &self,
        type_: enums::PrimitiveType,
        start_vertex: u32,
        start_index: u32,
        n_primitives: u32,
        indices: *mut Buffer,
        index_elem_size: enums::IndexElementSize,
    ) {
        unsafe {
            FNA3D_DrawIndexedPrimitives(
                self.raw(),
                type_ as FNA3D_PrimitiveType,
                start_vertex as i32,
                // min_vertex_index,
                -1, // this is ignored (it's just for XNA compatibility)
                // num_vertices,
                -1, // this is ignored (it's just for XNA compatibility)
                start_index as i32,
                n_primitives as i32,
                indices,
                index_elem_size as FNA3D_IndexElementSize,
            );
        }
    }

    /// Draws data from vertex/index buffers with instancing enabled.
    ///
    /// * `instance_count`:
    ///   The number of instances that will be drawn.
    ///
    /// * TODO: what is this
    pub fn draw_instanced_primitives(
        &self,
        type_: enums::PrimitiveType,
        base_vertex: u32,
        min_vertex_index: u32,
        num_vertices: u32,
        start_index: u32,
        prim_count: u32,
        instance_count: u32,
        indices: *mut Buffer,
        index_elem_size: enums::IndexElementSize,
    ) {
        unsafe {
            FNA3D_DrawInstancedPrimitives(
                self.raw(),
                type_ as FNA3D_PrimitiveType,
                base_vertex as i32,
                min_vertex_index as i32,
                num_vertices as i32,
                start_index as i32,
                prim_count as i32,
                instance_count as i32,
                indices,
                index_elem_size as FNA3D_IndexElementSize,
            );
        }
    }

    /// Draws data from vertex buffers.
    ///
    /// This may require duplicate vertices so prefer `draw_indexed_primitives` basically.
    pub fn draw_primitives(
        &self,
        type_: enums::PrimitiveType,
        vertex_start: u32,
        n_primitives: u32,
    ) {
        let vertex_start = vertex_start as i32;
        let prim_count = n_primitives as i32;
        unsafe {
            FNA3D_DrawPrimitives(
                self.raw(),
                type_ as FNA3D_PrimitiveType,
                vertex_start,
                prim_count,
            );
        }
    }
}

/// Mutable render states
/// ---
///
/// * TODO: what does mutable here mean
impl Device {
    /// Sets the view dimensions for rendering, relative to the active render target. It is required
    /// to call this at least once after calling `set_render_targets`, as the renderer may need to
    /// adjust these dimensions to fit the backend's potentially goofy coordinate systems.
    pub fn set_viewport(&self, viewport: &Viewport) {
        unsafe {
            FNA3D_SetViewport(self.raw(), viewport as *const _ as *mut _);
        }
    }

    /// Sets the scissor box for rendering, relative to the active render target. It is required to
    /// call this at least once after calling `set_render_targets`, as the renderer may need to
    /// adjust these dimensions to fit the backend's potentially goofy coordinate systems.
    pub fn set_scissor_rect(&self, scissor: &Rect) {
        unsafe {
            FNA3D_SetScissorRect(self.raw(), scissor as *const _ as *mut _);
        }
    }

    /// Gets the blending factor used for current draw calls.
    ///
    /// * `blend_factor`:
    ///   Filled with color being used as the device blend factor.
    pub fn blend_factor(&self, blend_factor: Color) {
        unsafe {
            FNA3D_GetBlendFactor(self.raw(), &mut blend_factor.raw() as *mut _);
        }
    }

    /// Sets the blending factor used for future draw calls.
    ///
    /// * `blend_factor`: The color to use as the device blend factor.
    pub fn set_blend_factor(&self, blend_factor: Color) {
        unsafe {
            FNA3D_SetBlendFactor(self.raw(), &mut blend_factor.raw() as *mut _);
        }
    }

    /// Gets the mask from which multisample fragment data is sampled from.
    ///
    /// Returns the coverage mask used to determine sample locations.
    pub fn multi_sample_mask(&self) -> i32 {
        unsafe { FNA3D_GetMultiSampleMask(self.raw()) }
    }

    /// Sets the reference value used for certain types of stencil testing.
    ///
    /// * `ref`: The new stencil reference value.
    pub fn set_multi_sample_mask(&self, mask: i32) {
        unsafe {
            FNA3D_SetMultiSampleMask(self.raw(), mask);
        }
    }

    /// Gets the reference value used for certain types of stencil testing.
    ///
    /// Returns the stencil reference value.
    pub fn reference_stencil(&self) -> i32 {
        unsafe { FNA3D_GetReferenceStencil(self.raw()) }
    }

    /// Sets the reference value used for certain types of stencil testing.
    ///
    /// * `ref`: The new stencil reference value.
    pub fn set_reference_stencil(&self, ref_: i32) {
        unsafe {
            FNA3D_SetReferenceStencil(self.raw(), ref_);
        }
    }
}

/// Immutable render states
/// ---
///
/// * TODO: what does immutable mean. fixed length?
impl Device {
    /// Applies a blending state to use for future draw calls. This only needs to be called when the
    /// state actually changes. Redundant calls may negatively affect performance!
    pub fn set_blend_state(&self, blend_state: &BlendState) {
        unsafe {
            FNA3D_SetBlendState(self.raw(), blend_state.raw() as *const _ as *mut _);
        }
    }

    /// Applies depth/stencil states to use for future draw calls. This only needs to be called when
    /// the states actually change. Redundant calls may negatively affect performance!
    pub fn set_depth_stencil_state(&self, depth_stencil_state: &DepthStencilState) {
        unsafe {
            FNA3D_SetDepthStencilState(self.raw(), depth_stencil_state.raw() as *const _ as *mut _);
        }
    }

    /// Applies the rasterizing state to use for future draw calls. It's generally a good idea to
    /// call this for each draw call, but if you really wanted to you could try reducing it to when
    ///  the state changes and when the render target state changes.
    pub fn apply_rasterizer_state(&self, rst: &RasterizerState) {
        unsafe {
            FNA3D_ApplyRasterizerState(self.raw(), rst.raw() as *const _ as *mut _);
        }
    }

    /// Updates a sampler slot with new texture/sampler data for future draw calls.
    /// This should only be called on slots that have modified texture/sampler state.
    /// Redundant calls may negatively affect performance!
    ///
    /// * `index`:
    ///   The sampler slot to update.
    pub fn verify_sampler(&self, index: u32, texture: *mut Texture, sampler: &SamplerState) {
        unsafe {
            FNA3D_VerifySampler(
                self.raw(),
                index as i32,
                texture,
                sampler as *const _ as *const FNA3D_SamplerState as *mut _,
            );
        }
    }

    /// Updates a vertex sampler slot with new texture/sampler data for future draw
    /// calls. This should only be called on slots that have modified texture/sampler
    /// state. Redundant calls may negatively affect performance!
    ///
    /// * `index`:
    ///   The vertex sampler slot to update.
    pub fn verify_vertex_sampler(&self, index: u32, texture: *mut Texture, sampler: &SamplerState) {
        unsafe {
            FNA3D_VerifyVertexSampler(
                self.raw(),
                index as i32,
                texture,
                sampler as *const _ as *mut FNA3D_SamplerState,
            );
        }
    }

    /// Updates the vertex attribute state to read from a set of vertex buffers. This
    /// should be the very last thing you call before making a draw call, as this
    /// does all the final prep work for the shader program before it's ready to use.
    ///
    /// * `bindings`:
    ///   The vertex buffers and their attribute data.
    /// * `is_bindings_updated`:
    ///   If the bindings array hasn't changed since the last update, this can be false. We'll only
    ///   update the shader state, updating vertex attribute data only if we 100% have to, for a
    ///   tiny performance improvement.
    /// * `base_vertex`:
    ///   This should be the same as the `base_vertex` parameter from your `draw*primitives` call,
    ///   if applicable. Not every rendering backend has native base vertex support, so we work
    ///   around it by passing this here.
    ///
    /// [`apply_effect`]: #method.apply_effect
    pub fn apply_vertex_buffer_bindings(
        &self,
        bindings: &[VertexBufferBinding],
        is_bindings_updated: bool,
        base_vertex: u32,
    ) {
        unsafe {
            FNA3D_ApplyVertexBufferBindings(
                self.raw(),
                bindings.as_ptr() as *mut _,
                bindings.len() as i32,
                is_bindings_updated as u8,
                base_vertex as i32,
            );
        }
    }
}

/// Render targets
/// ---
///
/// * back buffer = frame buffer = screen
impl Device {
    /// Sets the color/depth/stencil buffers to write future draw calls to.
    ///
    /// * `render_targets`:
    ///    The targets to write to, or `None` for the backbuffer (screen).
    /// * `n_render_targets`:
    ///    The size of the renderTargets array (can be 0).
    /// * `depth_stencil_buffer`:
    ///    The depth/stencil renderbuffer (can be `None`).
    /// * `depth_format`:
    ///    The format of the depth/stencil renderbuffer.
    /// * `preserve_depth_stencil_contents`:
    ///   `true` to store the color/depth/stencil contents for future use. Most of the time you'll
    ///    want to keep this at 0 to not waste GPU bandwidth.
    pub fn set_render_targets(
        &self,
        render_targets: Option<&mut RenderTargetBinding>,
        n_render_targets: u32,
        depth_stencil_buffer: Option<&mut Renderbuffer>,
        depth_format: enums::DepthFormat,
        preserve_target_contents: bool,
    ) {
        unsafe {
            FNA3D_SetRenderTargets(
                self.raw(),
                match render_targets {
                    Some(r) => r.raw_mut() as *mut _,
                    None => std::ptr::null_mut(),
                },
                n_render_targets as i32,
                depth_stencil_buffer.as_mut_ptr(),
                depth_format as FNA3D_DepthFormat,
                preserve_target_contents as u8,
            );
        }
    }

    /// After unsetting a render target, call this to resolve multisample targets or
    /// generate mipmap data for the final texture.
    ///
    /// * `target`: The render target to resolve once rendering is complete.
    pub fn resolve_target(&self, target: &mut RenderTargetBinding) {
        unsafe {
            FNA3D_ResolveTarget(self.raw(), target.raw_mut() as *mut _);
        }
    }

    /// After modifying the OS window state, call this to reset the backbuffer to
    /// match your window changes.
    ///
    /// * `params`: The new settings for the backbuffer.
    pub fn reset_backbuffer(&self, params: &PresentationParameters) {
        unsafe {
            FNA3D_ResetBackbuffer(self.raw(), params as *const _ as *mut _);
        }
    }

    /// Read the backbuffer's contents directly into client memory. This function is  basically one
    /// giant CPU/GPU sync point, do NOT ever call this during any  performance-critical situation!
    /// Just use it for screenshots.
    pub fn read_backbuffer(&self, x: u32, y: u32, w: u32, h: u32, data: &mut [u8]) {
        unsafe {
            FNA3D_ReadBackbuffer(
                self.raw(),
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                data as *const _ as *mut _,
                data.len() as i32,
            );
        }
    }

    pub fn get_backbuffer_size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            FNA3D_GetBackbufferSize(self.raw(), &mut w, &mut h);
        }
        (w as u32, h as u32)
    }

    pub fn get_backbuffer_surface_format(&self) -> enums::SurfaceFormat {
        let prim = unsafe { FNA3D_GetBackbufferSurfaceFormat(self.raw()) };
        // FIXME: is it ok to unwrap??
        enums::SurfaceFormat::from_u32(prim).unwrap()
    }

    pub fn get_backbuffer_depth_format(&self) -> enums::DepthFormat {
        let prim = unsafe { FNA3D_GetBackbufferDepthFormat(self.raw()) };
        // FIXME: is it ok to unwrap??
        enums::DepthFormat::from_u32(prim).unwrap()
    }

    pub fn get_backbuffer_multi_sample_count(&self) -> u32 {
        unsafe { FNA3D_GetBackbufferMultiSampleCount(self.raw()) as u32 }
    }
}

/// Textures
/// ---
impl Device {
    /// Creates a 2D texture to be applied to `verify_sampler`
    ///
    /// * `fmt`:
    ///   The pixel format of the texture data.
    /// * `level_count`:
    ///   The number of mipmap levels to allocate.
    /// * `is_render_target`:
    ///   Set this to 1 when using this with `set_render_targets`.
    ///
    /// Returns an allocated `Texture*` object. Note that the contents of the
    /// texture are undefined, so you must call `set_texture_data_2d` at least once before drawing!
    pub fn create_texture_2d(
        &self,
        fmt: enums::SurfaceFormat,
        w: u32,
        h: u32,
        level_count: u32,
        is_render_target: bool,
    ) -> *mut Texture {
        unsafe {
            FNA3D_CreateTexture2D(
                self.raw(),
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
    /// * `depth`:
    ///   The depth of the texture image.
    /// * `level_count`:
    ///   The number of mipmap levels to allocate.
    ///
    /// Returns an allocated FNA3D_Texture* object. Note that the contents of the
    /// texture are undefined, so you must call `SetData` at least once before drawing!
    pub fn create_texture_3d(
        &self,
        fmt: enums::SurfaceFormat,
        w: u32,
        h: u32,
        depth: u32,
        level_count: u32,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            FNA3D_CreateTexture3D(
                self.raw(),
                fmt as u32,
                w as i32,
                h as i32,
                depth as i32,
                level_count as i32,
            )
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
        &self,
        fmt: enums::SurfaceFormat,
        size: u32,
        level_count: u32,
        is_render_target: bool,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            FNA3D_CreateTextureCube(
                self.raw(),
                fmt as u32,
                size as i32,
                level_count as i32,
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
    pub fn add_dispose_texture(&self, texture: *mut Texture) {
        unsafe {
            FNA3D_AddDisposeTexture(self.raw(), texture);
        }
    }

    /// Uploads image data to a 2D texture object.
    ///
    /// * `level`:
    ///   The mipmap level being updated.
    pub fn set_texture_data_2d(
        &self,
        texture: *mut Texture,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        level: u32,
        data: &[u8],
    ) {
        unsafe {
            FNA3D_SetTextureData2D(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                level as i32,
                data.as_ptr() as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Uploads image data to a 3D texture object.
    ///
    /// * `level`:
    ///   The mipmap level being updated.
    pub fn set_texture_data_3d(
        &self,
        texture: &mut Texture,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
        h: u32,
        depth: u32,
        level: u32,
        data: &mut [u8],
    ) {
        unsafe {
            FNA3D_SetTextureData3D(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                z as i32,
                w as i32,
                h as i32,
                depth as i32,
                level as i32,
                data.as_mut_ptr() as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Uploads image data to a single face of a texture cube object.
    ///
    /// * `fmt`:
    ///   Should match the format provided to CreateTextureCube.
    /// * `cube_map_face`:
    ///   The face of the cube being updated.
    /// * `level`:
    ///   The mipmap level being updated.
    pub fn set_texture_data_cube(
        &self,
        texture: &mut Texture,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        cube_map_face: enums::CubeMapFace,
        level: i32,
        data: &mut [u8],
    ) {
        unsafe {
            FNA3D_SetTextureDataCube(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                cube_map_face as u32,
                level,
                data.as_mut_ptr() as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Uploads YUV image data to three ALPHA8 texture objects.
    ///
    /// * `data`:
    ///   A slice of the raw YUV image data.
    pub fn set_texture_data_yuv(
        &self,
        y: &mut Texture,
        u: &mut Texture,
        v: &mut Texture,
        y_width: u32,
        y_height: u32,
        uv_width: u32,
        uv_height: u32,
        data: &[u8],
    ) {
        unsafe {
            FNA3D_SetTextureDataYUV(
                self.raw(),
                y,
                u,
                v,
                y_width as i32,
                y_height as i32,
                uv_width as i32,
                uv_height as i32,
                data as *const _ as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Pulls image data from a 2D texture into client memory. Like any GetData,
    /// this is generally asking for a massive CPU/GPU sync point, don't call this
    /// unless there's absolutely no other way to use the image data!
    ///
    /// * `level`:
    ///   The mipmap level being read.
    pub fn get_texture_data_2d(
        &self,
        texture: &mut Texture,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        level: u32,
        data: &mut [u8],
    ) {
        unsafe {
            FNA3D_GetTextureData2D(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                level as i32,
                data as *const _ as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Pulls image data from a 3D texture into client memory. Like any GetData,
    /// this is generally asking for a massive CPU/GPU sync point, don't call this
    /// unless there's absolutely no other way to use the image data!
    ///
    /// * `level`:
    ///    The mipmap level being read.
    pub fn get_texture_data_3d(
        &self,
        texture: &mut Texture,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
        h: u32,
        depth: u32,
        level: u32,
        data: &mut [u8],
    ) {
        unsafe {
            FNA3D_GetTextureData3D(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                z as i32,
                w as i32,
                h as i32,
                depth as i32,
                level as i32,
                data as *const _ as *mut _,
                data.len() as i32,
            );
        }
    }

    /// Pulls image data from a single face of a texture cube object into client
    /// memory. Like any GetData, this is generally asking for a massive CPU/GPU sync
    /// point, don't call this unless there's absolutely no other way to use the
    /// image data!
    ///
    /// * `texture`:	The texture object being read.
    /// * `cube_map_face`:
    ///    The face of the cube being read.
    /// * `level`:
    ///    The mipmap level being read.
    pub fn get_texture_data_cube(
        &self,
        texture: *mut Texture,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        cube_map_face: enums::CubeMapFace,
        level: u32,
        data: &mut [u8],
    ) {
        unsafe {
            FNA3D_GetTextureDataCube(
                self.raw(),
                texture,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                cube_map_face as u32,
                level as i32,
                data as *const _ as *mut _,
                data.len() as i32,
            );
        }
    }
}

/// Renderbuffers
/// ---
impl Device {
    /// Creates a color buffer to be used by `set_render_targets` / `resolve_target`.
    ///
    /// * `multi_sample_count`:
    ///    The MSAA value for the color buffer.
    ///
    /// Returns a color [`FNA3D_Renderbuffer`] object.
    pub fn gen_color_renderbuffer(
        &self,
        w: u32,
        h: u32,
        fmt: enums::SurfaceFormat,
        multi_sample_count: u32,
        texture: *mut Texture,
    ) -> *mut Renderbuffer {
        unsafe {
            FNA3D_GenColorRenderbuffer(
                self.raw(),
                w as i32,
                h as i32,
                fmt as u32,
                multi_sample_count as i32,
                texture,
            )
        }
    }

    /// Creates a depth/stencil buffer to be used by `set_render_targets`.
    ///
    /// * `multi_sample_count`:
    ///    The MSAA value for the depth/stencil buffer.
    ///
    /// Returns a depth/stencil [`FNA3D_Renderbuffer`] object.
    pub fn gen_depth_stencil_renderbuffer(
        &self,
        w: u32,
        h: u32,
        fmt: enums::DepthFormat,
        multi_sample_count: i32,
    ) -> *mut Renderbuffer {
        unsafe {
            FNA3D_GenDepthStencilRenderbuffer(
                self.raw(),
                w as i32,
                h as i32,
                fmt as u32,
                multi_sample_count,
            )
        }
    }

    /// Sends a renderbuffer to be destroyed by the renderer. Note that we call it
    /// "add_dispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `renderbuffer`: The FNA3D_Renderbuffer to be destroyed.
    pub fn add_dispose_renderbuffer(&self, renderbuffer: &mut Renderbuffer) {
        unsafe {
            FNA3D_AddDisposeRenderbuffer(self.raw(), renderbuffer);
        }
    }
}

/// Vertex buffers
/// ---
///
/// `*mut Buffer` is GPU buffer and we need to upload (send, copy) from CPU memory. This is done
/// via `FNA3D_SetVertexBufferData` or `FNA3D_SetIndexBufferData` in FNA.
impl Device {
    /// Creates a vertex buffer to be used by Draw*Primitives.
    ///
    /// * `dynamic`:
    ///   Set to `true` if this buffer will be updated frequently.
    ///
    /// Returns an allocated FNA3D_Buffer* object. Note that the contents of the
    /// buffer are undefined, so you must call `SetData` at least once before drawing!
    pub fn gen_vertex_buffer(
        &self,
        is_dynamic: bool,
        usage: enums::BufferUsage,
        size_in_bytes: u32,
    ) -> *mut Buffer {
        unsafe {
            FNA3D_GenVertexBuffer(
                self.raw(),
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
    pub fn add_dispose_vertex_buffer(&self, buffer: *mut Buffer) {
        unsafe {
            FNA3D_AddDisposeVertexBuffer(self.raw(), buffer);
        }
    }

    /// Sets a region of the vertex buffer with client data.
    ///
    /// This is wrapped in `VertexBuffer:SetData` in FNA. Remember to call
    /// `apply_vertex_buffer_bindings` before drawing.
    ///
    /// * `opts`:
    ///   Try not to call NONE if this is a dynamic buffer!
    pub fn set_vertex_buffer_data<T>(
        &self,
        buf: *mut Buffer,
        buf_offset_in_bytes: u32,
        data: &[T],
        opts: enums::SetDataOptions,
    ) {
        let data_len_in_bytes = data.len() * std::mem::size_of::<T>();
        unsafe {
            // Note that it has odd API for XNA compatibility
            FNA3D_SetVertexBufferData(
                self.raw(),
                buf,
                buf_offset_in_bytes as i32,
                data as *const _ as *mut _,
                data_len_in_bytes as i32,
                1, // see `FNA3D.h` for details (XNA compatibility thing)
                1, // see `FNA3D.h` for details (XNA compatibility thing)
                opts as u32,
            );
        }
    }

    /// Pulls data from a region of the vertex buffer into a client pointer.
    pub fn get_vertex_buffer_data(
        &self,
        buffer: *mut Buffer,
        buf_offset_in_bytes: u32,
        data: *mut ::std::os::raw::c_void,
        // element_count: i32,
        elem_size_in_bytes: u32,
        // vertex_stride: i32,
    ) {
        unsafe {
            FNA3D_GetVertexBufferData(
                self.raw(),
                buffer,
                buf_offset_in_bytes as i32,
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
/// ---
///
/// `*mut Buffer` is GPU buffer and we need to upload (send, copy) from CPU memory. This is done
/// via `FNA3D_SetVertexBufferData` or `FNA3D_SetIndexBufferData` in FNA.
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
        &self,
        is_dynamic: bool,
        usage: enums::BufferUsage,
        size_in_bytes: u32,
    ) -> *mut Buffer {
        unsafe {
            FNA3D_GenIndexBuffer(
                self.raw(),
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
    pub fn add_dispose_index_buffer(&self, buf: *mut Buffer) {
        unsafe {
            FNA3D_AddDisposeIndexBuffer(self.raw(), buf);
        }
    }

    /// Sets a region of the index buffer with client data.
    ///
    /// The buffer will be copied so you can free it after calling this
    ///
    /// * `buf`:
    ///   The index buffer to be updated.
    /// * `buf_offset_in_bytes`:
    ///   The starting offset of the buffer to write into.
    /// * `data`:
    ///   The client data to write into the buffer.
    /// * `opts`:
    ///   Try not to call NONE if this is a dynamic buffer!
    pub fn set_index_buffer_data<T>(
        &self,
        buf: *mut Buffer,
        buf_offset_in_bytes: u32,
        data: &[T],
        opts: enums::SetDataOptions,
    ) {
        let len_bytes = data.len() * std::mem::size_of::<T>();
        unsafe {
            FNA3D_SetIndexBufferData(
                self.raw(),
                buf,
                buf_offset_in_bytes as i32,
                data.as_ptr() as *mut _,
                len_bytes as i32,
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
        &self,
        buf: *mut Buffer,
        buf_offset_in_bytes: u32,
        data: &[T],
        // data: *mut c_void,
        // data_len: i32,
    ) {
        let len_bytes = data.len() * std::mem::size_of::<T>();
        unsafe {
            FNA3D_GetIndexBufferData(
                self.raw(),
                buf,
                buf_offset_in_bytes as i32,
                data.as_ptr() as *mut _,
                len_bytes as i32,
            );
        }
    }
}

/// Effects
/// ---
///
/// See [`crate::mojo`] module for more information and some helpers.
impl Device {
    /// Parses and compiles a Direct3D 9 Effects Framework binary.
    ///
    /// Returns `(effect, effect_data)`. You have to detect errors by looking into `error_count`
    /// field of the second return value.
    ///
    /// * `effect_code`:
    ///   The D3D9 Effect binary blob.
    /// * `effect_code_length`:
    ///   The size (in bytes) of the blob.
    pub fn create_effect(
        &self,
        effect_code: *mut u8,
        effect_code_len: u32,
    ) -> (*mut Effect, *mut mojo::Effect) {
        let mut effect = std::ptr::null_mut();
        let mut data = std::ptr::null_mut();
        unsafe {
            FNA3D_CreateEffect(
                self.raw(),
                effect_code,
                effect_code_len,
                &mut effect,
                &mut data,
            );
        }

        if effect.is_null() || data.is_null() {
            log::warn!("tried to create effect but failed");
        }

        (effect, data as *mut _)
    }

    /// Copies a compiled Effect, including its current technique/parameter data.
    ///
    /// * `clone_source`:
    ///   The FNA3D_Effect to copy.
    /// * `effect`:
    ///   Filled with the new compiled FNA3D_Effect*.
    /// * `effect_data`:
    ///   Filled with the copied Effect Framework data.
    pub fn clone_effect(&self, clone_source: *mut Effect) -> (*mut Effect, *mut mojo::Effect) {
        let mut effect = std::ptr::null_mut();
        let mut data = std::ptr::null_mut();
        unsafe {
            FNA3D_CloneEffect(self.raw(), clone_source, &mut effect, &mut data);
        }
        (effect, data as *mut _)
    }

    /// Sends an Effect to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `effect`: The FNA3D_Effect to be destroyed.
    pub fn add_dispose_effect(&self, effect: *mut Effect) {
        unsafe {
            FNA3D_AddDisposeEffect(self.raw(), effect);
        }
    }

    /// Sets the active technique on the Effect.
    ///
    /// * `effect`:	The Effect to be modified.
    /// * `technique`:	The technique to be used by future `apply_effect` calls.
    pub fn set_effect_technique(&self, effect: *mut Effect, technique: *mut mojo::EffectTechnique) {
        unsafe {
            FNA3D_SetEffectTechnique(self.raw(), effect, technique as *mut _);
        }
    }

    /// Applies an effect pass from a given Effect, setting the active shader program
    /// and committing any parameter data changes to be used by future draw calls.
    ///
    /// * `effect`:
    ///   The Effect to be applied.
    /// * `pass`:
    ///   The current technique's pass index to be applied.
    /// * `state_changes`:
    ///   Structure to be filled with any render state changes
    ///	  made by the Effect. This must be valid for the entire
    ///   duration that this Effect is being applied.
    pub fn apply_effect(
        &self,
        effect: *mut Effect,
        pass: u32,
        state_changes: &mojo::EffectStateChanges,
    ) {
        unsafe {
            FNA3D_ApplyEffect(
                self.raw(),
                effect,
                pass,
                state_changes as *const _ as *mut _,
            );
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
        &self,
        effect: *mut Effect,
        state_changes: *mut mojo::EffectStateChanges,
    ) {
        unsafe {
            FNA3D_BeginPassRestore(self.raw(), effect, state_changes as *mut _);
        }
    }

    /// Ends a pass started by BeginPassRestore, unsetting the current Effect and
    /// restoring the previous shader state from before BeginPassRestore was called.
    ///
    /// * `effect`: The Effect that was applied at BeginPassRestore.
    pub fn end_pass_restore(&self, effect: *mut Effect) {
        unsafe {
            FNA3D_EndPassRestore(self.raw(), effect);
        }
    }
}

/// Queries
/// ---
impl Device {
    /// Creates an object used to run occlusion queries.
    ///
    /// Returns an FNA3D_Query object.
    pub fn create_query(&self) -> *mut Query {
        unsafe { FNA3D_CreateQuery(self.raw()) }
    }

    /// Sends a query object to be destroyed by the renderer. Note that we call it
    /// "AddDispose" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    ///
    /// * `query`: The FNA3D_Query to be destroyed.
    pub fn add_dispose_query(&self, query: *mut Query) {
        unsafe {
            FNA3D_AddDisposeQuery(self.raw(), query);
        }
    }

    /// Marks the start of when a query object should count pixels written.
    ///
    /// * `query`: The FNA3D_Query to start.
    pub fn query_begin(&self, query: *mut Query) {
        unsafe {
            FNA3D_QueryBegin(self.raw(), query);
        }
    }

    /// Marks the end of when a query object should count pixels written. Note that
    /// this does NOT mean the query has finished executing, you will need to poll
    /// QueryComplete before checking the pixel count.
    ///
    /// * `query`: The FNA3D_Query to stop.
    pub fn query_end(&self, query: *mut Query) {
        unsafe {
            FNA3D_QueryEnd(self.raw(), query);
        }
    }

    /// Call this until the function returns 1 to safely query for pixel counts.
    ///
    /// * `query`: The FNA3D_Query to sync with.
    ///
    /// Returns 1 when complete, 0 when still in execution.
    pub fn query_complete(&self, query: *mut Query) -> bool {
        unsafe { FNA3D_QueryComplete(self.raw(), query) != 0 }
    }

    /// Query the pixels counted between the begin/end markers set for the object.
    ///
    /// query: The FNA3D_Query to poll for pixel count
    ///
    /// Returns the pixels written during the begin/end period.
    pub fn query_pixel_count(&self, query: *mut Query) -> i32 {
        unsafe { FNA3D_QueryPixelCount(self.raw(), query) }
    }
}

/// Feature queries
/// ---
impl Device {
    /// True if the renderer natively supports DXT1 texture data.
    pub fn supports_dxt1(&self) -> bool {
        unsafe { FNA3D_SupportsDXT1(self.raw()) != 0 }
    }

    /// True if the renderer natively supports S3TC texture data.
    pub fn supports_s3_tc(&self) -> bool {
        unsafe { FNA3D_SupportsS3TC(self.raw()) != 0 }
    }

    /// True if the renderer natively supports hardware instancing.
    pub fn supports_hardware_instancing(&self) -> bool {
        unsafe { FNA3D_SupportsHardwareInstancing(self.raw()) != 0 }
    }

    /// True if the renderer natively supports asynchronous buffer writing.
    pub fn supports_no_overwrite(&self) -> bool {
        unsafe { FNA3D_SupportsNoOverwrite(self.raw()) != 0 }
    }

    /// Returns the number of sampler slots supported by the renderer (texture, vertex_texture)
    pub fn get_max_texture_slots(&self) -> (u32, u32) {
        let (mut textures, mut vertex_textures): (i32, i32) = (0, 0);
        unsafe {
            FNA3D_GetMaxTextureSlots(
                self.raw(),
                &mut textures as *mut _,
                &mut vertex_textures as *mut _,
            );
        }
        (textures as u32, vertex_textures as u32)
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
        &self,
        fmt: enums::SurfaceFormat,
        multi_sample_count: u32,
    ) -> i32 {
        unsafe { FNA3D_GetMaxMultiSampleCount(self.raw(), fmt as u32, multi_sample_count as i32) }
    }
}

/// Debug
/// ---
impl Device {
    /// Sets an arbitrary string constant to be stored in a rendering API trace,
    /// useful for labeling call streams for debugging purposes.
    ///
    /// * `text`: The string constant to mark in the API call stream.
    // FIXME: C string wrapper?? I have to read Rust nomicon
    pub fn set_string_marker(&self, text: *const ::std::os::raw::c_char) {
        unsafe {
            FNA3D_SetStringMarker(self.raw(), text);
        }
    }
}
