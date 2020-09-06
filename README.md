# Rust-FNA3D

Wrapper of [FNA3D](https://github.com/FNA-XNA/FNA3D). It's for making a higher framework on it!

## About

Please refer to [API documentation](https://docs.rs/rust-fna3d).

As an example, [ANF](https://github.com/toyboot4e/anf) is a higher-level 2D framework built on top of Rust-FNA3D.

### Getting started

You need to add dependency to `rust-fna3d` in your project. If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed, you can do this:

```sh
$ cargo add rust-fna3d
```

Add when you build your project, FNA3D will automatically be built and bundled to your output. Now you can use the [fna3d](https://docs.rs/rust-fna3d) module!

### Tips

* Use nightly version of `cargo doc` to build the document. It's for th e[Infra Rustdoc Links](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html) feature. Crates.io also uses it by default.

## State of this wrapper

Almost ready. Remaining tasks:

* [ ] Add more wrapper types rather than re-exporting raw types
* [ ] Add more methods to wrapper types
* [ ] `derive` more types

## Contact

Free free to contact with me. I love _any_ kind of improvements and anything will be welcomed!

## References

### Other repositories using FNA3D

Repositories using latest version of FNA3D:

* [Simple texture rendering in FNA3D](https://gist.github.com/jessechounard/d4252efc12ee24494484611d92b1debe) \(C gist)
* [Stone Tower Engine](https://github.com/silenttowergames/stonetowerengine) \(C)

Repositories using older version of FNA3D:

* [BNA](https://github.com/KillaMaaki/BNA) ([Beef](https://www.beeflang.org/))
* [Odin-Libs](https://github.com/prime31/Odin-Libs) ([Odin](https://odin-lang.org/))
* [Via](https://github.com/prime31/via) ([V](https://vlang.io/))

### Other C graphics libraries you could be interested in

* [Sokol](https://github.com/floooh/sokol)
A minimal cross-platform standalone C headers

