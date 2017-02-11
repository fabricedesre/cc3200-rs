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
use self::cc3200_sys::simplelink::*;
use logger::SimpleLogger;
use rtc::RTC;
use io::{File, Read, Write};

macro_rules! ignore {
    ($e:expr) => ({
        match $e {
            _ => { },
        }
    })
}

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

//
// Image updates
//

#[derive(Copy, Clone)]
pub enum ImageStatus {
    TESTING,
    TESTREADY,
    NOTEST
}

impl ImageStatus {
    pub fn from_u32(value: u32) -> Result<ImageStatus, ()> {
        let image_status = match value {
            IMG_STATUS_TESTING   => ImageStatus::TESTING,
            IMG_STATUS_TESTREADY => ImageStatus::TESTREADY,
            IMG_STATUS_NOTEST    => ImageStatus::NOTEST,
            value => {
                println!("unknwon image status {}", value);
                return Err(());
            }
        };
        Ok(image_status)
    }
    pub fn to_u32(image_status: ImageStatus) -> Result<u32, ()> {
        let value = match image_status {
            ImageStatus::NOTEST    => IMG_STATUS_NOTEST,
            ImageStatus::TESTREADY => IMG_STATUS_TESTREADY,
            ImageStatus::TESTING   => IMG_STATUS_TESTING
        };
        Ok(value)
    }
}

pub struct BootInfo {
    pub active_image: u8,
    pub image_status: ImageStatus,
}

impl BootInfo {

    /// Create a BootInfo structure
    pub fn new(active_image: u8, image_status: ImageStatus) -> BootInfo {
        BootInfo { active_image: active_image,
                   image_status: image_status }
    }

    /// Create a BootInfo structure the represents the factory reset
    pub fn factory_reset() -> BootInfo {
        BootInfo::new(0, ImageStatus::NOTEST)
    }

    /// Returns the currently active image's filename
    pub fn image_filename(&self) -> &'static str {
        match self.active_image {
            0 => IMG_FACTORY_DEFAULT,
            1 => IMG_USER_1,
            2 => IMG_USER_2,
            _ => IMG_FACTORY_DEFAULT
        }
    }

    /// Returns the filename of the image to update
    pub fn next_image_filename(&self) -> &'static str {
        // If we booted from image 1, we update image 2; in any other case
        // we update image 1. This mirrors the behavior of the TI SDK.
        match self.active_image {
            1 => IMG_USER_2,
            _ => IMG_USER_1
        }
    }
}

pub struct Update { }

impl Update {

    /// Sets the application to test mode
    pub fn test() -> bool {
        let res = unsafe {
            sl_extlib_FlcTest(FLC_TEST_RESET_MCU |
                              FLC_TEST_RESET_MCU_WITH_APP)
        };
        Update::reset_is_required(res)
    }

    /// Returns true if the application runs in testing mode, false otherwise
    pub fn is_testing() -> bool {
        let res = unsafe {
            sl_extlib_FlcIsPendingCommit()
        };
        res != 0
    }

    /// Commits all changes, moving the application state from 'testing' to 'stable'
    pub fn commit() -> bool {
        let res = unsafe {
            sl_extlib_FlcCommit(FLC_COMMITED)
        };
        Update::reset_is_required(res)
    }

    /// Aborts the update
    pub fn abort() -> bool {
        let res = unsafe {
            sl_extlib_FlcCommit(FLC_NOT_COMMITED)
        };
        Update::reset_is_required(res)
    }

    /// Reads or creates the device's boot-info structure
    pub fn get_boot_info() -> Result<BootInfo, ()> {
        let boot_info = match Update::read_boot_info() {
            Ok(boot_info) => boot_info,
            Err(_) => {
                let boot_info = BootInfo::factory_reset();
                match Update::write_boot_info(&boot_info) {
                    Ok(_)  => { },
                    Err(_) => { return Err(()); }
                };
                boot_info
            }
        };
        Ok(boot_info)
    }

    /// Opens the next image for updating; the filename is selected automatically
    pub fn next_image(max_len: usize) -> Result<File, SimpleLinkError> {
        // The TI filesystem doesn't resize files dynamically. Hence, if
        // the image file already exists and is too small, we cannot use
        // it for the update.
        //
        // Below, we read the boot-info file to get next image's filename and
        // its size. We remove the existing file if it's too small. The latter
        // call to File::create() will create a new file.
        let file_name = match Update::get_boot_info() {
            Ok(boot_info) => {
                let file_name = boot_info.next_image_filename();
                match File::get_info(file_name) {
                    Ok(file_info) => {
                        let allocated_length = file_info.allocated_length as usize;
                        if allocated_length < max_len {
                            File::remove(file_name)?;
                        }
                    },
                    Err(_) => {
                        // No file info available; unconditionally remove
                        // file if it exists.
                        ignore!(File::remove(file_name));
                    }
                };
                file_name
            },
            Err(_) => {
                return Result::Err(SimpleLinkError::FileSystem(FileSystemError::NOT_SUPPORTED))
            }
        };
        File::create(file_name, max_len, false)
    }

    fn reset_is_required(flags: i32) -> bool {
        (flags & (FLC_TEST_RESET_MCU | FLC_TEST_RESET_NWP)) != 0
    }

    fn encode_boot_info(boot_info: &BootInfo) -> Result<[u8; 8], ()> {
        let active_image = boot_info.active_image;
        let image_status = ImageStatus::to_u32(boot_info.image_status)?;
        let mut buf: [u8; 8] = [0; 8];
        buf[0] = active_image;
        buf[4] = 0xff & (image_status) as u8;
        buf[5] = 0xff & (image_status >> 8) as u8;
        buf[6] = 0xff & (image_status >> 16) as u8;
        buf[7] = 0xff & (image_status >> 24) as u8;
        Ok(buf)
    }

    fn decode_boot_info(buf: [u8; 8]) -> Result<BootInfo, ()> {
        let active_image = buf[0];
        let image_status = ((buf[4] as u32)) |
                           ((buf[5] as u32) << 8) |
                           ((buf[6] as u32) << 16) |
                           ((buf[7] as u32) << 24);
        Ok(BootInfo::new(active_image, ImageStatus::from_u32(image_status)?))
    }

    fn write_boot_info(boot_info: &BootInfo) -> Result<(), ()> {

        let buf = Update::encode_boot_info(boot_info)?;

        let mut file = match File::create(IMG_BOOT_INFO, 8, false) {
            Ok(file) => file,
            Err(_)   => { return Err(()); }
        };
        match file.write(&buf[..]) {
            Ok(buflen) => {
                if buflen < buf.len() {
                    return Err(());
                }
            },
            Err(_) => { return Err(()); }
        }
        Ok(())
    }

    fn read_boot_info() -> Result<BootInfo, ()> {
        let mut buf: [u8; 8] = [0; 8];
        let mut file = match File::open(IMG_BOOT_INFO) {
            Ok(file) => file,
            Err(_)   => return Err(())
        };
        match file.read(&mut buf) {
            Ok(buflen) => {
                if buflen != buf.len() {
                    return Err(());
                }
            },
            Err(_) => { return Err(()); }
        };
        Update::decode_boot_info(buf)
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
