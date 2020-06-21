//! Wraps unsafe ffi functions into safe Rust types

use fna3d_sys as sys;

// from line 2301
pub fn linked_version() -> u32 {
    unsafe { sys::FNA3D_LinkedVersion() }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_version() {
        println!("{}", super::linked_version());
    }
}
