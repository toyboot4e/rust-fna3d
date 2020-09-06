//! Notes, not Rust-FNA3D itself
//!
//! # Tips for using `fna3d`
//!
//! If you're new to graphics:
//!
//! * You may want to know about rendering pipeline to use FNA3D. That can be learned by reading
//!   some tutorial on a specific low-level graphics API. One example is [learnopengl.com];
//!   it's a good read and although OpenGL is an old API, it still maps well to FNA3D or other
//!   APIs.
//!
//! [learnopengl.com]: https://learnopengl.com/
//!
//! * You may want to structure multiple structs in FNA3D into one. For example, FNA3D doesn't have
//!   resource binding struct or pipeline object as [Sokol] does. [`miniquad`], which is inspired
//!   by Sokol, can also be a good learning resource.
//!
//! [Sokol]: https://github.com/floooh/sokol/blob/master/sokol_gfx.h
//! [`miniquad`]: https://docs.rs/miniquad/
//!
//! # Guide to making a wrapper of a C library
//!
//! The follows explain what Rust-FNA3D takes care to wrap Rust-FNA3D-sys, which is Rust FFI
//! bindings to FNA3D generated with [`bindgen`](https://github.com/rust-lang/rust-bindgen).
//!
//! ## References
//!
//! * [FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html#foreign-function-interface)
//! * [The (unofficial) Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
//!
//! ## 1. `build.rs`
//!
//! Let's setup our build script and automate compling & bundling C libraries.
//!
//! Here I only consider the case where we use `cmake`.
//!
//! WIP
//!
//! ## 2. Wrapping constants
//!
//! Let's get into examples.
//!
//! ### 2-1. Wrapping constants to an enum
//!
//! NOTE: this is the case where we don't use the `rustified-enum` option of `bindgen`.
//!
//! Consider the constants as an example:
//!
//! ```
//! pub const FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT: FNA3D_IndexElementSize = 0;
//! pub const FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT: FNA3D_IndexElementSize = 1;
//! pub type FNA3D_IndexElementSize = u32;
//! ```
//!
//! We want to wram them with an `enum`:
//!
//! ```
//! use enum_primitive::*;
//! use fna3d_sys as sys;
//! enum_from_primitive! {
//!     #[derive(Debug, Copy, Clone, PartialEq)]
//!     #[repr(u32)]
//!     pub enum IndexElementSize {
//!         Bits16 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT,
//!         Bits32 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT,
//!     }
//! }
//! ```
//!
//! Now `IndexElementSize` can be created from u32:
//!
//! ```no_run
//! use fna3d::{IndexElementSize, utils::FromPrimitive};
//! assert_eq!(IndexElementSize::from_u32(0), Some(IndexElementSize::Bits16));
//! ```
//!
//! [enum_primitive](https://crates.io/crates/enum_primitive) crate was used to implement
//! `IndexElementSize::from_u32` automatically.
//!
//! * TODO: use derive macro for it. or can I use `num` crate?
//! * [bindgen #1096: Improve codegen for C style enums](https://github.com/rust-lang/rust-bindgen/issues/1096)
//!
//! ### 2-2. Wrapping bitflags
//!
//! Consider the bitflag constants as an example:
//!
//! ```
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET: FNA3D_ClearOptions = 1;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER: FNA3D_ClearOptions = 2;
//! pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL: FNA3D_ClearOptions = 4;
//! pub type FNA3D_ClearOptions = u32;
//! ```
//!
//! We want to wrap them into a bitflags struct:
//!
//! ```
//! use fna3d_sys as sys;
//! bitflags::bitflags! {
//!     struct Flags: u32 {
//!         const Target = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET;
//!         const DepthBuffer = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER;
//!         const Stencil = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL;
//!     }
//! }
//! ```
//!
//! [bitflags](https://docs.rs/bitflags/1.2.1/bitflags/) crate was used. The internal data can
//! be got with `bits` method.
//!
//! ## 3. Wrapping a struct
//!
//! ### 3-1. Example data
//!
//! Consider the `struct` as an example:
//!
//! ```ignore
//! #[repr(C)]
//! #[derive(Debug, Copy, Clone)]
//! pub struct FNA3D_DepthStencilState {
//!     // use boolean type to wrap this
//!     pub depthBufferEnable: u8,
//!     // ~~
//!     // use an enum type to wrap this:
//!     pub depthBufferFunction: FNA3D_CompareFunction,
//!     // ~~
//! }
//! ```
//!
//! Issues:
//!
//! * Some fields are not strictly typed.
//! * It is a big `struct` but it implements `Copy`.
//!
//! So let's make a wrapper of it. We'll start with this:
//!
//! ```
//! use fna3d_sys as sys;
//!
//! #[derive(Debug, Clone)]
//! pub struct DepthStencilState {
//!     raw: sys::FNA3D_DepthStencilState,
//! }
//! ```
//!
//! Note that we hid the fields of the `struct`. This is an unfortunate cost to use such C structs
//! actually.
//!
//! ### 3-2. Raw access
//!
//! Before making interfaces, we may want to provide a way to access the inner content:
//!
//! ```ignore
//! impl DepthStencilState {
//!     pub fn raw(&mut self) -> &mut sys::FNA3D_DepthStencilState {
//!         &mut self.raw
//!     }
//! }
//! ```
//!
//! It's just for type conversions and not intended to provide with direct access to the fields.
//! We'll make accessors to get or set them.
//!
//! ### 3-2. Accessors
//!
//! * use snake case
//! * wrap enums, bit flags and booleans
//! * prefer `u32` to `i32` in some cases (e.g. indices)
//!
//! ```ignore
//! impl DepthStencilState {
//!     // Use `bool` to wrap `u8`
//!     pub fn is_depth_buffer_enabled(&self) -> bool {
//!         self.raw.depthBufferEnable != 0
//!     }
//!
//!     pub fn set_is_depth_buffer_enabled(&mut self, b: bool) {
//!         self.raw.depthBufferEnable = b as u8;
//!     }
//!
//!     // Use `enums::CompareFunction` to wrap `FNA3D_CompareFunction` i.e. `u32`
//!     pub fn depth_buffer_function(&self) -> enums::CompareFunction {
//!         enums::CompareFunction::from_u32(self.raw.depthBufferFunction).unwrap()
//!     }
//!
//!     pub fn set_depth_buffer_function(&mut self, f: enums::CompareFunction) {
//!         self.raw.depthBufferFunction = f as u32;
//!     }
//! }
//! ```
//!
//! * casting to `*mut T`
//!
//! We don't need mutability to get type `*mut T`:
//!
//! * `*mut T` can be casted from `*const T` (or directly from `&mut T` or `&mut [T]`)
//! * `*const T` can be casted from `&T` or `&[T]`
//!
//! So `value as *const _ as *mut _` is sufficient.
//!
//! In reverse, `value: *mut T` can be casted to `&mut T` as this: `&mut *(value as *mut T)`.
//!
//! ### 3-3. Trait implementations
//!
//! * `Debug`, `Clone`
//! * `Default`
