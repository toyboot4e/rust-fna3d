//! [fna3d] items in a hierarchy
//!
//! [fna3d]: https://docs.rs/fna3d/latest/fna3d/index.html
//!
//! Just a collection of re-exports.
//!
//! Unexported items: [`fna3d::linked_version`]

pub use fna3d;

pub use fna3d::{Color, Device, SurfaceFormat};

pub mod tex {
    //! Texture

    pub use fna3d::{CubeMapFace, Texture};
}

pub mod buf {
    //! GPU buffer
    //!
    //! TODO: provide with `derive` macro for vertices

    pub use fna3d::{Buffer, BufferUsage, SetDataOptions};

    pub use fna3d::{
        IndexElementSize, VertexDeclaration, VertexElement, VertexElementFormat, VertexElementUsage,
    };
}

pub mod draw {
    /*! Drawing

    # Rendering cycle

    As described in [`crate::Device`], one rendering pass is as follows:

    * [`Device::clear`]
    * for each draw call:
        * (set shader matrix)
        * apply pipeline
            * [`Device::apply_effect`] ([`mojo::Effect`])
            * [`Device::set_vertex_buffer_data`] ([`crate::buf`])
            * [`Device::verify_sampler`] ([`pip::SamplerState`])
        * [`Device::apply_vertex_buffer_bindings`] ([`VertexBufferBinding`])
        * [`Device::draw_indexed_primitives`]: (range of vertex buffer and index buffer)

    And call [`Device::swap_buffers`] at the end of a frame.

                */

    use crate::Device; // for docstring

    pub use fna3d::{PrimitiveType, VertexBufferBinding};

    pub use fna3d::Viewport;

    pub use fna3d::{Query, Rect};

    pub mod blend {
        //! Blending

        pub use fna3d::{Blend, BlendFunction, BlendState, ColorWriteChannels};
    }

    pub mod pip {
        //! Pipeline ([`SamplerState`] + [`DepthStencilState`] + [`RasterizerState`])

        pub use fna3d::{SamplerState, TextureAddressMode, TextureFilter};

        pub use fna3d::{CompareFunction, DepthStencilState, StencilOperation};

        pub use fna3d::{FillMode, RasterizerState};
    }

    pub mod pass {
        //! Rendering to frame buffer or offscreen (render target)
        //!
        //! [`crate::Device::clear`] is a pass action.

        pub use fna3d::{DepthFormat, Renderbuffer};

        pub use fna3d::{RenderTargetBinding, RenderTargetType, RenderTargetUsage};
    }

    pub mod mojo {
        //! MojoShader

        pub use fna3d::mojo::*;
    }
}

pub mod win {
    //! Window

    pub use fna3d::{DisplayOrientation, PresentInterval, PresentationParameters};

    pub use fna3d::{get_drawable_size, prepare_window_attributes, SdlWindowFlags};
}
