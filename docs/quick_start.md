# Quick start

## Adding Rust-FNA3D as dependency

If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed, run `cargo add` to add dependency to Rust-FNA3D:

```sh
$ cargo add rust-fna3d
```

Or see [crates.io](https://crates.io/crates/rust-fna3d) and add the `rust-fna3d` crate as your dependency:

```toml
# Cargo.toml
[dependency]
rust-fna3d = "<put the latest version here>"
```

You can also take the git repo:

```toml
[dependency]
rust-fna3d = { git = "https://github.com/toyboot4e/rust-fna3d" }
# rust-fna3d = { git = "https://github.com/toyboot4e/rust-fna3d", rev = "<commit hash>" }
```

After adding the dependency, when you build your project, FNA3D will automatically be built and bundled to your output. Now you can use the [fna3d](https://docs.rs/rust-fna3d) module!

## More examples

* [ANF](https://github.com/toyboot4e/anf) framework
* [Simple texture rendering in FNA3D](https://gist.github.com/jessechounard/d4252efc12ee24494484611d92b1debe) \(C gist)
