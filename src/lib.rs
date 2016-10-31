// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#![feature(asm, lang_items)]

pub mod cc3200;
pub mod isr_vectors;

// We need to define the panic_fmt "lang item", which is just a function. This specifies
// what the program should do when a `panic!` occurs. Our program won't panic, so we can leave the
// function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
