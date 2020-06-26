//! Wrapper of `FNA3D_Device`

use std::ptr;
// this should be `std::ffi::c_void` but `bindgen` uses:
use std::os::raw::c_void;

use fna3d_sys as sys;

use crate::{fna3d_enums as enums, fna3d_structs::*, utils::AsVec4};
use enum_primitive::*;

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
/// # Things to take care
///
/// - using disposing functions for:
///     - `Buffer`
///     - `Renderbuffer`
///     - `Effect`
///     - `Query`
///     - `Texture`
pub struct Device {
    raw: *mut sys::FNA3D_Device,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            sys::FNA3D_DestroyDevice(self.raw);
        };
    }
}

impl Device {
    pub fn from_params(params: &mut PresentationParameters, is_debug: bool) -> Self {
        let dbg = if is_debug { 1 } else { 0 };
        Self {
            // debug mode
            raw: unsafe { sys::FNA3D_CreateDevice(params, dbg) },
        }
    }

    pub fn raw(&self) -> *mut sys::FNA3D_Device {
        self.raw
    }

    pub fn begin_frame(&mut self) {
        unsafe {
            sys::FNA3D_BeginFrame(self.raw);
        }
    }

    pub fn swap_buffers(
        &mut self,
        // TODO: different function name for (None, None)?
        mut src: Option<Rect>,
        mut dest: Option<Rect>,
        window_handle: *mut c_void,
    ) {
        let src = src.as_mut().as_mut_ptr();
        let dest = dest.as_mut().as_mut_ptr();
        unsafe {
            sys::FNA3D_SwapBuffers(self.raw, src, dest, window_handle);
        }
    }

    pub fn clear(&mut self, options: enums::ClearOptions, color: &Color, depth: f32, stencil: i32) {
        unsafe {
            sys::FNA3D_Clear(
                self.raw,
                options as u32,
                &mut color.as_vec4() as *mut _,
                depth,
                stencil,
            );
        }
    }

