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

use cc3200::cc3200::Board;
use freertos_rs::Task;

fn test_null() {
}

fn test0() {
    println!("x");
}

fn test1() {
    println!("{}", 1);
}

fn test2() {
    println!("{}.{}", 1, 2);
}

fn test3() {
    println!("{}.{}.{}", 1, 2, 3);
}

fn test4() {
    println!("{}.{}.{}.{}", 1, 2, 3, 4);
}

/*
fn test1f() {
    let num1: f64 = 1.2;
    println!("{:.1}", num1);
}

fn test2f() {
    let num1: f64 = 1.2;
    let num2: f64 = 3.4;
    println!("{:.1} {:.1}", num1, num2);
}

fn test3f() {
    let num1: f64 = 1.2;
    let num2: f64 = 3.4;
    let num3: f64 = 5.6;
    println!("{:.1} {:.1} {:.1}", num1, num2, num3);
}

fn test4f() {
    let num1: f64 = 1.2;
    let num2: f64 = 3.4;
    let num3: f64 = 5.6;
    let num4: f64 = 7.8;
    println!("{:.1} {:.1} {:.1} {:1}", num1, num2, num3, num4);
}
*/

fn test1f2() {
    let num1: f64 = 1.234;
    println!("{:.*}", 2, num1);
}

fn test2f2() {
    let num1: f64 = 1.234;
    let num2: f64 = 2.345;
    println!("{:.*} {:.*}", 2, num1, 2, num2);
}

fn test3f2() {
    let num1: f64 = 1.234;
    let num2: f64 = 2.345;
    let num3: f64 = 3.456;
    println!("{:.*} {:.*} {:.*}", 2, num1, 2, num2, 2, num3);
}

fn test_main() {
    let base = Board::get_stack_high_water_mark();
    test_null();
    let null_usage = Board::get_stack_high_water_mark();
    test0();
    let println_usage = Board::get_stack_high_water_mark();
    test1();
    let int_1_usage = Board::get_stack_high_water_mark();
    test2();
    let int_2_usage = Board::get_stack_high_water_mark();
    test3();
    let int_3_usage = Board::get_stack_high_water_mark();
    test4();
    let int_4_usage = Board::get_stack_high_water_mark();
    test1f2();
    let float_1_usage = Board::get_stack_high_water_mark();
    test2f2();
    let float_2_usage = Board::get_stack_high_water_mark();
    test3f2();
    let float_3_usage = Board::get_stack_high_water_mark();
    /*
    test4f();
    let float_4_usage = Board::get_stack_high_water_mark();
    */

    Board::console_printf_1_u32("    null: %5lu\n", (base - null_usage) as u32);
    Board::console_printf_1_u32("println!: %5lu\n", (null_usage - println_usage) as u32);
    Board::console_printf_1_u32("   int 1: %5lu\n", (println_usage - int_1_usage) as u32);
    Board::console_printf_1_u32("   int 2: %5lu\n", (int_1_usage - int_2_usage) as u32);
    Board::console_printf_1_u32("   int 3: %5lu\n", (int_2_usage - int_3_usage) as u32);
    Board::console_printf_1_u32("   int 4: %5lu\n", (int_3_usage - int_4_usage) as u32);
    Board::console_printf_1_u32(" float 1: %5lu\n", (println_usage - float_1_usage) as u32);
    Board::console_printf_1_u32(" float 2: %5lu\n", (float_1_usage - float_2_usage) as u32);
    Board::console_printf_1_u32(" float 3: %5lu\n", (float_2_usage - float_3_usage) as u32);
//    Board::console_printf_1_u32(" float 4: %5lu\n", (float_3_usage - float_4_usage) as u32);
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
    let _test = {
        Task::new()
            .name("test")
            .stack_size(2048)   // 32-bit words
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
