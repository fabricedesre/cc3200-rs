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

use cc3200::config;
use cc3200::cc3200::{Board, LedEnum, LedName};
use cc3200::simplelink::{self, NetConfigSet, Policy, SimpleLink, SimpleLinkError,
                         WlanConfig, WlanMode, WlanRxFilterOp, WlanRxFilterOpBuf};
use cc3200::socket_channel::SocketChannel;

use freertos_rs::{CurrentTask, Duration, Task};
use smallhttp::Client;

static VERSION: &'static str = "1.0";

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AppError {
    DEVICE_NOT_IN_STATION_MODE,
    PING_FAILED,
    INTERNET_CONNECTION_FAILED,
    LAN_CONNECTION_FAILED,
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

fn configure_simple_link_to_default() -> Result<(), Error> {
    let mode = try!(SimpleLink::start());
    if mode != WlanMode::ROLE_STA {
        if mode == WlanMode::ROLE_AP {
            // If the device is in AP mode, then we need to wait for the
            // acquired event before doing anything.

            while !SimpleLink::is_ip_acquired() {
                CurrentTask::delay(Duration::ms(100));
            }
        }

        // Switch to STA mode and restart

        try!(SimpleLink::wlan_set_mode(WlanMode::ROLE_STA));
        try!(SimpleLink::stop(255));
        let mode = try!(SimpleLink::start());
        if mode != WlanMode::ROLE_STA {
            return Err(Error::App(AppError::DEVICE_NOT_IN_STATION_MODE));
        }
    }

    // Get the device's version-information
    let ver = SimpleLink::get_version();

    println!("Host Driver Version: {}", SimpleLink::get_driver_version());
    println!("Build Version {}.{}.{}.{}.31.{}.{}.{}.{}.{}.{}.{}.{}",
             ver.nwp_version[0], ver.nwp_version[1], ver.nwp_version[2], ver.nwp_version[3],
             ver.fw_version[0], ver.fw_version[1], ver.fw_version[2], ver.fw_version[3],
             ver.phy_version[0], ver.phy_version[1], ver.phy_version[2], ver.phy_version[3]);

    // Set connection policy to Auto + SmartConfig
    //      (Device's default connection policy)
    try!(SimpleLink::wlan_set_policy(Policy::ConnectionDefault, &[]));

    // Remove all profiles
    try!(SimpleLink::wlan_delete_profile(0xff));

    // Device is in station mode. Disconnect previous connection, if any.
    if SimpleLink::wlan_disconnect().is_ok() {
        // This means that we were previously connected. Wait for the
        // notification event.
        while !SimpleLink::is_connected() {
            CurrentTask::delay(Duration::ms(100));
        }
    }

    // Enable DHCP client
    try!(SimpleLink::netcfg_set(NetConfigSet::Ipv4StaP2pClientDhcpEnable, &[1]));

    // Disable Scan
    try!(SimpleLink::wlan_set_policy(Policy::ScanDisable, &[]));

    // Set Tx power level for station mode
    // Number between 0-15, as dB offset from max power - 0 will set max power

    try!(SimpleLink::wlan_set(WlanConfig::GeneralStaTxPower, &[0]));

    // Set PM policy to normal
    try!(SimpleLink::wlan_set_policy(Policy::PowerNormal, &[]));

    // Unregister mDNS services
    try!(SimpleLink::netapp_mdns_unregister_service(""));

    // Remove  all 64 filters (8*8)

    let all_filters = WlanRxFilterOpBuf::all_filters();
    try!(SimpleLink::wlan_rx_filter(WlanRxFilterOp::Remove, &all_filters));

    try!(SimpleLink::stop(simplelink::SL_STOP_TIMEOUT));

    SimpleLink::init_app_variables();
    Ok(())
}

fn wlan_connect() -> Result<(), Error> {

    let sec_params = config::security_params();

    try!(SimpleLink::wlan_connect(config::SSID, &[], sec_params, None));

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

fn wlan_station_mode() -> Result<(), Error> {
    SimpleLink::init_app_variables();

    try!(configure_simple_link_to_default());
    let mode = try!(SimpleLink::start());
    if mode != WlanMode::ROLE_STA {
        return Err(Error::App(AppError::DEVICE_NOT_IN_STATION_MODE));
    }
    println!("Device started as STATION");

    try!(wlan_connect());

    println!("Connection established w/ AP and IP is aquired");

    println!("Will now send temperature to server...");

    let mut client = Client::new(SocketChannel::new());


    Ok(())
}

fn http_demo() -> Result<(), Error> {

    Board::led_configure(&[LedEnum::LED1]);

    try!(SimpleLink::start_spawn_task());
    try!(wlan_station_mode());

    // Power off the network processor.
    try!(SimpleLink::stop(simplelink::SL_STOP_TIMEOUT));
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

    println!("Welcome to CC3200 HTTP Client {}", VERSION);

    let _client = {
        Task::new()
            .name("client")
            .stack_size(2048) // 32-bit words
            .start(|| {
                match http_demo() {
                    Ok(())  => { println!("http_demo succeeded"); },
                    Err(e)  => { println!("http_demo failed: {:?}", e); },
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
