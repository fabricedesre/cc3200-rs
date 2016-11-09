// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200::I2C;

pub struct I2CDevice {
    i2c: I2C,
    dev_addr: u8,
}

pub trait Converter {
    fn convert(input: [u8; 2]) -> Self;
}

impl Converter for u16 {
    fn convert(input: [u8; 2]) -> u16 {
        (input[0] as u16) << 8 | (input[1] as u16)
    }
}

impl Converter for i16 {
    fn convert(input: [u8; 2]) -> i16 {
        (input[0] as i16) << 8 | (input[1] as i16) as i16
    }
}

impl I2CDevice {
    pub fn create(i2c: &I2C, dev_addr: u8) -> I2CDevice {
        I2CDevice {
            i2c: i2c.clone(),
            dev_addr: dev_addr,
        }
    }

    pub fn get_register_value<T: Converter>(&self, addr: u8) -> Result<T, ()> {
        let inp: [u8; 1] = [addr];
        let mut out: [u8; 2] = [0; 2];

        match self.i2c.read_from(self.dev_addr, &inp, &mut out) {
            Ok(_) => {
                return Ok(T::convert(out));
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
