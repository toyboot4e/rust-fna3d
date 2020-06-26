//! Build script that generates Rust FFI bindings to FNA3D using `bindgen`
//!
//! # Resources
//!
//! * [The `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen/)
//! * [Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-building-some-native-code)

use std::{
    env,
    path::{Path, PathBuf},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    // link `libFNA3D.dylib` from absolute path (TODO: bundle FNA on release build)
    self::setup_lib_paths();

    println!("cargo:rustc-link-lib=dylib=FNA3D");

    self::gen_bindings("fna3d_wrapper.h", "fna3d_bindings.rs");

    Ok(())
}

/// Absolute path string to the directory where `libFNA3D.dylib` is
fn fna3d_abs_path() -> String {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/FNA3D/build", root)
}

/// Somehow this is required on macOS
///
/// If this function is skipped, such an error occurs:
///
/// ```
/// ld: Library not loaded: @rpath/libFNA3D.0.dylib
///   Referenced from: /Users/toy/dev/rs/rust-fna3d/target/debug/deps/fna3d_sys-5727a581b25bfeea
///   Reason: image not found
/// ```
fn setup_lib_paths() {
    let fna = self::fna3d_abs_path();
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", fna);
    println!("cargo:rustc-env=LIBRARY_PATH={}", fna);
    // we don't need this?
    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", fna);

    // println!("cargo:rustc-link-search=native={}", fna);
}

/// Generates bindings using a wrapper header file
fn gen_bindings(wrapper: impl AsRef<Path>, dest_file_name: impl AsRef<Path>) {
    let wrapper = wrapper.as_ref();
    let dest = {
        // You may want to know about `OUT_DIR`:
        // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
        let root = PathBuf::from(env::var("OUT_DIR").unwrap());
        root.join(dest_file_name)
    };

    println!("cargo:rerun-if-changed={}", wrapper.display());
    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&dest)
        .expect("Couldn't write bindings!");
}
