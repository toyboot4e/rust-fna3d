# Quick start

## Adding Rust-FNA3D as dependency

See [crates.io](https://crates.io/crates/rust-fna3d) and add the `rust-fna3d` crates as your dependency in Cargo.toml:

```toml
[dependency]
rust-fna3d = "<put the latest version here>"
```

If you want, you can take the git repo:

```toml
[dependency]
rust-fna3d = { git = "https://github.com/toyboot4e/rust-fna3d" }
# rust-fna3d = { git = "https://github.com/toyboot4e/rust-fna3d", rev = "<commit hash>" }
```

When you build your project, FNA3D will automatically be built and bundled to your output. Now you can use the [fna3d](https://docs.rs/rust-fna3d) module!

## More examples

* [ANF](https://github.com/toyboot4e/anf) framework
* [Simple texture rendering in FNA3D](https://gist.github.com/jessechounard/d4252efc12ee24494484611d92b1debe) \(C gist)
