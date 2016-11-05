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

use cc3200::cc3200::{Board, Console, Utils, LedEnum, LedName};

use alloc::arc::Arc;
use freertos_rs::{CurrentTask, Duration, Task, Queue};

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

    Console::init_term();
    Console::clear_term();
    println!("Welcome to CC3200 blinking leds version {}", VERSION);

    Board::test();

    let queue = Arc::new(Queue::new(10).unwrap());
    let _producer = {
        let queue = queue.clone();
        Task::new()
            .name("producer")
            .start(move || {
                let msgs = ["Welcome ", "to ", "CC32xx ", "development !\n"];
                loop {
                    for msg in msgs.iter() {
                        queue.send(msg, Duration::ms(15)).unwrap();
                        CurrentTask::delay(Duration::ms(15))
                    }
                    CurrentTask::delay(Duration::ms(1000))
                }
            })
            .unwrap()
    };

    let _consumer = {
        let queue = queue.clone();
        Task::new()
            .name("consumer")
            .start(move || {
                loop {
                    let msg = queue.receive(Duration::ms(2000)).unwrap();
                    Console::message("Received: ");
                    Console::message(msg);
                    Console::message("\n");
                }
            })
            .unwrap()
    };

    let _blinky = {
        Task::new()
            .name("blinky")
            .start(|| {
                Board::led_configure(&[LedEnum::LED1, LedEnum::LED2, LedEnum::LED3]);
                Board::led_off(LedName::MCU_ALL_LED_IND);
                let mut counter = 0;
                loop {
                    Board::led_on(LedName::MCU_RED_LED_GPIO);
                    if counter & 1 != 0 {
                        Board::led_on(LedName::MCU_ORANGE_LED_GPIO);
                    } else {
                        Board::led_off(LedName::MCU_ORANGE_LED_GPIO);
                    }
                    if counter & 2 != 0 {
                        Board::led_on(LedName::MCU_GREEN_LED_GPIO);
                    } else {
                        Board::led_off(LedName::MCU_GREEN_LED_GPIO);
                    }
                    Utils::delay(1333333);
                    Board::led_off(LedName::MCU_RED_LED_GPIO);
                    Utils::delay(1333333);
                    Board::led_on(LedName::MCU_RED_LED_GPIO);
                    Utils::delay(1333333);
                    Board::led_off(LedName::MCU_RED_LED_GPIO);
                    Utils::delay(1333333 * 6);

                    counter += 1;
                }
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