    pub fn draw_indexed_primitives(
        &mut self,
        prim: enums::PrimitiveType,
        base_vertex: i32,
        min_vertex_index: i32,
        num_vertices: i32,
        start_index: i32,
        prim_count: i32,
        // TODO: is this OK?
        indices: &Device,
        index_element_size: sys::FNA3D_IndexElementSize,
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
                indices.raw as *mut _,
                index_element_size,
            );
        }
    }

    pub fn draw_instanced_primitives(
        &mut self,
        prim: enums::PrimitiveType,
        base_vertex: i32,
        min_vertex_index: i32,
        num_vertices: i32,
        start_index: i32,
        primitive_count: i32,
        instance_count: i32,
        // TODO: is this OK?
        indices: *mut sys::FNA3D_Buffer,
        index_element_size: sys::FNA3D_IndexElementSize,
    ) {
        unsafe {
            sys::FNA3D_DrawInstancedPrimitives(
                self.raw,
                prim as sys::FNA3D_PrimitiveType,
                base_vertex,
                min_vertex_index,
                num_vertices,
                start_index,
                primitive_count,
                instance_count,
                indices,
                index_element_size,
            );
        }
    }

    pub fn set_viewport(&mut self, viewport: &mut Viewport) {
        unsafe {
            sys::FNA3D_SetViewport(self.raw, viewport);
        }
    }

    pub fn set_scissor_rect(&mut self, mut scissor: Option<Rect>) {
        unsafe {
            sys::FNA3D_SetScissorRect(self.raw, scissor.as_mut().as_mut_ptr());
        }
    }

    pub fn get_blend_factor(&mut self, mut blend_factor: Color) {
        unsafe {
            sys::FNA3D_GetBlendFactor(self.raw, &mut blend_factor as *mut _);
        }
    }

    pub fn set_blend_factor(&mut self, mut blend_factor: Color) {
        unsafe {
            sys::FNA3D_SetBlendFactor(self.raw, &mut blend_factor as *mut _);
        }
    }

    pub fn get_multi_sample_mask(&self) -> i32 {
        unsafe { sys::FNA3D_GetMultiSampleMask(self.raw) }
    }

    pub fn set_multi_sample_mask(&mut self, mask: i32) {
        unsafe {
            sys::FNA3D_SetMultiSampleMask(self.raw, mask);
        }
    }

    pub fn get_reference_stencil(&self) -> i32 {
        unsafe { sys::FNA3D_GetReferenceStencil(self.raw) }
    }

    pub fn set_reference_stencil(&mut self, ref_: i32) {
        unsafe {
            sys::FNA3D_SetReferenceStencil(self.raw, ref_);
        }
    }

    pub fn set_blend_state(&mut self, blend_state: &BlendState) {
        unsafe {
            // REMARK: this is SAFE because BlendState is copied in a specific Device
            sys::FNA3D_SetBlendState(self.raw, &mut blend_state.as_sys_value() as *mut _);
        }
    }

    pub fn set_depth_stencil_state(&mut self, depth_stencil_state: &mut DepthStencilState) {
        unsafe {
            sys::FNA3D_SetDepthStencilState(self.raw, depth_stencil_state.raw() as *mut _);
        }
    }

    pub fn apply_rasterizer_state(&mut self, rst: &mut RasterizerState) {
        unsafe {
            sys::FNA3D_ApplyRasterizerState(self.raw, rst);
        }
    }

    pub fn verify_sampler(
        &mut self,
        index: i32,
        texture: &mut Texture,
        sampler: &mut SamplerState,
    ) {
        unsafe {
            sys::FNA3D_VerifySampler(self.raw, index, texture, sampler);
        }
    }

    pub fn verify_vertex_sampler(
        &mut self,
        index: i32,
        texture: &mut Texture,
        sampler: &mut SamplerState,
    ) {
        unsafe {
            sys::FNA3D_VerifyVertexSampler(self.raw, index, texture, sampler);
        }
    }

    pub fn apply_vertex_buffer_bindings(
        &mut self,
        bindings: &mut VertexBufferBinding,
        num_bindings: i32,
        bindings_updated: u8,
        base_vertex: i32,
    ) {
        unsafe {
            sys::FNA3D_ApplyVertexBufferBindings(
                self.raw,
                bindings,
                num_bindings,
                bindings_updated,
                base_vertex,
            );
        }
    }

    pub fn set_render_targets(
        &mut self,
        render_targets: &mut RenderTargetBinding,
        num_render_targets: i32,
        depth_stencil_buffer: &mut Renderbuffer,
        depth_format: enums::DepthFormat,
    ) {
        unsafe {
            sys::FNA3D_SetRenderTargets(
                self.raw,
                render_targets,
                num_render_targets,
                depth_stencil_buffer,
                depth_format as u32,
            );
        }
    }

    pub fn resolve_target(&mut self, target: &mut RenderTargetBinding) {
        unsafe {
            sys::FNA3D_ResolveTarget(self.raw, target);
        }
    }

    pub fn reset_backbuffer(&mut self, params: &mut PresentationParameters) {
        unsafe {
            sys::FNA3D_ResetBackbuffer(self.raw, params as *mut _);
        }
    }

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

    pub fn get_backbuffer_size(&mut self) -> (i32, i32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            sys::FNA3D_GetBackbufferSize(self.raw, &mut w, &mut h);
        }
        (w, h)
    }

    pub fn get_backbuffer_surface_format(&self) -> enums::SurfaceFormat {
        let prim = unsafe { sys::FNA3D_GetBackbufferSurfaceFormat(self.raw) };
        // FIXME: is it ok to unwrap??
        enums::SurfaceFormat::from_u32(prim).unwrap()
    }

    pub fn get_backbuffer_depth_format(&self) -> enums::DepthFormat {
        let prim = unsafe { sys::FNA3D_GetBackbufferDepthFormat(self.raw) };
        // FIXME: is it ok to unwrap??
        enums::DepthFormat::from_u32(prim).unwrap()
    }

    pub fn get_backbuffer_multi_sample_count(&self) -> i32 {
        unsafe { sys::FNA3D_GetBackbufferMultiSampleCount(self.raw) }
    }

    pub fn create_texture_2d(
        &mut self,
        format: enums::SurfaceFormat,
        width: i32,
        height: i32,
        level_count: i32,
        is_render_target: u8,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTexture2D(
                self.raw,
                format as u32,
                width,
                height,
                level_count,
                is_render_target,
            )
        }
    }

    pub fn create_texture_3d(
        &mut self,
        format: enums::SurfaceFormat,
        width: i32,
        height: i32,
        depth: i32,
        level_count: i32,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTexture3D(self.raw, format as u32, width, height, depth, level_count)
        }
    }

    pub fn create_texture_cube(
        &mut self,
        format: enums::SurfaceFormat,
        size: i32,
        level_count: i32,
        is_render_target: bool,
        // TODO: maybe make a wrapper
    ) -> *mut Texture {
        unsafe {
            sys::FNA3D_CreateTextureCube(
                self.raw,
                format as u32,
                size,
                level_count,
                is_render_target as u8,
            )
        }
    }

    pub fn add_dispose_texture(&mut self, texture: &mut Texture) {
        unsafe {
            sys::FNA3D_AddDisposeTexture(self.raw, texture);
        }
    }

    pub fn set_texture_data_2d(
        &mut self,
        texture: &mut Texture,
        format: enums::SurfaceFormat,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        level: i32,
        data: *mut c_void,
        data_length: i32,
    ) {
        unsafe {
            sys::FNA3D_SetTextureData2D(
                self.raw,
                texture,
                format as u32,
                x,
                y,
                w,
                h,
                level,
                data,
                data_length,
            );
        }
    }

    pub fn set_texture_data_3d(
        &mut self,
        texture: &mut Texture,
        format: enums::SurfaceFormat,
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
                self.raw,
                texture,
                format as u32,
                x,
                y,
                z,
                w,
                h,
                d,
                level,
                data,
                data_len,
            );
        }
    }

    pub fn set_texture_data_cube(
        &mut self,
        texture: &mut Texture,
        format: enums::SurfaceFormat,
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
                format as u32,
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

    pub fn get_texture_data_2d(
        &mut self,
        texture: &mut Texture,
        format: enums::SurfaceFormat,
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
                self.raw,
                texture,
                format as u32,
                x,
                y,
                w,
                h,
                level,
                data,
                data_len,
            );
        }
    }

    pub fn get_texture_data_3d(
        &mut self,
        texture: &mut Texture,
        format: enums::SurfaceFormat,
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
                self.raw,
                texture,
                format as u32,
                x,
                y,
                z,
                w,
                h,
                d,
                level,
                data,
                data_len,
            );
        }
    }

    pub fn get_texture_data_cube(
        &mut self,
        texture: *mut Texture,
        format: enums::SurfaceFormat,
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
                format as u32,
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

    pub fn gen_color_renderbuffer(
        &mut self,
        width: i32,
        height: i32,
        format: enums::SurfaceFormat,
        multi_sample_count: i32,
        texture: &mut Texture,
    ) -> *mut Renderbuffer {
        unsafe {
            sys::FNA3D_GenColorRenderbuffer(
                self.raw,
                width,
                height,
                format as u32,
                multi_sample_count,
                texture,
            )
        }
    }

    pub fn gen_depth_stencil_renderbuffer(
        &mut self,
        width: i32,
        height: i32,
        format: enums::DepthFormat,
        multi_sample_count: i32,
    ) -> *mut Renderbuffer {
        unsafe {
            sys::FNA3D_GenDepthStencilRenderbuffer(
                self.raw,
                width,
                height,
                format as u32,
                multi_sample_count,
            )
        }
    }

    pub fn add_dispose_renderbuffer(&mut self, renderbuffer: &mut Renderbuffer) {
        unsafe {
            sys::FNA3D_AddDisposeRenderbuffer(self.raw, renderbuffer);
        }
    }

    pub fn gen_vertex_buffer(
        &mut self,
        dynamic: u8,
        usage: enums::BufferUsage,
        size_in_bytes: i32,
    ) -> *mut Buffer {
        unsafe { sys::FNA3D_GenVertexBuffer(self.raw, dynamic, usage as u32, size_in_bytes) }
    }

    pub fn add_dispose_vertex_buffer(&mut self, buffer: &mut Buffer) {
        unsafe {
            sys::FNA3D_AddDisposeVertexBuffer(self.raw, buffer);
        }
    }

    pub fn set_vertex_buffer_data(
        &mut self,
        buffer: *mut Buffer,
        offset_in_bytes: i32,
        data: *mut c_void,
        element_count: i32,
        element_size_in_bytes: i32,
        vertex_stride: i32,
        options: enums::SetDataOptions,
    ) {
        unsafe {
            sys::FNA3D_SetVertexBufferData(
                self.raw,
                buffer,
                offset_in_bytes,
                data,
                element_count,
                element_size_in_bytes,
                vertex_stride,
                options as u32,
            );
        }
    }

    pub fn get_vertex_buffer_data(
        &mut self,
        buffer: &mut Buffer,
        offset_in_bytes: i32,
        data: *mut ::std::os::raw::c_void,
        element_count: i32,
        element_size_in_bytes: i32,
        vertex_stride: i32,
    ) {
        unsafe {
            sys::FNA3D_GetVertexBufferData(
                self.raw,
                buffer,
                offset_in_bytes,
                data,
                element_count,
                element_size_in_bytes,
                vertex_stride,
            );
        }
    }

    pub fn gen_index_buffer(
        &mut self,
        dynamic: u8,
        usage: enums::BufferUsage,
        size_in_bytes: i32,
    ) -> *mut Buffer {
        unsafe { sys::FNA3D_GenIndexBuffer(self.raw, dynamic, usage as u32, size_in_bytes) }
    }

    pub fn add_dispose_index_buffer(&mut self, buf: &mut Buffer) {
        unsafe {
            sys::FNA3D_AddDisposeIndexBuffer(self.raw, buf);
        }
    }

    pub fn set_index_buffer_data(
        &mut self,
        buffer: &mut Buffer,
        offset_in_bytes: i32,
        data: *mut c_void,
        data_len: i32,
        options: enums::SetDataOptions,
    ) {
        unsafe {
            sys::FNA3D_SetIndexBufferData(
                self.raw,
                buffer,
                offset_in_bytes,
                data,
                data_len,
                options as u32,
            );
        }
    }

    pub fn get_index_buffer_data(
        &mut self,
        buffer: &mut Buffer,
        offset_in_bytes: i32,
        data: *mut c_void,
        data_len: i32,
    ) {
        unsafe {
            sys::FNA3D_GetIndexBufferData(self.raw, buffer, offset_in_bytes, data, data_len);
        }
    }
}

