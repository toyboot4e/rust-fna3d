use std::env;

fn main() {
    setup_lib_paths();
}

/// Absolute path string to the directory where `libFNA3D.dylib` is
fn fna3d_abs_path() -> String {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/../fna3d-sys/FNA3D/build", root)
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
