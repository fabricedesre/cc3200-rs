// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]
#![feature(try_from)]

#![feature(try_from)]

#[macro_use]
extern crate log;

pub use self::UtilsDelay as MAP_UtilsDelay;

pub mod simplelink;
pub mod socket;

extern "C" {
    // From board.c
    pub fn board_init();
    pub fn board_test();
    pub fn format_float_into(buf: *mut i8, buf_len: u32, num: f64, digits_after_decimal: u32);
    pub fn console_putchar(char: i8);
    pub fn reset();

    // From sdk/examples/common/gpio_if.c
    pub fn GPIO_IF_LedConfigure(pins: u8);
    pub fn GPIO_IF_LedOff(ledNum: i8);
    pub fn GPIO_IF_LedOn(ledNum: i8);

    // From sdk/examples/common/uart_if.c
    pub fn ClearTerm();
    pub fn InitTerm();

    // From sdk/driverlib/prcm.c
    pub fn PRCMRTCInUseSet();
    pub fn PRCMRTCSet(secs: u32, msecs: u16);
    pub fn PRCMRTCGet(secs: *mut u32, msecs: *mut u16);

    // From sdk/driverlib/utils.c
    pub fn UtilsDelay(loops: u32);

    // From FreeRTOS tasks.c
    pub fn vTaskStartScheduler();

    // From sdk/examples/common/i2c_if.c
    pub fn I2C_IF_Open(mode: u32) -> i32;
    pub fn I2C_IF_Close() -> i32;
    pub fn I2C_IF_Write(ucDevAddr: u8, pucData: *mut u8, ucLen: u8, ucStop: u8) -> i32;
    pub fn I2C_IF_Read(ucDevAddr: u8, pucData: *mut u8, ucLen: u8) -> i32;
    pub fn I2C_IF_ReadFrom(ucDevAddr: u8,
                           pucWrDataBuf: *mut u8,
                           ucWrLen: u8,
                           pucRdDataBuf: *mut u8,
                           ucRdLen: u8)
                           -> i32;
}
