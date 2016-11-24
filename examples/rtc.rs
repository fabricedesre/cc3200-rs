// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

#[macro_use]
extern crate cc3200;
extern crate freertos_rs;
extern crate freertos_alloc;

#[macro_use]
extern crate log;

use cc3200::cc3200::Board;
use cc3200::rtc::RTC;
use freertos_rs::{CurrentTask, Duration, Task};

pub fn rtc_demo() {

    for _ in 0..10 {
        println!("RTC = {}", RTC::get());
        CurrentTask::delay(Duration::ms(1000));
    }

    println!("Setting RTC to 0x1000");
    RTC::set(0x1000);
    println!("RTC = 0x{:x}", RTC::get());
    for _ in 0..10 {
        CurrentTask::delay(Duration::ms(1000));
        println!("RTC = 0x{:x}", RTC::get());
    }

    println!("Setting RTC to 0xFFFFFFFC");
    RTC::set(0xFFFFFFFC);
    println!("RTC = 0x{:09x}", RTC::get());
    for _ in 0..10 {
        CurrentTask::delay(Duration::ms(1000));
        println!("RTC = 0x{:09x}", RTC::get());
    }
    println!("Done");
}

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

    println!("Welcome to CC3200 RTC Demo");

    let _client = {
        Task::new()
            .name("rtc-demo")
            .stack_size(2048) // 32-bit words
            .start(|| {
                rtc_demo();
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
