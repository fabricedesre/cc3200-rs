// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200::I2C;
use i2c_devices::{I2CDevice, TemperatureSensor};

// Port of example/out_of_box/tmp006drv.c

static TMP006_DEV_ADDR: u8 = 0x41;
static TMP006_VOBJECT_REG_ADDR: u8 = 0x00;
static TMP006_TAMBIENT_REG_ADDR: u8 = 0x01;
static TMP006_MANUFAC_ID_REG_ADDR: u8 = 0xFE;
static TMP006_DEVICE_ID_REG_ADDR: u8 = 0xFF;

static TMP006_MANUFAC_ID: u16 = 0x5449;
static TMP006_DEVICE_ID: u16 = 0x0067;

#[derive(Clone, Copy)]
pub struct TMP006 {
    sensor: I2CDevice,
}

impl TMP006 {
    fn create(i2c: &I2C, dev_addr: u8) -> Option<TMP006> {
        let tmp006 = TMP006 { sensor: I2CDevice::create(i2c, dev_addr) };
        if tmp006.init() {
            return Some(tmp006);
        }
        None
    }

    pub fn default(i2c: &I2C) -> Option<Self> {
        TMP006::create(i2c, TMP006_DEV_ADDR)
    }

    fn init(&self) -> bool {
        if let Ok(manufac_id) = self.sensor.get_register_value::<u16>(TMP006_MANUFAC_ID_REG_ADDR) {
            if manufac_id != TMP006_MANUFAC_ID {
                return false;
            }

            if let Ok(device_id) = self.sensor
                .get_register_value::<u16>(TMP006_DEVICE_ID_REG_ADDR) {
                if device_id == TMP006_DEVICE_ID {
                    return true;
                }
            }
            return false;
        }
        return false;
    }

    // Returns the temperature in Celcius.
    fn compute_temp(vobject: f64, ambient: f64) -> f64 {
        use core::intrinsics::{powf64, powif64};
        // This algorithm is obtained from
        // http://processors.wiki.ti.com/index.php/SensorTag_User_Guide
        // #IR_Temperature_Sensor
        //
        let t_die2 = ambient + 273.15;
        let s0 = 6.4e-14; // Calibration factor
        let a1 = 1.75e-3;
        let a2 = -1.678e-5;
        let b0 = -2.94e-5;
        let b1 = -5.7e-7;
        let b2 = 4.63e-9;
        let c2 = 13.4;
        let t_ref = 298.15;
        let t_obj: f64;
        unsafe {
            let s = s0 * (1.0 + a1 * (t_die2 - t_ref) + a2 * powif64((t_die2 - t_ref), 2));
            let v_os = b0 + b1 * (t_die2 - t_ref) + b2 * powif64(t_die2 - t_ref, 2);
            let f_obj = (vobject - v_os) + c2 * powif64(vobject - v_os, 2);
            t_obj = powf64(powif64(t_die2, 4) + (f_obj / s), 0.25);
        }
        return t_obj - 273.15;
    }
}

impl TemperatureSensor for TMP006 {
    fn get_temperature(&self) -> Option<f64> {
        if let Ok(vobject) = self.sensor.get_register_value::<i16>(TMP006_VOBJECT_REG_ADDR) {
            if let Ok(ambient_temp) = self.sensor
                .get_register_value::<u16>(TMP006_TAMBIENT_REG_ADDR) {
                return Some(TMP006::compute_temp((vobject as f64) * 156.25e-9,
                                                 (ambient_temp as f64) / 128.0));
            }
        }
        None
    }
}
