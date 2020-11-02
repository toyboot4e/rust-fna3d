use {
    anyhow::{Error, Result},
    std::mem,
};

use super::gfx::{Shader2d, Vertex};

#[derive(Debug, Clone, Default)]
pub struct QuadData(pub [Vertex; 4]);
pub const N_QUADS: u32 = 2048;

#[derive(Debug)]
pub struct Batch {
    device: fna3d::Device,
    vbuf: *mut fna3d::Buffer,
    vbind: fna3d::VertexBufferBinding,
    ibuf: *mut fna3d::Buffer,
    quads: Vec<QuadData>,
    /// The number of quads stored in this batch
    n_quads: usize,
    track: Vec<*mut fna3d::Texture>,
}

impl Drop for Batch {
    fn drop(&mut self) {
        self.device.add_dispose_vertex_buffer(self.vbuf);
        self.device.add_dispose_index_buffer(self.ibuf);
    }
}

/// Creates index buffer for quadliterals
macro_rules! gen_quad_indices {
    ( $n_quads:expr ) => {{
        let mut indices = [0; 6 * $n_quads as usize];

        for n in 0..$n_quads as i16 {
            let (i, v) = (n * 6, n * 4);
            indices[i as usize] = v as i16;
            indices[(i + 1) as usize] = v + 1 as i16;
            indices[(i + 2) as usize] = v + 2 as i16;
            indices[(i + 3) as usize] = v + 3 as i16;
            indices[(i + 4) as usize] = v + 2 as i16;
            indices[(i + 5) as usize] = v + 1 as i16;
        }

        indices
    }};
}

impl Batch {
    pub fn new(device: &fna3d::Device) -> Result<Self> {
        // GPU vertex buffer (marked as "dynamic")
        let n_verts = N_QUADS * 4;
        let vbuf = device.gen_vertex_buffer(
            true,
            fna3d::BufferUsage::None,
            n_verts * mem::size_of::<Vertex>() as u32,
        );

        // GPU index buffer (marked as "static")
        let ibuf = device.gen_index_buffer(false, fna3d::BufferUsage::None, 16 * n_verts);
        {
            let data = gen_quad_indices!(N_QUADS);
            device.set_index_buffer_data(ibuf, 0, &data, fna3d::SetDataOptions::None);
        }

        // vertex attributes
        // META: such types are just re-exported from FFI to FNA3D and don't have snake_case fields
        let vbind = fna3d::VertexBufferBinding {
            vertexBuffer: vbuf,
            vertexDeclaration: Vertex::DECLARATION,
            vertexOffset: 0,
            instanceFrequency: 0,
        };

        let quads = vec![QuadData::default(); N_QUADS as usize];
        let track = vec![std::ptr::null_mut(); N_QUADS as usize];

        Ok(Self {
            device: device.clone(),
            vbuf,
            vbind,
            ibuf,
            quads,
            n_quads: 0,
            track,
        })
    }

    /// Make sure the [`Batch`] is not yet satured
    pub unsafe fn push_quad(&mut self, quad: &QuadData, tex: *mut fna3d::Texture) {
        self.quads[self.n_quads] = quad.clone();
        self.track[self.n_quads] = tex;
        self.n_quads += 1;
    }

    pub fn iter(&self) -> DrawCallIterator {
        DrawCallIterator::from_batch(self)
    }
}

/// Quad index [lo, hi) and texture
#[derive(Debug)]
pub struct DrawCall {
    pub texture: *mut fna3d::Texture,
    /// low (inclusive)
    pub lo: usize,
    /// high (exclusive)
    pub hi: usize,
}

impl DrawCall {
    pub fn n_quads(&self) -> usize {
        self.hi - self.lo
    }

    pub fn n_verts(&self) -> usize {
        4 * self.n_quads()
    }

    pub fn n_indices(&self) -> usize {
        6 * self.n_quads()
    }

    pub fn n_triangles(&self) -> usize {
        2 * self.n_quads()
    }

    pub fn base_vtx(&self) -> usize {
        4 * self.lo
    }

    pub fn base_idx(&self) -> usize {
        6 * self.lo
    }
}

pub struct DrawCallIterator<'a> {
    batch: &'a Batch,
    /// Index of next quad
    ix: usize,
}

impl<'a> DrawCallIterator<'a> {
    pub fn from_batch(batch: &'a Batch) -> Self {
        Self { batch, ix: 0 }
    }
}

impl<'a> Iterator for DrawCallIterator<'a> {
    type Item = DrawCall;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix >= self.batch.n_quads {
            return None;
        }

        let lo = self.ix;
        let texture = self.batch.track[lo];

        for hi in lo..self.batch.n_quads {
            let new_texture = self.batch.track[hi];
            if new_texture != texture {
                self.ix = hi;
                return Some(DrawCall { lo, hi, texture });
            }
        }

        let hi = self.batch.n_quads;
        self.ix = hi;
        return Some(DrawCall { lo, hi, texture });
    }
}

pub struct Batcher {
    batch: Batch,
    shader: Shader2d,
}

impl Batcher {
    pub fn new(device: &fna3d::Device, shader: Shader2d) -> Result<Self> {
        Ok(Self {
            batch: Batch::new(device)?,
            shader,
        })
    }

    pub fn next_quad_mut(&mut self) -> &mut QuadData {
        self.flush_if_satured();

        let quad = &mut self.batch.quads[self.batch.n_quads];
        self.batch.n_quads += 1;
        quad
    }

    pub fn push_quad(&mut self, quad: &QuadData, tex: *mut fna3d::Texture) {
        self.flush_if_satured();

        unsafe {
            self.batch.push_quad(quad, tex);
        }
    }

    fn flush_if_satured(&mut self) {
        if self.batch.n_quads >= self.batch.quads.len() {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        if self.batch.n_quads == 0 {
            return;
        }

        // upload the CPU vertices to the GPU vertices (we don't have to do it every frame in though)
        {
            let offset = 0;
            self.batch.device.set_vertex_buffer_data(
                self.batch.vbuf,
                offset,
                &self.batch.quads[0..self.batch.n_quads],
                fna3d::SetDataOptions::None,
            );
        }

        for call in self.batch.iter() {
            log::trace!("draw call: {:?}", call);
            self.draw(&call);
        }

        self.batch.n_quads = 0;
    }

    fn draw(&self, call: &DrawCall) {
        let device = &self.batch.device;

        self.shader.apply_to_device();

        {
            let sst = fna3d::SamplerState::default();
            let slot = 0;
            device.verify_sampler(slot, call.texture, &sst);
        }

        {
            device.apply_vertex_buffer_bindings(&[self.batch.vbind], true, call.base_vtx() as u32);

            device.draw_indexed_primitives(
                fna3d::PrimitiveType::TriangleList,
                call.base_vtx() as u32,
                0,
                call.n_verts() as u32,
                call.base_idx() as u32,
                call.n_triangles() as u32,
                self.batch.ibuf,
                fna3d::IndexElementSize::Bits16,
            );
        }
    }
}
