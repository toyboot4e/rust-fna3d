# Guide to making a wrapper of a C library

The follows explain what Rust-FNA3D takes care to wrap Rust-FNA3D-sys, which is Rust FFI
bindings to FNA3D generated with [`bindgen`](https://github.com/rust-lang/rust-bindgen).

## References

* [FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html#foreign-function-interface)
* [The (unofficial) Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
* [Using C libraries in Rust: make a sys crate](https://kornel.ski/rust-sys-crate)
* [Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-building-some-native-code)
* [The `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen/)

## 1. `build.rs`

Let's setup our build script and automate compling & bundling C libraries.

Here I only consider the case where we use `cmake`.

WIP

## 2. Wrapping constants

Let's get into examples.

### 2-1. Wrapping constants with an enum

NOTE: this is the case where we we use [`bindgen::Builder`](https://docs.rs/bindgen/newest/bindgen/struct.Builder.html) with default settings. If you want to know, the documentation tells us how to change the output.

Consider the constants as an example of `bindgen` output:

```rust
pub const FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT: FNA3D_IndexElementSize = 0;
pub const FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT: FNA3D_IndexElementSize = 1;
pub type FNA3D_IndexElementSize = u32;
```

We want to wram them with an `enum`:

```rust
use enum_primitive_derive::Primitive;
use fna3d_sys as sys;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum IndexElementSize {
    Bits16 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_16BIT,
    Bits32 = sys::FNA3D_IndexElementSize_FNA3D_INDEXELEMENTSIZE_32BIT,
}
```

We used [enum_primitive_derive](https://crates.io/crates/enum_primitive_derive) crate to implement `num_traits::FromPrimitive`.

Other option would be [num_derive](https://docs.rs/num-derive/latest/num_derive/) or [num_enum](https://docs.rs/num_enum/latest/num_enum/)

Now `IndexElementSize` can be created from u32:

```rust
use fna3d::{IndexElementSize, utils::FromPrimitive};
assert_eq!(IndexElementSize::from_u32(0), Some(IndexElementSize::Bits16));
```

* [bindgen #1096: Improve codegen for C style enums](https://github.com/rust-lang/rust-bindgen/issues/1096)

### 2-2. Wrapping bitflags

Consider the bitflag constants as an example of `bindgen` output:

```rust
pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET: FNA3D_ClearOptions = 1;
pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER: FNA3D_ClearOptions = 2;
pub const FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL: FNA3D_ClearOptions = 4;
pub type FNA3D_ClearOptions = u32;
```

We want to wrap them into a bitflags struct:

```rust
use fna3d_sys as sys;
bitflags::bitflags! {
    struct Flags: u32 {
        const Target = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_TARGET;
        const DepthBuffer = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_DEPTHBUFFER;
        const Stencil = sys::FNA3D_ClearOptions_FNA3D_CLEAROPTIONS_STENCIL;
    }
}
```

[bitflags](https://docs.rs/bitflags/newest/bitflags/) crate was used. The internal data can be got with `bits` method.

## 3. Wrapping a struct

### 3-1. Example data

Consider the `struct` as an example:

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FNA3D_DepthStencilState {
    // use boolean type to wrap this
    pub depthBufferEnable: u8,
    // ~~
    // use an enum type to wrap this:
    pub depthBufferFunction: FNA3D_CompareFunction,
    // ~~
}
```

Issues:

* Some fields are not strictly typed.
* It is a big `struct` but it implements `Copy`.

So let's make a wrapper of it. We'll start with this:

```rust
use fna3d_sys as sys;

#[derive(Debug, Clone)]
pub struct DepthStencilState {
    raw: sys::FNA3D_DepthStencilState,
}
```

Note that we hid the fields of the `struct`. This is an unfortunate cost to use such C structs actually.

### 3-2. Raw access

Before making interfaces, we may want to provide a way to access the inner content:

```rust
impl DepthStencilState {
    pub fn raw_mut(&mut self) -> &mut sys::FNA3D_DepthStencilState {
        &mut self.raw
    }
}
```

It's just for type conversions and not intended to provide with direct access to the fields. We'll make accessors to get or set them.

### 3-2. Accessors

1. [x] use snake case
2. [x] wrap enums, bit flags and booleans

```rust
impl DepthStencilState {
    // Use `bool` to wrap `u8`
    pub fn is_depth_buffer_enabled(&self) -> bool {
        self.raw.depthBufferEnable != 0
    }

    pub fn set_is_depth_buffer_enabled(&mut self, b: bool) {
        self.raw.depthBufferEnable = b as u8;
    }

    // Use an enum to wrap `FNA3D_CompareFunction` i.e. C enum binded as an alias of `u32`
    pub fn depth_buffer_function(&self) -> enums::CompareFunction {
        enums::CompareFunction::from_u32(self.raw.depthBufferFunction).unwrap()
    }

    pub fn set_depth_buffer_function(&mut self, f: enums::CompareFunction) {
        self.raw.depthBufferFunction = f as u32;
    }
}
```

3. [x] prefer `u32` to `i32` in some cases (e.g. indices) and cast it to `i32` using `as`
4. [x] casting types to `*mut T`:

We don't need mutability to get type `*mut T`:

* `*mut T` can be created from `*const T` (or directly from `&mut T` or `&mut [T]`)
* `*const T` can be created from `&T` or `&[T]`

So `value as *const _ as *mut _` is sufficient in most cases.

In reverse, `value: *mut T` can be casted to `&mut T` as this: `&mut *(value as *mut T)`.

### 3-3. Trait implementations

* `Debug`, `Clone`
* `Copy` if it's cheap
* `Default`
* `Hash`, `Eq`, `PartialEq` .. needed?
