// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]
#![feature(alloc)]
#![feature(collections)]

#[macro_use]
extern crate cc3200;
extern crate alloc;
extern crate freertos_rs;
extern crate freertos_alloc;

use cc3200::cc3200::Board;
use freertos_rs::Task;

#[macro_use]
extern crate collections;

use core::str;
use collections::string::String;

use cc3200::format::{format_int_into, format_float_into};

fn buf_find(buf: &[u8], needle: &str) -> Option<usize> {
    if let Ok(s) = str::from_utf8(buf) {
        s.find(needle)
    } else {
        None
    }
}

fn test_int_fill(num: i32, fill: char) {
    let mut buf: [u8; 32] = ['@' as u8; 32];

    let tmpl = b"Num: >99999<";
    let num_tmpl = "99999";

    buf[0..tmpl.len()].copy_from_slice(tmpl);

    let num_idx = buf_find(&buf, num_tmpl).unwrap();

    format_int_into(&mut buf[num_idx..num_idx + num_tmpl.len()], num, fill);

    println!("{}", String::from_utf8_lossy(&buf[0..tmpl.len()]));
}

fn test_int(num: i32) {
    test_int_fill(num, ' ');
    test_int_fill(num, '0');
}

fn test_float(num: f64, digits_after_decimal: u32) {
    let mut buf: [u8; 32] = ['@' as u8; 32];

    let tmpl = b"Num: >99999.99<";
    let num_tmpl = "99999.99";

    buf[0..tmpl.len()].copy_from_slice(tmpl);

    let num_idx = buf_find(&buf, num_tmpl).unwrap();

    format_float_into(&mut buf[num_idx..num_idx + num_tmpl.len()],
                      num,
                      digits_after_decimal);

    println!("{}", String::from_utf8_lossy(&buf[0..tmpl.len()]));
}

fn test_main() {
    let test_ints = vec![123456, 12345, 1234, 123, 12, 1, 0, -1, -12, -123, -1234, -12345, -123456];

    for test_i in test_ints.iter() {
        test_int(*test_i);
    }

    let test_floats = vec![123456.7890,
                           12345.6789,
                           1234.5678,
                           123.4567,
                           12.3456,
                           1.2345,
                           1.0000,
                           0.9999,
                           0.1000,
                           0.0000,
                           -0.1000,
                           -0.9999,
                           -1.0000,
                           -1.2345,
                           -12.3456,
                           -123.4567,
                           -1234.5678,
                           -12345.6789];

    for test_f in test_floats.iter() {
        test_float(*test_f, 2);
    }
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

    let _blinky = {
        Task::new()
            .name("blinky")
            .start(|| {
                test_main();
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
