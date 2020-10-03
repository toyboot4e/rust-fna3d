//! Build script of `fna3d-sys`
//!
//! # What it does
//!
//! 1. pull FNA3D recursively if there's not
//! 2. apply patches to FNA3D and MojoShader
//! 3. run `cmake` and build MojoShader and FNA3D (if they're not built yet)
//! 4. link to the output libraries
//! 5. make bindings (FFI) to the C libraries
//!
//! # TODOs
//!
//! * TODO: Windows/Linux
//! * TODO: bundling libFNA3D.dylib with executable?
//! * TODO: static linking?
//! * FIXME: @rpath? (run executable without cargo after outputting)
//!
//! # Resources

use cmake::Config;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // get submodules ready
    pull_and_apply_patches();

    // compile MojoShader and FNA3D
    run_cmake();

    // make bindings to them
    self::gen_bindings("fna3d_wrapper.h", "fna3d_bindings.rs");
    self::gen_bindings("mojoshader_wrapper.h", "mojoshader_bindings.rs");

    // these files are statically included in src/lib.rs
}

fn pull_and_apply_patches() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(root);

    Command::new("git")
        .current_dir(&root)
        .args(&["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("is git in your PATH?");

    {
        // MojoShader
        let dir = root.join("FNA3D/MojoShader");
        let dir = dir.into_os_string().into_string().unwrap();
        let patch = root.join("mojoshader_patch.diff");
        let patch = patch.into_os_string().into_string().unwrap();
        apply_patch(&dir, &patch);
    }

    {
        // FNA3D
        let dir = root.join("FNA3D");
        let dir = dir.into_os_string().into_string().unwrap();
        let patch = root.join("fna3d_patch.diff");
        let patch = patch.into_os_string().into_string().unwrap();
        apply_patch(&dir, &patch);
    }
}

/// Checkouts to master then applies the patch
fn apply_patch(dir: &str, patch: &str) {
    // Command::new("git")
    //     .current_dir(dir)
    //     .args(&["checkout", "master"])
    //     .status()
    //     .unwrap_or_else(|e| {
    //         panic!(
    //             "failed to checkout master in dir `{}`. original error {}",
    //             dir, e
    //         )
    //     });

    println!("applying patch for {}", dir);

    Command::new("git")
        .current_dir(dir)
        .args(&["apply", patch])
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "failed to apply patch `{}` in dir `{}`. original error {}",
                patch, dir, e
            )
        });

    println!("=====> succeeded!");
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
    // TODO: does it work for Windows and Linux?
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
