// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200::I2C;
//use core::marker::Sized;

// Port of example/out_of_box/tmp006drv.c

static TMP006_DEV_ADDR: u8 = 0x41;
static TMP006_VOBJECT_REG_ADDR: u8 = 0x00;
static TMP006_TAMBIENT_REG_ADDR: u8 = 0x01;
static TMP006_MANUFAC_ID_REG_ADDR: u8 = 0xFE;
static TMP006_DEVICE_ID_REG_ADDR: u8 = 0xFF;

static TMP006_MANUFAC_ID: u16 = 0x5449;
static TMP006_DEVICE_ID: u16 = 0x0067;

pub struct I2CDevice {
    i2c: I2C,
    dev_addr: u8,
}

impl I2CDevice {
    pub fn create(i2c: &I2C, dev_addr: u8) -> I2CDevice {
        I2CDevice {
            i2c: i2c.clone(),
            dev_addr: dev_addr,
        }
    }

    pub fn get_register_value(&self, addr: u8) -> Result<u16, ()> {
        let inp: [u8; 1] = [addr];
        let mut out: [u8; 2] = [0; 2];

        match self.i2c.read_from(self.dev_addr, &inp, &mut out) {
            Ok(_) => {
                return Ok((out[0] as u16) << 8 | (out[1] as u16));
            }
            Err(_) => {
                return Err(());
            }
        }
    }
}

pub trait TemperatureSensor {
    fn get_temperature(&self) -> Option<f64>;
}

pub struct TMP006 {
    sensor: I2CDevice,
}

impl TMP006 {
    fn create(i2c: &I2C, dev_addr: u8) -> Option<TMP006> {
        let tmp006 = TMP006 {
            sensor: I2CDevice::create(i2c, dev_addr)
        };
        if tmp006.init() {
            return Some(tmp006);
        }
        None
    }

    pub fn default(i2c: &I2C) -> Option<Self> {
        TMP006::create(i2c, TMP006_DEV_ADDR)
    }

    fn init(&self) -> bool {
        if let Ok(manufac_id) = self.sensor.get_register_value(TMP006_MANUFAC_ID_REG_ADDR) {
            if manufac_id != TMP006_MANUFAC_ID {
                return false;
            }

            if let Ok(device_id) = self.sensor.get_register_value(TMP006_DEVICE_ID_REG_ADDR) {
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
        let s0 = 6.4E-14; // Calibration factor
        let a1 = 1.75E-3;
        let a2 = -1.678E-5;
        let b0 = -2.94E-5;
        let b1 = -5.7E-7;
        let b2 = 4.63E-9;
        let c2 = 13.4;
        let t_ref = 298.15;
        let t_obj: f64;
        unsafe {
            let s = s0 * (1.0 + a1 * (t_die2 - t_ref) + a2 * powif64((t_die2 - t_ref), 2));
            let v_os = b0 + b1 * (t_die2 - t_ref) + b2 * powif64((t_die2 - t_ref), 2);
            let f_obj = (vobject - v_os) + c2 * powif64((vobject - v_os), 2);
            t_obj = powf64(powif64(t_die2, 4) + (f_obj / s), 0.25);
        }
        return t_obj - 273.15;
    }
}

impl TemperatureSensor for TMP006 {
    fn get_temperature(&self) -> Option<f64> {
        debug!("TMP006::get_temperature start");
        if let Ok(vobject) = self.sensor.get_register_value(TMP006_VOBJECT_REG_ADDR) {
            if let Ok(ambient_temp) = self.sensor.get_register_value(TMP006_TAMBIENT_REG_ADDR) {
                /*return Some(TMP006::compute_temp((vobject as f64) * 156.25e-9,
                                                 (ambient_temp as f64) / 128.0));*/
                let a = (vobject as f64) * 156.25e-9;
                let b = (ambient_temp as f64) / 128.0;
                let res = TMP006::compute_temp(a, b);
                return Some((vobject as f64) + (ambient_temp as f64));
            }
        }
        None
    }
}
