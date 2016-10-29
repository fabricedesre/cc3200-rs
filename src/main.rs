// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(lang_items)]
#![feature(asm)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

mod cc3200;
use cc3200::{CC3200, LedEnum, LedName};

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {

    CC3200::init();

    //CC3200::LedConfigure([LedEnum::LED1, as u8 | LedEnum::LED2 as u8 | LedEnum::LED3 as u8);
    CC3200::led_configure(&[LedEnum::LED1, LedEnum::LED2, LedEnum::LED3]);

    CC3200::led_off(LedName::MCU_ALL_LED_IND);
    let mut counter = 0;

    // We can't return from this function so we just spin endlessly here.
    loop {
        CC3200::led_on(LedName::MCU_RED_LED_GPIO);
        if counter & 1 != 0 {
            CC3200::led_on(LedName::MCU_ORANGE_LED_GPIO);
        } else {
            CC3200::led_off(LedName::MCU_ORANGE_LED_GPIO);
        }
        if counter & 2 != 0 {
            CC3200::led_on(LedName::MCU_GREEN_LED_GPIO);
        } else {
            CC3200::led_off(LedName::MCU_GREEN_LED_GPIO);
        }
        CC3200::delay(1333333);
        CC3200::led_off(LedName::MCU_RED_LED_GPIO);
        CC3200::delay(1333333);
        CC3200::led_on(LedName::MCU_RED_LED_GPIO);
        CC3200::delay(1333333);
        CC3200::led_off(LedName::MCU_RED_LED_GPIO);
        CC3200::delay(1333333 * 6);

        counter += 1;
    }
}

pub mod isr_vectors;

// Finally, we need to define the panic_fmt "lang item", which is just a function. This specifies
// what the program should do when a `panic!` occurs. Our program won't panic, so we can leave the
// function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
