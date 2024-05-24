Voxel Garden
============

Compilation Instructions
------------------------

You only need to install Rust and its build system Cargo. We have developed on
Arch Linux, where everything we need is available in the `rust` package.
Lately, more distributions have started packaging Rust (and sometimes Cargo in
a separate package), but as they may be out of date (sometimes severly), using
https://rustup.rs (which installs everything for your local user) is probably
easiest. Follow the instructions on https://rustup.rs.

With Rust and Cargo installed, `cargo run --release --locked` should be enough
to compile and run. `--release` is equivalent to `-O2` and `--locked` tells
Cargo to use the `Cargo.lock` file which tracks the exact version of
dependencies used during development.

We have also built and distribute the following:

- Statically linked Linux binary.
- Cross-compiled `x86_64-windows-pc-gnu` binary, which should work on Windows.

Usage Instructions
------------------

TODO

Code Structure
--------------

This section will quickly mention the structure of the code, i.e. what each file and module contains.

camera.rs
main.rs
models/
models/biomes.rs
models/flower.rs
models/mod.rs
models/primitives.rs
models/rock.rs
models/terrain.rs
models/tree.rs
shader/
shader/mod.rs
shader/shader.frag
shader/shader.vert
utils.rs
