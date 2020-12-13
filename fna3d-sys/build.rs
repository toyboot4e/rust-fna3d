//! Build script of `fna3d-sys`

// * TODO: support Windows
// * TODO: application bundle?

use {
    cmake::Config,
    std::{
        env,
        path::{Path, PathBuf},
    },
};

fn main() {
    // FIXME: somehow reruns too often?
    self::compile();
    self::gen_bindings("wrappers/fna3d_wrapper.h", "fna3d_bindings.rs");
    self::gen_bindings("wrappers/mojoshader_wrapper.h", "mojoshader_bindings.rs");
}

/// Add `mojoshader_version.h` to `FNA3D/MojoShader`
///
/// I'm not sure why we need it.
fn prepare() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // copy `mojoshader_version.h`
    use std::{fs, io::prelude::*};
    let src = fs::read("wrappers/mojoshader_version.h").unwrap();

    let p = root.join("FNA3D/MojoShader/mojoshader_version.h");
    if p.is_file() {
        // FIXME: should we unwrap
        match fs::read(&p) {
            Ok(content) if content == src => {}
            _ => return,
        }
    }

    // NOTE: we can't write on crates.io (since it's read-only)
    if let Ok(mut dst) = fs::File::create(&p) {
        // NOTE: this forces rebuilding `fna3d-sys`
        dst.write_all(&src).unwrap();
    }
}

/// Run `cmake` (only when it's necessary) and link the output library
fn compile() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // FNA3D
    let out_lib_path = out_dir.join("libFNA3D.dylib");
    if !out_lib_path.is_file() {
        let path = root.join("FNA3D");
        let _out = Config::new(path)
            .no_build_target(true)
            .cflag("-w") // suppress errors
            .cflag("-DMOJOSHADER_EFFECT_SUPPORT")
            .build();
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib=FNA3D");
}

/// Generates bindings using a wrapper header file
fn gen_bindings(wrapper: impl AsRef<Path>, dst_file_name: impl AsRef<Path>) {
    let wrapper = wrapper.as_ref();
    let dst_file_name = dst_file_name.as_ref();

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = root.join("src/ffi");
    let dst = out_dir.join(&dst_file_name);

    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper.display()))
        .derive_default(true)
        .clang_arg("-DMOJOSHADER_EFFECT_SUPPORT")
        .clang_arg(format!("-I{}", root.join("FNA3D/include").display()))
        .clang_arg(format!("-I{}", root.join("FNA3D/MojoShader").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap_or_else(|err| {
            panic!(
                "Unable to generate bindings for `{}`. Original error {:?}",
                dst_file_name.display(),
                err
            )
        });

    // it's `ok` to fail conidering crates.io
    bindings.write_to_file(&dst).ok();
}
