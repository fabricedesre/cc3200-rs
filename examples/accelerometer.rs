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

use cc3200::cc3200::{Board, I2C, I2COpenMode};
use cc3200::i2c_devices::Accelerometer;
use cc3200::bma222::BMA222;

use freertos_rs::{CurrentTask, Duration, Task};

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

    info!("Welcome to CC3200 accelerometer version {}", VERSION);

    let _temp = {
        Task::new()
            .name("accelerometer")
            .start(|| {
                let i2c = I2C::open(I2COpenMode::MasterModeFst).unwrap();
                let accelerometer = BMA222::default(&i2c).unwrap();
                debug!("Accelerometer initialized...");
                loop {
                    // Get the acceleration only if it has changed.
                    match accelerometer.get_acceleration(true) {
                        None => info!("No change"),
                        Some(value) => {
                            info!("Current acceleration is {} {} {}", value.0, value.1, value.2);
                        }
                    }
                    CurrentTask::delay(Duration::ms(1000))
                }
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
