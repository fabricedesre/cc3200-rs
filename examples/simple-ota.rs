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

use cc3200::cc3200::{Board, LedEnum, LedName, Update};
use cc3200::io::{File, Read, Write};
use cc3200::simplelink::{self, NetConfigSet, Policy, SimpleLink, SimpleLinkError, WlanConfig,
                         WlanMode, WlanRxFilterOp, WlanRxFilterOpBuf};
use cc3200::socket_channel::SocketChannel;
use collections::{String, Vec};
use core::str;

use freertos_rs::{CurrentTask, Duration, Task};
use smallhttp::{Client, HttpHeader};
use smallhttp::traits::{Channel, ChannelError};

mod config;

use config::SIMPLE_OTA_URL;

static VERSION: &'static str = "1.0";

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AppError {
    DEVICE_NOT_IN_STATION_MODE,
    PING_FAILED,
    INTERNET_CONNECTION_FAILED,
    LAN_CONNECTION_FAILED,
    INVALID_IMAGE_SIZE,
    IMAGE_DOWNLOAD_FAILED,
}

#[derive(Debug)]
pub enum Error {
    SLE(SimpleLinkError),
    App(AppError),
}

impl From<SimpleLinkError> for Error {
    fn from(err: SimpleLinkError) -> Error {
        Error::SLE(err)
    }
}

impl From<AppError> for Error {
    fn from(err: AppError) -> Error {
        Error::App(err)
    }
}

macro_rules! ignore {
    ($e:expr) => ({
        match $e {
            _  => { }
        }
    })
}

fn configure_simple_link_to_default() -> Result<(), Error> {
    let mode = SimpleLink::start()?;
    if mode != WlanMode::ROLE_STA {
        if mode == WlanMode::ROLE_AP {
            // If the device is in AP mode, then we need to wait for the
            // acquired event before doing anything.

            while !SimpleLink::is_ip_acquired() {
                CurrentTask::delay(Duration::ms(100));
            }
        }

        // Switch to STA mode and restart

        SimpleLink::wlan_set_mode(WlanMode::ROLE_STA)?;
        SimpleLink::stop(255)?;
        let mode = SimpleLink::start()?;
        if mode != WlanMode::ROLE_STA {
            return Err(Error::App(AppError::DEVICE_NOT_IN_STATION_MODE));
        }
    }

    // Get the device's version-information
    let ver = SimpleLink::get_version();

    println!("Host Driver Version: {}", SimpleLink::get_driver_version());
    println!("Build Version {}.{}.{}.{}.31.{}.{}.{}.{}.{}.{}.{}.{}",
             ver.nwp_version[0],
             ver.nwp_version[1],
             ver.nwp_version[2],
             ver.nwp_version[3],
             ver.fw_version[0],
             ver.fw_version[1],
             ver.fw_version[2],
             ver.fw_version[3],
             ver.phy_version[0],
             ver.phy_version[1],
             ver.phy_version[2],
             ver.phy_version[3]);

    // Set connection policy to Auto + SmartConfig
    //      (Device's default connection policy)
    SimpleLink::wlan_set_policy(Policy::ConnectionDefault, &[])?;

    // Remove all profiles
    SimpleLink::wlan_delete_profile(0xff)?;

    // Device is in station mode. Disconnect previous connection, if any.
    if SimpleLink::wlan_disconnect().is_ok() {
        // This means that we were previously connected. Wait for the
        // notification event.
        while !SimpleLink::is_connected() {
            CurrentTask::delay(Duration::ms(100));
        }
    }

    // Enable DHCP client
    SimpleLink::netcfg_set(NetConfigSet::Ipv4StaP2pClientDhcpEnable, &[1])?;

    // Disable Scan
    SimpleLink::wlan_set_policy(Policy::ScanDisable, &[])?;

    // Set Tx power level for station mode
    // Number between 0-15, as dB offset from max power - 0 will set max power

    SimpleLink::wlan_set(WlanConfig::GeneralStaTxPower, &[0])?;

    // Set PM policy to normal
    SimpleLink::wlan_set_policy(Policy::PowerNormal, &[])?;

    // Unregister mDNS services
    SimpleLink::netapp_mdns_unregister_service("")?;

    // Remove  all 64 filters (8*8)

    let all_filters = WlanRxFilterOpBuf::all_filters();
    SimpleLink::wlan_rx_filter(WlanRxFilterOp::Remove, &all_filters)?;

    SimpleLink::stop(simplelink::SL_STOP_TIMEOUT)?;

    SimpleLink::init_app_variables();
    Ok(())
}

