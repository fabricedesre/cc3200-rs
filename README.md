Rust on the cc3200
==================
The sample code is from http://japaric.github.io/copper/

Prerequisites
=============
- Install rust nightly from https://rustup.rs/
- Install `xargo` with `cargo install xargo`
- Install openocd version *0.7.0*

Building
========
Build with `xargo build --target cortex-m4`

Load on the board to debug with `arm-none-eabi-gdb -x gdbinit ./target/cortex-m4/debug/firmware`
