# Building FNA3D/MojoShader

Please tell me when you find missing dependencies to install.

## macOS

SDL2:

```sh
$ brew install sdl
```

Maybe we need XCode tools..?

## Linux (Ubuntu)

SDL2, build tools, and compilers:

```sh
$ sudo apt install \
    sdl2-devel \
    cmake gcc clang \
    install gobjc gnustep gnustep-devel
```

Probablly it contains missing/redundant libraries, but it lists most of the required packages.

## Windows

WIP. I first have to get some Windows machine.
