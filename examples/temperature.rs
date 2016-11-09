// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate cc3200;
extern crate alloc;
extern crate freertos_rs;
extern crate freertos_alloc;
#[macro_use]
extern crate log;

use cc3200::cc3200::{Board, Utils, I2C, I2COpenMode};
use cc3200::i2c_devices::{TMP006, TemperatureSensor};

use freertos_rs::Task;

static VERSION: &'static str = "1.0";

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {

    Board::init();

    info!("Welcome to CC3200 temperature sensor version {}", VERSION);

    let _temp = {
        Task::new()
            .name("temperature")
            .start(|| {
                let i2c = I2C::open(I2COpenMode::MasterModeFst).unwrap();
                let temp_sensor = TMP006::default(&i2c).unwrap();
                debug!("Temperature sensor initialized...");
                loop {
                    info!("Current temperature is {}", temp_sensor.get_temperature().unwrap());
                    Utils::delay(1333333 * 6);
                }
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
