#![feature(lang_items)]
#![feature(asm)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

extern crate cc3200_sys;

use cc3200_sys::{ board_init,
                  GPIO_IF_LedConfigure, GPIO_IF_LedOn, GPIO_IF_LedOff,
                  MAP_UtilsDelay, LedEnum, LedName };

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {

    unsafe { board_init() };

    unsafe { GPIO_IF_LedConfigure(LedEnum::LED1 as u8|LedEnum::LED2 as u8|LedEnum::LED3 as u8) };

    unsafe { GPIO_IF_LedOff(LedName::MCU_ALL_LED_IND as i8) };
    let mut counter = 0;

    // We can't return from this function so we just spin endlessly here.
    loop {
        unsafe {
            GPIO_IF_LedOn(LedName::MCU_RED_LED_GPIO as i8);
            if counter & 1 != 0
            {
              GPIO_IF_LedOn(LedName::MCU_ORANGE_LED_GPIO as i8);
            } else {
                GPIO_IF_LedOff(LedName::MCU_ORANGE_LED_GPIO as i8);
            }
            if counter & 2 != 0
            {
                GPIO_IF_LedOn(LedName::MCU_GREEN_LED_GPIO as i8);
            } else {
                GPIO_IF_LedOff(LedName::MCU_GREEN_LED_GPIO as i8);
            }
            MAP_UtilsDelay(1333333);
            GPIO_IF_LedOff(LedName::MCU_RED_LED_GPIO as i8);
            MAP_UtilsDelay(1333333);
            GPIO_IF_LedOn(LedName::MCU_RED_LED_GPIO as i8);
            MAP_UtilsDelay(1333333);
            GPIO_IF_LedOff(LedName::MCU_RED_LED_GPIO as i8);
            MAP_UtilsDelay(1333333 * 6);
        }

        counter += 1;
    }
}

pub mod isr_vectors;

// Finally, we need to define the panic_fmt "lang item", which is just a function. This specifies
// what the program should do when a `panic!` occurs. Our program won't panic, so we can leave the
// function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}
}
