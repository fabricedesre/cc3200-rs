Rust on the cc3200
==================
[![Build Status](https://travis-ci.org/fabricedesre/cc3200-rs.svg?branch=master)](https://travis-ci.org/fabricedesre/cc3200-rs)

Prerequisites
=============
- Install rust nightly from https://rustup.rs/
- Install `xargo` with `cargo install xargo`
- Install openocd (for instance the default 0.9.0 version from Ubuntu)

Building, etc.
==============
Build with `./build.sh`

Load on the board to debug with `./run.sh`

Flash with `./flash.sh`

Note that flashing requires the use of cc3200tool, which can be installed by following
the README [here](https://github.com/ALLTERCO/cc3200tool)
