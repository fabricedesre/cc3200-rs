// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#![feature(asm, lang_items)]

extern crate cc3200_sys;
#[macro_use]
extern crate log;

#[macro_use]
pub mod logger;
pub mod cc3200;
pub mod isr_vectors;

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

// Needed in debug builds to not get this linking error:
// .../rustlib/src/rust/src/libcore/fmt/num.rs:61: undefined reference to `__aeabi_memclr4'
#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr4(s: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = 0u8;
        i += 1;
    }
    return s;
}
