// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200::I2C;
use i2c_devices::{I2CDevice, Accelerometer};

// Simple driver for the bma222 accelerometer.
// Fancy features like interrupts are not implemented.

static BMA222_DEV_ADDR: u8 = 0x18;
static BMA222_CHID_ID_REG: u8 = 0x00;
static BMA222_CHID_ID_VALUE: u8 = 0xf8;

static BMA222_ACC_DATA_X_NEW: u8 = 0x2;

// Other registers to directly access data:
// static BMA222_ACC_DATA_X: u8 = 0x3;
// static BMA222_ACC_DATA_Y_NEW: u8 = 0x4;
// static BMA222_ACC_DATA_Y: u8 = 0x5;
// static BMA222_ACC_DATA_Z_NEW: u8 = 0x6;
// static BMA222_ACC_DATA_Z: u8 = 0x7;

pub struct BMA222 {
    sensor: I2CDevice,
}

impl BMA222 {
    fn create(i2c: &I2C, dev_addr: u8) -> Option<BMA222> {
        let bma222 = BMA222 {
            sensor: I2CDevice::create(i2c, dev_addr)
        };
        if bma222.init() {
            return Some(bma222);
        }
        None
    }

    pub fn default(i2c: &I2C) -> Option<Self> {
        BMA222::create(i2c, BMA222_DEV_ADDR)
    }

    fn init(&self) -> bool {
        if let Ok(chip_id) = self.sensor.get_register_value::<u8>(BMA222_CHID_ID_REG) {
            info!("chip_id={:x}", chip_id);
            return chip_id == BMA222_CHID_ID_VALUE;
        }
        return false;
    }

    fn in_g(&self, value: u8) -> f64 {
        // The default resolution is +- 2g
        (value as f64) * 4.0 / 256.0 - 2.0
    }
}

impl Accelerometer for BMA222 {
    fn get_acceleration(&self, has_changed: bool) -> Option<(f64, f64, f64)> {
        let inp: [u8; 1] = [BMA222_ACC_DATA_X_NEW];
        let mut out: [u8; 6] = [0; 6];

        match self.sensor.i2c.read_from(self.sensor.dev_addr, &inp, &mut out) {
            Ok(_) => {
                let some_changed = (out[0] == 0x01) || (out[2] == 0x01) || (out[4] == 0x01);
                if !has_changed || (has_changed && some_changed) {
                    return Some((self.in_g(out[1]), self.in_g(out[3]), self.in_g(out[5])));
                }
                return None;
            }
            Err(_) => {
                return None;
            }
        }
    }
}
