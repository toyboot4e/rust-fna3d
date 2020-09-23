//! Build script of `fna3d-sys`
//!
//! * WARN: This script is for macOS only for now
//! * WARN: This script requires some manual patching `CMakeList.txt` files
//!
//! # What it does
//!
//! 1. TODO: pull FNA3D recursively if there's not
//! 2. TODO: apply patches to FNA3D
//! 3. run `cmake` and build MojoShader and FNA3D (only when they're not built yet)
//!     * TODO: (consider Linux and Windows build: file names and library types?)
//! 4. link to the output libraries
//! 5. make bindings (FFI) to the C libraries
//!
//! * TODO: Vulkan headers?
//! * TODO: enalbe release build and stand-alone app?
//! * TODO: remove `CMakeCache.txt` automatically (or `Error: could not load cache`)
//! * FIXME: @rpath? (run executable without cargo after outputting)
//!
//! # Resources
//!
//! * Using C libraries in Rust: make a sys crate
//! https://kornel.ski/rust-sys-crate
//!
//! * Build Scripts - The Cargo Book
//! https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-building-some-native-code
//!
//! * The `bindgen` User Guide
//! https://rust-lang.github.io/rust-bindgen/
//!
//! * `build.rs` in Rust-SDL2
//! https://github.com/Rust-SDL2/rust-sdl2/blob/master/sdl2-sys/build.rs

use cmake::Config;
use std::{
    env,
    path::{Path, PathBuf},
};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // compile MojoShader and FNA3D
    run_cmake();

    // make bindings to them
    self::gen_bindings("fna3d_wrapper.h", "fna3d_bindings.rs");
    self::gen_bindings("mojoshader_wrapper.h", "mojoshader_bindings.rs");
    // these files are "included" in src/lib.rs
}

/// Runs `cmake` (only when it's necessary) and links the output libraries
fn run_cmake() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(root);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // MojoShader
    let out_file_path = out_dir.join("libmojoshader.a");
    if !out_file_path.is_file() {
        let path = root.join("FNA3D/MojoShader");
        let _out = Config::new(path)
            .cflag("-DMOJOSHADER_EFFECT_SUPPORT")
            .build();
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=mojoshader");

    // FNA3D
    let out_file_path = out_dir.join("libFNA3D.dylib");
    if !out_file_path.is_file() {
        let path = root.join("FNA3D");
        let _out = Config::new(path)
            .cflag("-DMOJOSHADER_EFFECT_SUPPORT")
            .build();
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=FNA3D");
}

/// Generates bindings using a wrapper header file
fn gen_bindings(wrapper_path: impl AsRef<Path>, dest_file_name: impl AsRef<Path>) {
    let wrapper = wrapper_path.as_ref();
    let dest_file_name = dest_file_name.as_ref();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join(&dest_file_name);

    println!("cargo:rerun-if-changed={}", wrapper.display());
    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper.display()))
        // SUPPORT MOJOSHADER EFFECT (only needed when building MojoShader)
        .clang_arg("-DMOJOSHADER_EFFECT_SUPPORT")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap_or_else(|_| {
            panic!(
                "Unable to generate bindings for {}",
                dest_file_name.display()
            )
        });

    bindings
        .write_to_file(&dest)
        .unwrap_or_else(|_| panic!("Couldn't write bindings for {}", dest_file_name.display()));
}
