//! Build script
//!
//! # Resources
//!
//! * Build Scripts - The Cargo Book
//! https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-building-some-native-code
//!
//! * The `bindgen` User Guide
//! https://rust-lang.github.io/rust-bindgen/

use cmake::Config;
use std::{
    env,
    path::{Path, PathBuf},
};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // FIXME: run cmake only when FNA3D is changed
    // compile MojoShader and FNA3D
    run_cmake();

    // make bindings to them
    self::gen_bindings("fna3d_wrapper.h", "fna3d_bindings.rs");
    self::gen_bindings("mojoshader_wrapper.h", "mojoshader_bindings.rs");
    // these files are "included" in src/lib.rs
}

fn run_cmake() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(root);

    {
        let path = root.join("FNA3D/MojoShader");
        // env::set_current_dir(path).unwrap();
        let out = Config::new(path)
            .cflag("-DMOJOSHADER_EFFECT_SUPPORT")
            .build();

        // let name = out.file_stem().unwrap().to_str().unwrap();
        let name = out.display();
        println!("cargo:rustc-link-search=native={}", name);
        println!("cargo:rustc-link-lib=static=mojoshader");
    }

    {
        let path = root.join("FNA3D");
        // env::set_current_dir(path).unwrap();
        let out = Config::new(path)
            .cflag("-DMOJOSHADER_EFFECT_SUPPORT")
            .build();

        // let name = out.file_stem().unwrap().to_str().unwrap();
        let name = out.display();
        println!("cargo:rustc-link-search=native={}", name);
        // FIXME: why is this OK??
        println!("cargo:rustc-link-lib=dylib=FNA3D");
    }
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
        // SUPPORT MOJOSHADER EFFECT
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
