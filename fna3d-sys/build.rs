/*! Build script of `fna3d-sys`

If the compilation fails, run `cargo clean`.

# What it does

1. Applies patches to FNA3D and MojoShader
2. Compiles MojoShader and FNA3D if they're not found in `OUT_DIR`
3. Links to the output libraries
4. Makes bindings (FFI) to the C libraries

# TODOs

* TODO: support Windows
* TODO: how to publish executable with dynamic libraries (application bundle)?
*/

use {
    cmake::Config,
    std::{
        env,
        path::{Path, PathBuf},
        process::Command,
    },
};

fn main() {
    // FIXME: somehow rerun too much
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", root.join("wrappers").display());
    // when we update `FNA3D`, we have to manually rebuild!

    self::prepare();
    self::compile();
    self::gen_bindings("wrappers/fna3d_wrapper.h", "fna3d_bindings.rs");
    self::gen_bindings("wrappers/mojoshader_wrapper.h", "mojoshader_bindings.rs");
}

/// Pulls FNA3D and applies patches
fn prepare() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let dir = root.join("FNA3D");
    let patch = root.join("wrappers/fna3d_patch.diff");
    apply_patch(&dir, &patch);

    // copy `mojoshader_version.h`
    use std::{fs, io::prelude::*};
    let src = fs::read("wrappers/mojoshader_version.h").unwrap();
    // consider `crates.io` (read-only)
    if let Ok(mut dst) = fs::File::create(root.join("FNA3D/MojoShader/mojoshader_version.h")) {
        dst.write_all(&src).unwrap();
    }

    fn apply_patch(dir: &Path, patch: &Path) {
        let patch = format!("{}", patch.display());

        Command::new("git")
            .current_dir(dir)
            .args(&["apply", &patch])
            // suppress patch error
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap_or_else(|e| {
                panic!(
                    "failed to apply patch `{}` in dir `{}`. original error {}",
                    patch,
                    dir.display(),
                    e
                )
            });
    }
}

/// Runs `cmake` (only when it's necessary) and links the output libraries
fn compile() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // FNA3D
    let out_lib_path = out_dir.join("libFNA3D.dylib");
    if !out_lib_path.is_file() {
        let path = root.join("FNA3D");
        let _out = Config::new(path)
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

    bindings.write_to_file(&dst).unwrap_or_else(|err| {
        panic!(
            "Couldn't write bindings for {}. Original error {}",
            dst_file_name.display(),
            err
        )
    });
}
