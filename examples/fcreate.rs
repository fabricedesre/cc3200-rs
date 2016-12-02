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
use cc3200::simplelink::{FileSystemError, SimpleLink, SimpleLinkError};
use cc3200::io::{File};

use freertos_rs::{Task};

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

fn fcreate_demo() -> Result<(), Error> {

    try!(SimpleLink::start_spawn_task());
    SimpleLink::init_app_variables();
    try!(SimpleLink::start());

    let filename = "myfile.txt";

    let mut lbound = 0 as usize; // always available
    let mut ubound = 1024 * 1024 as usize; // not available

    println!("Detecting maximum file length... (might take a moment)");

    loop {
        ignore!(File::remove(filename));

        let len = (ubound + lbound) / 2;

        match File::create(filename, len, false) {
            Ok(_) => {
                lbound = len;
                if lbound + 1 == ubound {
                    break; // found maximum length
                }
            },
            Err(_) => {
                ubound = len;
            }
        };

        if lbound == ubound {
            // cannot legally happen in practice
            println!("Recursion bug detected");
            return Err(Error::from(SimpleLinkError::FileSystem(FileSystemError::UNKNOWN)))
        }
    }

    println!("Maximum detected file length is {} bytes", lbound);

    let info = try!(File::get_info(filename));

    println!("File info:");
    println!("        flags={}", info.flags);
    println!("        current length={}", info.file_length);
    println!("        allocated length={}", info.allocated_length);
    println!("        tokens=({}, {}, {}, {})",
                info.token[0], info.token[1], info.token[2], info.token[3]);

    ignore!(File::remove(filename));

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

    println!("Welcome to CC3200 fcreate example {}", VERSION);

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match fcreate_demo() {
                    Ok(())  => { println!("fcreate_demo succeeded"); },
                    Err(e)  => { println!("fcreate_demo failed: {:?}", e); },
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
