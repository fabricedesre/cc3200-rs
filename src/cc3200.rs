// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate cc3200_sys;

use collections::String;
use core;
use core::result::Result;

use self::cc3200_sys::{board_init, GPIO_IF_LedConfigure, GPIO_IF_LedOn, GPIO_IF_LedOff,
                       MAP_UtilsDelay, I2C_IF_Open, I2C_IF_Close, I2C_IF_Write, I2C_IF_Read,
                       I2C_IF_ReadFrom};
use logger::SimpleLogger;
use rtc::RTC;

#[allow(non_camel_case_types, dead_code)]
pub enum LedName {
    NO_LED_IND = 0,
    MCU_SENDING_DATA_IND,
    MCU_ASSOCIATED_IND, // Device associated to an AP
    MCU_IP_ALLOC_IND, // Device acquired an IP
    MCU_SERVER_INIT_IND, // Device connected to remote server
    MCU_CLIENT_CONNECTED_IND, // Any client connects to device
    MCU_ON_IND, // Device SLHost invoked successfully
    MCU_EXECUTE_SUCCESS_IND, // Task executed sucessfully
    MCU_EXECUTE_FAIL_IND, // Task execution failed
    MCU_RED_LED_GPIO, // GP09 for LED RED as per LP 3.0
    MCU_ORANGE_LED_GPIO, // GP10 for LED ORANGE as per LP 3.0
    MCU_GREEN_LED_GPIO, // GP11 for LED GREEN as per LP 3.0
    MCU_ALL_LED_IND,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum LedEnum {
    NO_LED = 0x0,
    LED1 = 0x1, // RED LED D7/GP9/Pin64
    LED2 = 0x2, // ORANGE LED D6/GP10/Pin1
    LED3 = 0x4, // GREEN LED D5/GP11/Pin2
}

pub struct Board { }

impl Board {
    pub fn init() {
        unsafe {
            board_init();
        }
        SimpleLogger::init().unwrap();
        RTC::init();
    }

    pub fn is_debugger_running() -> bool {
        unsafe { cc3200_sys::is_debugger_running() }
    }

    pub fn test() {
        unsafe {
            cc3200_sys::board_test();
        }
    }

    pub fn print_reg(label: &str, val: u32) {
        let mut s = String::with_capacity(label.len() + 1);
        s.push_str(label);
        s.push('\0');
        unsafe {
            cc3200_sys::print_reg(s.as_ptr(), val);
        }
    }

    pub fn led_configure(leds: &[LedEnum]) {
        let mut val = LedEnum::NO_LED as u8;
        for led in leds {
            val |= *led as u8;
        }
        unsafe {
            GPIO_IF_LedConfigure(val);
        }
    }

    pub fn led_off(led: LedName) {
        unsafe {
            GPIO_IF_LedOff(led as i8);
        }
    }

    pub fn led_on(led: LedName) {
        unsafe {
            GPIO_IF_LedOn(led as i8);
        }
    }

    pub fn start_scheduler() {
        unsafe {
            cc3200_sys::vTaskStartScheduler();
        }
    }

    pub fn disable_irq() {
        unsafe {
            asm!("cpsid i");
        }
    }

    pub fn reset() {
        unsafe {
            cc3200_sys::reset();
        }
    }
}

pub struct Console { }

impl Console {
    pub fn clear_term() {
        unsafe {
            cc3200_sys::ClearTerm();
        }
    }

    pub fn init_term() {
        unsafe {
            cc3200_sys::InitTerm();
        }
    }

    pub fn message(msg: &str) {
        for char in msg.chars() {
            // Lossy converstion from unicode to ASCII
            let ascii_char = {
                if char > '\x7f' { '?' } else { char }
            };
            unsafe {
                cc3200_sys::console_putchar(ascii_char as i8);
            }
        }
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Console::message(s);
        Ok(())
    }
}

pub enum I2COpenMode {
    MasterModeStd = 0,
    MasterModeFst = 1,
}

static mut I2C_IS_OPEN: bool = false;
static I2C_SUCCESS: i32 = 0;

#[derive(Clone, Copy)]
pub struct I2C { }

impl I2C {
    pub fn open(mode: I2COpenMode) -> Option<Self> {
        unsafe {
            // Only allow one instance to be created in a given open mode.
            if I2C_IS_OPEN || I2C_IF_Open(mode as u32) != I2C_SUCCESS {
                return None;
            }
            I2C_IS_OPEN = true;
        }
        return Some(I2C {});
    }

    pub fn close(&self) {
        unsafe {
            // TODO: decide what to do if we never called I2C_IF_Open
            I2C_IF_Close();
            I2C_IS_OPEN = false;
        }
    }

    pub fn write(&self, addr: u8, data: &[u8], stop: u8) -> Result<(), ()> {
        if data.len() > 255 {
            return Err(());
        }
        unsafe {
            if I2C_IF_Write(addr, data.as_ptr() as *mut u8, data.len() as u8, stop) == I2C_SUCCESS {
                return Ok(());
            }
        }
        return Err(());
    }

    pub fn read(&self, addr: u8, data: &mut [u8]) -> Result<(), ()> {
        if data.len() > 255 {
            return Err(());
        }
        unsafe {
            if I2C_IF_Read(addr, data.as_ptr() as *mut u8, data.len() as u8) == I2C_SUCCESS {
                return Ok(());
            }
        }
        return Err(());
    }

    pub fn read_from_with_length(&self,
                                 addr: u8,
                                 wr_data: &[u8],
                                 rd_data: &mut [u8],
                                 rd_len: u8)
                                 -> Result<(), ()> {
        if wr_data.len() > 255 || rd_data.len() > 255 {
            return Err(());
        }
        unsafe {
            if I2C_IF_ReadFrom(addr,
                               wr_data.as_ptr() as *mut u8,
                               wr_data.len() as u8,
                               rd_data.as_ptr() as *mut u8,
                               rd_len) == I2C_SUCCESS {
                return Ok(());
            }
        }
        return Err(());
    }

    pub fn read_from(&self, addr: u8, wr_data: &[u8], rd_data: &mut [u8]) -> Result<(), ()> {
        let len = rd_data.len() as u8;
        self.read_from_with_length(addr, wr_data, rd_data, len)
    }
}

pub struct Utils { }

impl Utils {
    pub fn delay(loops: u32) {
        unsafe {
            MAP_UtilsDelay(loops);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn abort() {
    unimplemented!();
}