fn wlan_connect() -> Result<(), Error> {

    let sec_params = config::security_params();

    SimpleLink::wlan_connect(config::SSID, &[], sec_params, None)?;

    println!("Connecting to {} ...", config::SSID);
    // Wait for WLAN event
    while !SimpleLink::is_connected() || !SimpleLink::is_ip_acquired() {
        // Toggle LEDs to indicate Connection Progress
        Board::led_on(LedName::MCU_RED_LED_GPIO);
        CurrentTask::delay(Duration::ms(100));
        Board::led_off(LedName::MCU_RED_LED_GPIO);
        CurrentTask::delay(Duration::ms(100));
    }
    Ok(())
}

fn content_length(headers: &Vec<(HttpHeader, String)>) -> Result<usize, Error> {
    for header in headers {
        let &(ref name, ref value) = header;
        match *name {
            HttpHeader::ContentLength => {
                match value.parse::<usize>() {
                    Ok(res) => {
                        return Ok(res);
                    },
                    Err(_) => {
                        return Err(Error::App(AppError::INVALID_IMAGE_SIZE));
                    }
                };
            }
            _ => {
                // nothing to do
            }
        }
    }
    return Err(Error::App(AppError::INVALID_IMAGE_SIZE));
}

fn get_update(filename: &str, url: &str) -> Result<(), Error> {
    let mut client = Client::new(SocketChannel::new().unwrap());
    let response = client.get(url)
        .open()
        .unwrap()
        .send(&[])
        .unwrap()
        .response(|header_name| header_name == HttpHeader::ContentLength)
        .unwrap();

    println!("Received URL {}, downloading update...", url);

    let file_length = content_length(&response.headers)?;
    let mut file = File::create(filename, file_length, true)?;

    let mut buf = [0u8; 64];
    let buflen = buf.len();
    loop {
        match response.body.recv(&mut buf, buflen) {
            Ok(len) => {
                file.write(&buf[0..len])?;
            },
            Err(err) => {
                if err == ChannelError::EndOfStream {
                    break;
                } else {
                    return Err(Error::App(AppError::IMAGE_DOWNLOAD_FAILED));
                }
            },
        }
    }

    Ok(())
}

fn apply_update(filename: &str, imagename: &str) -> Result<(), Error> {

    // 1) Get file info for update binary
    let info = File::get_info(filename)?;
    println!("Found update {} of {} bytes.", filename, info.file_length);

    // 2) Open image file for writing
    let mut image = File::create(imagename, info.file_length as usize, false)?;

    // 3) Copy update to image

    let mut file = File::open(filename)?;
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

        len += image.write(&buf[0..buflen])?;
    }

    println!("Wrote {} bytes to {}.", len, imagename);

    // 4) Commit
    Update::commit();

    Ok(())
}

fn update_board() -> Result<(), Error> {

    let imagename = "/sys/mcuimg.bin";
    let filename = "/update/mcuimg.bin";

    ignore!(File::remove(filename));

    if let Err(err) = get_update(filename, SIMPLE_OTA_URL) {
        ignore!(File::remove(filename));
        return Err(err);
    };

    apply_update(filename, imagename)?;

    println!("Press RESET to run updated image...");
    Ok(())
}

fn wlan_station_mode() -> Result<(), Error> {
    SimpleLink::init_app_variables();

    configure_simple_link_to_default()?;
    let mode = SimpleLink::start()?;
    if mode != WlanMode::ROLE_STA {
        return Err(Error::App(AppError::DEVICE_NOT_IN_STATION_MODE));
    }
    println!("Device started as STATION");

    wlan_connect()?;

    println!("Connection established w/ AP and IP is acquired");
    Ok(())
}

fn http_demo() -> Result<(), Error> {

    Board::led_configure(&[LedEnum::LED1]);

    SimpleLink::start_spawn_task()?;
    wlan_station_mode()?;

    update_board()?;

    // Power off the network processor.
    SimpleLink::stop(simplelink::SL_STOP_TIMEOUT)?;
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

    println!("Welcome to CC3200 Simple OTA {}", VERSION);

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match http_demo() {
                    Ok(())  => { println!("simple_ota succeeded"); },
                    Err(e)  => { println!("simple_ota failed: {:?}", e); },
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
