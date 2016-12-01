// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#![feature(asm, lang_items)]
// For i2c_devices pow functions.
#![feature(core_intrinsics)]
#![feature(collections)]
#![feature(try_from)]

// #![feature(compiler_builtins_lib)]
// extern crate compiler_builtins;

extern crate cc3200_sys;
#[macro_use]
extern crate log;

#[macro_use]
extern crate collections;

extern crate freertos_rs;
#[macro_use]
extern crate lazy_static;
extern crate smallhttp;

#[macro_use]
pub mod logger;
pub mod cc3200;
pub mod i2c_devices;
pub mod isr_vectors;
pub mod format;
pub mod time;

pub mod bma222;
pub mod tmp006;
pub mod rtc;
pub mod simplelink;
pub mod socket_channel;

// We need to make sure that we pull in soft float versions of libm.a, libc.a
// and libgcc.a. The build.rs sets up the paths needed for these.
#[link(name = "m")]
extern "C" {} // for pow
#[link(name = "c")]
extern "C" {} // for __erno
#[link(name = "gcc")]
extern "C" {}

// These functions are used by the compiler, but are normally provided by libstd.
#[allow(private_no_mangle_fns)]
mod lang_items {
    use core::fmt::Arguments;
    use cc3200::{Board, LedEnum, LedName, Utils};

    #[lang = "eh_personality"]
    #[no_mangle]
    pub extern "C" fn rust_eh_personality() {}

    // This function may be needed based on the compilation target.
    #[lang = "eh_unwind_resume"]
    #[no_mangle]
    pub extern "C" fn rust_eh_unwind_resume() {}

    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn rust_begin_panic(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
        println!("Panic at {}:{} : {}", _file, _line, _msg);

        // Disable irqs.
        Board::disable_irq();

        // Configure the LEDs in case it's not done by the application and blink them.
        Board::led_configure(&[LedEnum::LED1, LedEnum::LED2, LedEnum::LED3]);
        for _ in 0..4 {
            Board::led_off(LedName::MCU_RED_LED_GPIO);
            Board::led_off(LedName::MCU_ORANGE_LED_GPIO);
            Board::led_off(LedName::MCU_GREEN_LED_GPIO);
            Utils::delay(1000000);
            Board::led_on(LedName::MCU_RED_LED_GPIO);
            Board::led_on(LedName::MCU_ORANGE_LED_GPIO);
            Board::led_on(LedName::MCU_GREEN_LED_GPIO);
            Utils::delay(1000000);
        }

        // Reset the processor.
        Board::reset();

        // Just please the Rust compiler which expects a divergent function.
        loop {}
    }
}
