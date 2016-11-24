// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200::I2C;

pub struct I2CDevice {
    pub i2c: I2C,
    pub dev_addr: u8,
}

// Array of the max length we support.
type I2cArray = [u8; 2];

// Helper trait to convert a byte array into a primitive type.
pub trait Converter {
    fn convert16(input: I2cArray) -> Self;
    fn convert8(input: I2cArray) -> Self;
}

impl Converter for u16 {
    fn convert16(input: I2cArray) -> Self {
        (input[0] as u16) << 8 | (input[1] as u16)
    }

    fn convert8(_: I2cArray) -> Self {
        unimplemented!();
    }
}

impl Converter for i16 {
    fn convert16(input: I2cArray) -> Self {
        (input[0] as i16) << 8 | (input[1] as i16) as i16
    }

    fn convert8(_: I2cArray) -> Self {
        unimplemented!();
    }
}

impl Converter for u8 {
    fn convert16(_: I2cArray) -> Self {
        unimplemented!();
    }

    fn convert8(input: I2cArray) -> Self {
        input[0]
    }
}

impl Converter for i8 {
    fn convert16(_: I2cArray) -> Self {
        unimplemented!();
    }

    fn convert8(input: I2cArray) -> Self {
        input[0] as i8
    }
}

impl I2CDevice {
    pub fn create(i2c: &I2C, dev_addr: u8) -> I2CDevice {
        I2CDevice {
            i2c: i2c.clone(),
            dev_addr: dev_addr,
        }
    }

    // Retrieves a value from the register at addr.
    // Supported value types are 8 & 16 bits signed or unsigned integers.
    pub fn get_register_value<T: Converter>(&self, addr: u8) -> Result<T, ()> {
        use core::intrinsics::size_of;

        let inp: [u8; 1] = [addr];
        let mut out: I2cArray = [0; 2];

        let type_size = unsafe { size_of::<T>() };
        match self.i2c.read_from_with_length(self.dev_addr, &inp, &mut out, type_size as u8) {
            Ok(_) => {
                match type_size {
                    1 => return Ok(T::convert8(out)),
                    2 => return Ok(T::convert16(out)),
                    _ => return Err(()),
                }
            }
            Err(_) => {
                return Err(());
            }
        }
    }
}

// Traits for common device types.

pub trait TemperatureSensor {
    fn get_temperature(&self) -> Option<f64>;
}

pub trait Accelerometer {
    // Returns the (x, y, z) g values.
    // If has_changed is true and the state has not changed, will return None.
    fn get_acceleration(&self, has_changed: bool) -> Option<(f64, f64, f64)>;
}
