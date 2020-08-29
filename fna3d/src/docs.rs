//! Explains how to make a C wrapper
//!
//! Bundling C binary is out of scope of this document.
//!
//! # Guide to making a wrapper for C
//!
//! The follows explain what Rust-FNA3D take care to wrap Rust-FNA3D-sys, which is Rust FFI
//! bindings to FNA3D generated with `bindgen`.
//!
//! ## References
//!
//! * [FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html#foreign-function-interface)
//! * [The (unofficial) Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
//!
//! ## 1. Output of `bindgen`
//!
//! We need to wrap the output of `bindgen` using rusty types to provide with a cozy interface.
//!
//! ### 1-1 enums, bit flags and booleans
//!
//! Since C is loosly typed, `bindgen` translates `enum` s as `u32` and `bool` s as `u8`. But they
//! should be accessed via `SomeEnumType` or `bool`. So we wrap `bindgen` functions with ours.
//!
//! I'm using these crates:
//!
//! * `enum_primitive`: to generate `enum` s from primitive values
//! * `bitflags`: for making bit flags
//!
//! ### 1-2 Zero-sized structs
//!
//! They are used to represent a pointer type. Example:
//!
//! ```
//! #[repr(C)]
//! #[derive(Debug, Copy, Clone)]
//! pub struct FNA3D_Device {
//!     _unused: [u8; 0],
//! }
//! ```
//!
//! ### 1-3. `*c_void`
//!
//! It is used to represent e.g. a function pointer. There's a [corresponding page] in Rust
//! nomicon.
//!
//! [corresponding page]: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
//!
//! ### 1-4. structs
//!
//! They are `Copy` be default and it's unfortunate if the C `struct` is big. Also, as I mentioned
//! in `1-2`, some types of fields are loosely typed. So I wrapped C structs with another (which may
//! not be `Copy`) and provided with accessor methods of each field.
//!
//! This is a lot of work and ridiculous. Maybe macros could be used (though I didn't...)
//!
//! You may not need wrap C structs in such a way, especially when they are used in internals and
//! hidden under rusty APIs in high level crates.
//!
//! ## 2. Wrapping constants
//!
//! Let's get into examples.
//!
//! ### 2-1. Wrapping constants to an enum
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
//! [enum_primitive](https://crates.io/crates/enum_primitive) crate was used to implement
//! `IndexElementSize::from_u32` automatically. TODO: is this way up to date?s
//!
//! References:
//!
//! > * [Wrapping Unsafe C Libraries in Rust - Dwelo Research and Development - Medium](https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65)
//! > * [bindgen #1096: Improve codegen for C style enums](https://github.com/rust-lang/rust-bindgen/issues/1096)
//!
//! ### 2-2. Wrapping bit flags
//!
//! NOTE: this is in case where we don't use the `rustified-enum` option of `bindgen`.
//!
//! Consider the constants as an example:
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
//! Some fields are not strictly typed. Also, while it is a big `struct`, it implements `Copy`.
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
//! Unfortunately, fields of `FNA3D_DepthStencilState` are now hidden.
//!
//! ### 3-2. Raw access
//!
//! Before making interfaces, we may want to provide a way to access the inner content:
//!
//! ```ignore
//! impl DepthStencilState {
//!     pub fn raw(&mut self) -> sys::FNA3D_DepthStencilState {
//!         &mut self.raw
//!     }
//! }
//! ```
//!
//! It's just for type conversions and we'll make accessors to get or modify the fields.
//!
//! ### 3-2. Accessors
//!
//! * [x] use snake case
//! * [x] wrap enums, bit flags and booleans
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
//! * [ ] wrap function pointers?
//! * [ ] take care to ownership?
//!
//! ### 3-3. Trait implementations
//!
//! * [x] `Debug`, `Clone`
//! * [x] `Default`
//!
//! ### 3-4. Mutability
//!
//! `T*` defined in C is translated to `*mut T` by `bindgen`. `*mut T` can only be create from
//! `&mut T`, not from `&T`. However, you may want not to take mutability because it's not mutated
//! in the C source code.
//!
//! So how can we avoid to take mutability?
//!
//! 1. Make a clone of the value and then take a mutable reference of it
//! 2. Modify the output of bindgen to take `*const T`
//!
//! The method `2.` is preferrable.
//!
//! ### 3-5. Other changes
//!
//! `*mut void` and `int` -> `&[u8]`
