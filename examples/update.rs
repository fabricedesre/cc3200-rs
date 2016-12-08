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

use cc3200::cc3200::{Board, Update};
use cc3200::simplelink::{SimpleLink, SimpleLinkError};
use cc3200::io::{File, Image, Read, Write};

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

fn update_demo() -> Result<(), Error> {

    try!(SimpleLink::start_spawn_task());
    SimpleLink::init_app_variables();
    try!(SimpleLink::start());

    // BUG? tzimmermann: I got a FILE_NOT_ERROR if I reverse
    // the next two lines.

    let imagename = "/sys/mcuimg.bin";

    let filename = "/update/mcuimg.bin";

    // 1) Get file info for update binary

    let info = try!(File::get_info(filename));

    println!("Found update {} of {} bytes.", filename, info.file_length);

    // 2) Open image file for writing

    let mut image = try!(Image::create(imagename, info.file_length as usize));

    // 3) Copy update to image

    {
        let mut file = try!(File::open(filename));

        let mut len = 0 as usize;

        loop {
            let mut buf: [u8; 256] = [0; 256];

            let buflen = match file.read(&mut buf) {
                Ok(res) => {
                    res
                },
                Err(_) => {
                    break; // assume EOF
                },
            };

            len += try!(image.write(&buf[0..buflen]));
        }

        println!("Wrote {} bytes to {}.", len, imagename);
    }

    // 4) Commit

    Update::commit();

    println!("Press RESET to run updated image...");

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

    println!("Welcome to CC3200 Update example {}", VERSION);

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match update_demo() {
                    Ok(())  => { println!("update_demo succeeded"); },
                    Err(e)  => { println!("update_demo failed: {:?}", e); },
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