/// Effects
impl Device {
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

    pub fn add_dispose_effect(&mut self, effect: *mut Effect) {
        unsafe {
            sys::FNA3D_AddDisposeEffect(self.raw, effect);
        }
    }

    pub fn set_effect_technique(
        &mut self,
        effect: *mut Effect,
        technique: *mut mojo::EffectTechnique,
    ) {
        unsafe {
            sys::FNA3D_SetEffectTechnique(self.raw, effect, technique);
        }
    }

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

    pub fn begin_pass_restore(
        &mut self,
        effect: *mut Effect,
        state_changes: *mut mojo::EffectStateChanges,
    ) {
        unsafe {
            sys::FNA3D_BeginPassRestore(self.raw, effect, state_changes);
        }
    }

    pub fn end_pass_restore(&mut self, effect: *mut Effect) {
        unsafe {
            sys::FNA3D_EndPassRestore(self.raw, effect);
        }
    }

    pub fn create_query(&mut self) -> *mut Query {
        unsafe { sys::FNA3D_CreateQuery(self.raw) }
    }

    pub fn add_dispose_query(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_AddDisposeQuery(self.raw, query);
        }
    }

    pub fn query_begin(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_QueryBegin(self.raw, query);
        }
    }

    pub fn query_end(&mut self, query: *mut Query) {
        unsafe {
            sys::FNA3D_QueryEnd(self.raw, query);
        }
    }

    pub fn query_complete(&mut self, query: *mut Query) -> bool {
        unsafe { sys::FNA3D_QueryComplete(self.raw, query) == 0 }
    }

    pub fn query_pixel_count(&mut self, query: *mut Query) -> i32 {
        unsafe { sys::FNA3D_QueryPixelCount(self.raw, query) }
    }

    pub fn supports_dxt1(&self) -> bool {
        unsafe { sys::FNA3D_SupportsDXT1(self.raw) == 0 }
    }

    pub fn supports_s3_tc(&self) -> bool {
        unsafe { sys::FNA3D_SupportsS3TC(self.raw) == 0 }
    }

    pub fn supports_hardware_instancing(&self) -> bool {
        unsafe { sys::FNA3D_SupportsHardwareInstancing(self.raw) == 0 }
    }

    pub fn supports_no_overwrite(&self) -> bool {
        unsafe { sys::FNA3D_SupportsNoOverwrite(self.raw) == 0 }
    }

    pub fn get_max_texture_slots(
        &mut self,
        // FIXME: this..
        textures: *mut i32,
        vertex_textures: *mut i32,
    ) {
        unsafe {
            sys::FNA3D_GetMaxTextureSlots(self.raw, textures, vertex_textures);
        }
    }

    pub fn get_max_multi_sample_count(
        &mut self,
        format: enums::SurfaceFormat,
        multi_sample_count: i32,
    ) -> i32 {
        unsafe { sys::FNA3D_GetMaxMultiSampleCount(self.raw, format as u32, multi_sample_count) }
    }

    // FIXME: C string wrapper?? I have to read Rust nomicon
    pub fn set_string_marker(&mut self, text: *const ::std::os::raw::c_char) {
        unsafe {
            sys::FNA3D_SetStringMarker(self.raw, text);
        }
    }
}
