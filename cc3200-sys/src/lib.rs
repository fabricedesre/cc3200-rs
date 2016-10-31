// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

pub use self::UtilsDelay as MAP_UtilsDelay;

extern "C" {
    // From board.c
    pub fn board_init();
    pub fn console_putchar(char: i8);

    // From sdk/examples/common/gpio_if.c
    pub fn GPIO_IF_LedConfigure(pins: u8);
    pub fn GPIO_IF_LedOff(ledNum: i8);
    pub fn GPIO_IF_LedOn(ledNum: i8);

    // From sdk/examples/common/uart_if.c
    pub fn ClearTerm();
    pub fn InitTerm();

    // From sdk/examples/common/utils.c
    pub fn UtilsDelay(loops: u32);

    // From FreeRTOS tasks.c
    pub fn vTaskStartScheduler();
}
