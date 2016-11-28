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

extern crate smallhttp;

#[macro_use]
extern crate log;

#[macro_use]
extern crate collections;

use cc3200::cc3200::{Board};
use cc3200::simplelink::{SimpleLink, SimpleLinkError};
use cc3200::io::{File, Read, Seek, Write};

use freertos_rs::{Task};

use collections::String;

static VERSION: &'static str = "1.0";

#[derive(Debug)]
pub enum Error {
    SLE(SimpleLinkError),
}

impl From<SimpleLinkError> for Error {
    fn from(err: SimpleLinkError) -> Error {
        Error::SLE(err)
    }
}

macro_rules! ignore {
    ($e:expr) => ({
        match $e {
            Ok(_) => { },
            Err(_) => { },
        }
    })
}

fn fileio_demo() -> Result<(), Error> {

    try!(SimpleLink::start_spawn_task());
    SimpleLink::init_app_variables();
    try!(SimpleLink::start());

    let filename = "myfile.txt";
    let test_string = "Hello world";

    // 1) Remove test file, if any

    ignore!(File::remove(filename));

    // 2) Write test file

    {
        println!("Writing string \"{}\"", test_string);

        let mut file = try!(File::create(filename));
        try!(file.write(test_string.as_bytes()));
    }

    // 3) Get file info

    let info = try!(File::get_info(filename));

    println!("File info:");
    println!("        flags={}", info.flags);
    println!("        current length={}", info.file_length);
    println!("        allocated length={}", info.allocated_length);
    println!("        tokens=({}, {}, {}, {})",
                info.token[0], info.token[1], info.token[2], info.token[3]);


    // 4) Read test file

    let mut file = try!(File::open(filename));

    let mut str = String::new();
    try!(file.read_to_string(&mut str)); // "Hello world"

    println!("Read string \"{}\"", str);

    try!(file.seek(6));
    try!(file.read_to_string(&mut str)); // "world"

    println!("Read string \"{}\" at offset 6", str);

    Ok(())
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

    println!("Welcome to CC3200 File I/O example {}", VERSION);

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match fileio_demo() {
                    Ok(())  => { println!("fileio_demo succeeded"); },
                    Err(e)  => { println!("fileio_demo failed: {:?}", e); },
                };
                loop {}
            })
            .unwrap()
    };

    Board::start_scheduler();

    // The only reason start_scheduler should fail is if there wasn't enough
    // heap to initialize the IDLE and timer tasks

    loop {}
}
