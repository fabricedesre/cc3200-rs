// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate cc3200_sys;

use core::convert::TryFrom;
use core::mem;
use core::ptr;
use core::slice;
use core::str;
use collections::String;
use freertos_rs::{Duration, Mutex, MutexGuard};

use numeric_utils::{format_bssid_into, format_ip_as_string};
use cc3200_sys::socket;

pub use self::cc3200_sys::simplelink::*;

macro_rules! try_wlan {
    ($e:expr) => ({
        let rc: i16 = unsafe { $e };
        if rc < 0 {
            return Err(SimpleLinkError::Wlan(try!(WlanError::try_from(rc))));
        }
        rc
    })
}

// UNIQUE_ID is set exactly once, and it never changes after that, so we
// don't need to use a Mutex.
lazy_static! {
    static ref UNIQUE_ID: u64 = {
        let mut mac_addr: [u8; SL_MAC_ADDR_LEN] = [0; SL_MAC_ADDR_LEN];
        if SimpleLink::netcfg_get_mac_addr(&mut mac_addr).is_ok() {
            ((mac_addr[0] as u64) << 40) | ((mac_addr[1] as u64) << 32) |
            ((mac_addr[2] as u64) << 24) | ((mac_addr[3] as u64) << 16) |
            ((mac_addr[4] as u64) << 8) | (mac_addr[5] as u64)
        } else {
            0
        }
    };
}

pub struct SimpleLinkGlobals {
    inner: Mutex<SimpleLinkGlobalsInner>,
}

#[derive(Default)]
pub struct SimpleLinkGlobalsInner {
    connection_ssid_len: u8,
    connection_ssid_buf: [u8; 32],
    connection_bssid: [u8; 6],
    ping_packets_rcvd: u32,
    gateway_ip: u32,

    // TODO: See if we can replace status with event flags.
    status: u32,
}

impl SimpleLinkGlobalsInner {
    pub fn clear_all(&mut self) {
        self.clear_connection_ssid();
        self.clear_connection_bssid();
        self.ping_packets_rcvd = 0;
        self.gateway_ip = 0;
        self.status = 0;
    }

    pub fn connection_ssid(&self) -> String {
        String::from_utf8_lossy(&self.connection_ssid_buf[0..self.connection_ssid_len as usize])
            .into_owned()
    }

    pub fn clear_connection_ssid(&mut self) {
        self.connection_ssid_len = 0;
        self.connection_ssid_buf = [0; 32];
    }

    pub fn clear_connection_bssid(&mut self) {
        self.connection_bssid = [0; 6];
    }

    pub fn get_status_bit(&self, status_bit: StatusBit) -> bool {
        (self.status & (1 << (status_bit as u32))) != 0
    }

    pub fn set_status_bit(&mut self, status_bit: StatusBit) {
        self.status |= 1 << (status_bit as u32);
    }

    pub fn clear_status_bit(&mut self, status_bit: StatusBit) {
        self.status &= !(1 << (status_bit as u32));
    }
}

lazy_static! {
    static ref GLOBALS: SimpleLinkGlobals = SimpleLinkGlobals {
        inner: Mutex::new(SimpleLinkGlobalsInner {..Default::default() }).unwrap(),
    };
}

impl SimpleLinkGlobals {
    pub fn lock(&self) -> MutexGuard<SimpleLinkGlobalsInner> {
        self.inner.lock(Duration::infinite()).unwrap()
    }

    // TODO: Maybe we should create an SSID struct?
    pub fn connection_ssid(&self) -> String {
        self.lock().connection_ssid()
    }
}

pub struct SimpleLink { }

impl SimpleLink {
    pub fn start_spawn_task() -> Result<(), SimpleLinkError> {
        let rc = unsafe { VStartSimpleLinkSpawnTask(SPAWN_TASK_PRIORITY) };
        if rc < 0 {
            Err(SimpleLinkError::Osi(try!(OsiError::try_from(rc))))
        } else {
            Ok(())
        }
    }

    pub fn get_version() -> SlVersionFull {
        let mut version: SlVersionFull = Default::default();
        let mut ver_slice: &mut [u8] =
            unsafe {
                slice::from_raw_parts_mut(&mut version as *mut _ as *mut u8,
                                          mem::size_of_val(&version))
            };
        SimpleLink::dev_cfg_get(DeviceConfig::GeneralConfig, &mut ver_slice);
        version
    }

    pub fn get_driver_version() -> &'static str {
        let mut ver_len: u32 = 0;
        unsafe {
            let ver_ptr = simplelink_get_driver_version(&mut ver_len);
            str::from_utf8_unchecked(slice::from_raw_parts(ver_ptr, ver_len as usize))
        }
    }

    // App Variables

    pub fn init_app_variables() {
        let ref mut globals = GLOBALS.lock();
        globals.clear_all();
    }

    pub fn is_ip_acquired() -> bool {
        // get_status_bit(StatusBit::STATUS_BIT_IP_ACQUIRED)
        GLOBALS.lock().get_status_bit(StatusBit::STATUS_BIT_IP_ACQUIRED)
    }

    pub fn is_connected() -> bool {
        // get_status_bit(StatusBit::STATUS_BIT_CONNECTION)
        GLOBALS.lock().get_status_bit(StatusBit::STATUS_BIT_CONNECTION)
    }

    pub fn is_ping_done() -> bool {
        // get_status_bit(StatusBit::STATUS_BIT_PING_DONE)
        GLOBALS.lock().get_status_bit(StatusBit::STATUS_BIT_PING_DONE)
    }

    pub fn clear_ping_done() {
        // clear_status_bit(StatusBit::STATUS_BIT_PING_DONE)
        GLOBALS.lock().clear_status_bit(StatusBit::STATUS_BIT_PING_DONE)
    }

    pub fn gateway_ip() -> u32 {
        GLOBALS.lock().gateway_ip
    }

    pub fn ping_packets_received() -> u32 {
        GLOBALS.lock().ping_packets_rcvd
    }

    // Device

    pub fn start() -> Result<WlanMode, SimpleLinkError> {

        // This will trigger initialization of the GLOBALS, which will in turn
        // create the Mutex
        let _ = *GLOBALS;

        let rc = try_wlan!(sl_Start(ptr::null(), ptr::null(), None));
        Ok(try!(WlanMode::try_from(rc)))
    }

    pub fn stop(timeout_msecs: u16) -> Result<WlanMode, SimpleLinkError> {
        let rc = try_wlan!(sl_Stop(timeout_msecs));
        Ok(try!(WlanMode::try_from(rc)))
    }

    pub fn unique_id() -> u64 {
        *UNIQUE_ID
    }

    pub fn dev_cfg_get(config: DeviceConfig, result: &mut [u8]) {
        let config_id = ((config as u32 & 0xff00) >> 8) as u8;
        let mut config_opt = (config as u32 & 0x00ff) as u8;
        let mut result_len = result.len() as u8;
        unsafe {
            sl_DevGet(config_id,
                      &mut config_opt,
                      &mut result_len,
                      result.as_mut_ptr());
        }
    }

    // Net App

    pub fn netapp_get_host_by_name(name: &str) -> Result<u32, SimpleLinkError> {
        let mut out_ip_addr: u32 = 0;
        try_wlan!(sl_NetAppDnsGetHostByName(name.as_ptr(),
                                            name.len() as u16,
                                            &mut out_ip_addr as *mut u32,
                                            SocketFamily::AF_INET as u8));
        Ok(out_ip_addr)
    }

    pub fn netapp_mdns_unregister_service(name: &str) -> Result<(), SimpleLinkError> {
        let name_len = name.len() as u8;
        let name_ptr = {
            if name_len > 0 {
                name.as_ptr()
            } else {
                ptr::null()
            }
        };
        try_wlan!(sl_NetAppMDNSUnRegisterService(name_ptr, name_len));
        Ok(())
    }

    pub fn netapp_ping_start(ping_params: &SlPingStartCommand,
                             family: SocketFamily)
                             -> Result<(), SimpleLinkError> {
        let params_ptr = ping_params as *const SlPingStartCommand;

        // Since we're provinding a callback, the ping_report parameter is ignored.
        try_wlan!(sl_NetAppPingStart(params_ptr,
                                     family as u8,
                                     ptr::null_mut(),
                                     Some(SimpleLinkPingReport)));
        Ok(())
    }

    // Net Config

    pub fn netcfg_set(config: NetConfigSet, val: &[u8]) -> Result<(), SimpleLinkError> {
        let config_id = ((config as u32 & 0xff00) >> 8) as u8;
        let config_opt = (config as u32 & 0x00ff) as u8;
        try_wlan!(sl_NetCfgSet(config_id, config_opt, val.len() as u8, val.as_ptr()) as i16);
        Ok(())
    }

    pub fn netcfg_get(config: NetConfigGet,
                      config_opt: Option<*mut u8>,
                      result: &mut [u8])
                      -> Result<&mut [u8], SimpleLinkError> {
        let config_opt_ptr = if let Some(p) = config_opt {
            p
        } else {
            ptr::null_mut()
        };
        let mut result_len = result.len() as u8;
        try_wlan!(sl_NetCfgGet(config as u8,
                               config_opt_ptr,
                               &mut result_len,
                               result.as_mut_ptr()) as i16);
        Ok(&mut result[0..result_len as usize])
    }

    pub fn netcfg_get_mac_addr(mac_addr: &mut [u8; self::SL_MAC_ADDR_LEN])
                               -> Result<&mut [u8], SimpleLinkError> {
        SimpleLink::netcfg_get(NetConfigGet::MacAddress, None, mac_addr)
    }

    // WLAN

    pub fn wlan_delete_profile(index: i16) -> Result<(), SimpleLinkError> {
        try_wlan!(sl_WlanProfileDel(index));
        Ok(())
    }

    pub fn wlan_connect(ssid: &str,
                        mac_addr: &[u8],
                        sec_params: Option<SlSecParams>,
                        sec_params_ext: Option<SlSecParamsExt>)
                        -> Result<(), SimpleLinkError> {
        let ssid_ptr = ssid.as_ptr();
        let ssid_len = ssid.len() as i16;
        let mac_addr_len = mac_addr.len();
        let mac_addr_ptr = if mac_addr_len > 0 {
            mac_addr.as_ptr()
        } else {
            ptr::null()
        };
        let sec_params_ptr =
            sec_params.as_ref().map(|r| r as *const SlSecParams).unwrap_or(ptr::null());
        let sec_params_ext_ptr =
            sec_params_ext.as_ref().map(|r| r as *const SlSecParamsExt).unwrap_or(ptr::null());
        try_wlan!(sl_WlanConnect(ssid_ptr,
                                 ssid_len,
                                 mac_addr_ptr,
                                 sec_params_ptr,
                                 sec_params_ext_ptr));
        Ok(())
    }

    pub fn wlan_disconnect() -> Result<(), SimpleLinkError> {
        try_wlan!(sl_WlanDisconnect());
        Ok(())
    }

    pub fn wlan_set(config: WlanConfig, val: &[u8]) -> Result<(), SimpleLinkError> {
        let config_id = ((config as u32 & 0xff00) >> 8) as u16;
        let config_opt = (config as u32 & 0x00ff) as u16;
        try_wlan!(sl_WlanSet(config_id, config_opt, val.len() as u16, val.as_ptr()));
        Ok(())
    }

    pub fn wlan_set_mode(mode: WlanMode) -> Result<WlanMode, SimpleLinkError> {
        let rc = try_wlan!(sl_WlanSetMode(mode as u8));
        Ok(try!(WlanMode::try_from(rc)))
    }

    pub fn wlan_set_policy(policy: Policy, val: &[u8]) -> Result<(), SimpleLinkError> {
        let policy_type = ((policy as u32 & 0xff00) >> 8) as u8;
        let policy_policy = (policy as u32 & 0x00ff) as u8;
        try_wlan!(sl_WlanPolicySet(policy_type, policy_policy, val.as_ptr(), val.len() as u8));
        Ok(())
    }

    // WLAN Rx Filter

    pub fn wlan_rx_filter(op: WlanRxFilterOp,
                          buf: &WlanRxFilterOpBuf)
                          -> Result<(), SimpleLinkError> {
        let buf_size = mem::size_of::<WlanRxFilterOpBuf>() as u16;
        let buf_ptr: *const u8 = buf as *const _ as *const u8;
        try_wlan!(sl_WlanRxFilterSet(op as u8, buf_ptr, buf_size));
        Ok(())
    }
}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkGeneralEventHandler(dev_event: *const SlDeviceEvent) {
    let dev_event = unsafe { &*dev_event };
    match SlDeviceDriverError::try_from(dev_event.event_num) {

        Ok(SlDeviceDriverError::SL_DEVICE_GENERAL_ERROR_EVENT) => {
            let device_event = unsafe { &dev_event.event_data.device_event };
            println!("[GENERAL EVENT] ID=[{}] Sender=[{}]",
                     device_event.status,
                     device_event.sender);
        }

        Ok(SlDeviceDriverError::SL_DEVICE_ABORT_ERROR_EVENT) => {
            let device_report = unsafe { &dev_event.event_data.device_report };
            println!("[ABORT EVENT] Type=[{}] Data=[{}]",
                     device_report.abort_type,
                     device_report.abort_data);
        }

        Ok(SlDeviceDriverError::SL_DEVICE_DRIVER_ASSERT_ERROR_EVENT) |
        Ok(SlDeviceDriverError::SL_DEVICE_DRIVER_TIMEOUT_CMD_COMPLETE) |
        Ok(SlDeviceDriverError::SL_DEVICE_DRIVER_TIMEOUT_SYNC_PATTERN) |
        Ok(SlDeviceDriverError::SL_DEVICE_DRIVER_TIMEOUT_ASYNC_EVENT) => {
            let device_driver_report = unsafe { &dev_event.event_data.device_driver_report };
            println!("[DRIVER ERROR] Error=[{}] Info=[{}]",
                     dev_event.event_num,
                     device_driver_report.info);
        }

        Err(_) => {
            println!("[UNKNOWN EVENT] [{}]", dev_event.event_num);
        }
    }

}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkWlanEventHandler(wlan_event: *mut SlWlanEvent_t) {
    let wlan_event = unsafe { &*wlan_event };
    let ref mut globals = GLOBALS.lock();
    match WlanEvent::try_from(wlan_event.event_type as i16) {

        Ok(WlanEvent::SL_WLAN_CONNECT_EVENT) => {
            let connection_event =
                unsafe { &wlan_event.event_data.sta_and_p2p_mode_wlan_connected };

            globals.connection_ssid_len = connection_event.ssid_len;
            globals.connection_ssid_buf = connection_event.ssid_name;
            globals.connection_bssid = connection_event.bssid;

            let mut bssid_str: [u8; 17] = [0; 17];
            format_bssid_into(&mut bssid_str, globals.connection_bssid);

            println!("[WLAN EVENT] STA Connected to AP: '{}', BSSID: {}",
                     globals.connection_ssid(),
                     str::from_utf8(&bssid_str[0..17]).unwrap());

            globals.set_status_bit(StatusBit::STATUS_BIT_CONNECTION);
        }

        Ok(WlanEvent::SL_WLAN_DISCONNECT_EVENT) => {
            let disconnect_event =
                unsafe { &wlan_event.event_data.sta_and_p2p_mode_wlan_disconnected };

            globals.clear_status_bit(StatusBit::STATUS_BIT_CONNECTION);
            globals.clear_status_bit(StatusBit::STATUS_BIT_IP_ACQUIRED);

            let mut bssid_str: [u8; 17] = [0; 17];
            format_bssid_into(&mut bssid_str, globals.connection_bssid);

            match WlanDisconnectReason::try_from(disconnect_event.reason_code as u32) {
                Ok(WlanDisconnectReason::SL_WLAN_DISCONNECT_USER_INITIATED_DISCONNECTION) => {
                    println!("[WLAN EVENT]Device disconnected from the AP: '{}', BSSID: {} on \
                              applications request",
                             globals.connection_ssid(),
                             str::from_utf8(&bssid_str[0..17]).unwrap());
                }
                _ => {
                    println!("[WLAN_ERROR] Device disconnected from the AP: '{}' BSSID: {} on an \
                              ERROR..!!",
                             GLOBALS.connection_ssid(),
                             str::from_utf8(&bssid_str[0..17]).unwrap());
                }
            };

            globals.clear_connection_ssid();
            globals.clear_connection_bssid();
        }

        _ => {
            println!("Unexpected event: {}", wlan_event.event_type);
        }
    }
}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkNetAppEventHandler(net_app_event: *const SlNetAppEvent) {
    let net_app_event = unsafe { &*net_app_event };
    let ref mut globals = GLOBALS.lock();

    match NetAppEvent::try_from(net_app_event.event) {

        Ok(NetAppEvent::SL_NETAPP_IPV4_IPACQUIRED_EVENT) => {
            let event_data = unsafe { &net_app_event.event_data.ip_acquired_v4 };
            globals.gateway_ip = event_data.gateway;
            println!("[NETAPP EVENT] IP Acquired: IP: {}, Gateway: {}",
                     format_ip_as_string(event_data.ip),
                     format_ip_as_string(event_data.gateway));
            globals.set_status_bit(StatusBit::STATUS_BIT_IP_ACQUIRED);
        }

        _ => {
            println!("[NETAPP EVENT] Unexpected event [{}]", net_app_event.event);
        }
    }
}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkSockEventHandler(sock_event: *const socket::SlSockEvent) {
    let sock_event = unsafe { &*sock_event };
    match socket::SlSocketEventNum::try_from(sock_event.event_num) {
        Ok(socket::SlSocketEventNum::SL_SOCKET_TX_FAILED_EVENT) => {
            let tx_fail_data = unsafe { &sock_event.event_data.tx_fail_data };
            if tx_fail_data.status == socket::SocketError::ECLOSE as i16 {
                println!("[SOCK ERROR] close socket ({}) operation failed to transmit all queued \
                          packets",
                         tx_fail_data.sd);
            } else {
                println!("[SOCK ERROR] TX FAILED: socket ({}), reason {}",
                         tx_fail_data.sd,
                         tx_fail_data.status);
            }
        }

        Ok(socket::SlSocketEventNum::SL_SOCKET_ASYNC_EVENT) => {
            // Nothing to do.
        }

        Err(_) => {
            println!("[SOCK EVENT] Unexpected Event [{:#x}]",
                     sock_event.event_num);
        }
    };
}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkPingReport(ping_report: *mut SlPingReport) {
    let ref mut globals = GLOBALS.lock();
    globals.ping_packets_rcvd = unsafe { (*ping_report).packets_rcvd };
    globals.set_status_bit(StatusBit::STATUS_BIT_PING_DONE);
}

#[linkage = "weak"]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SimpleLinkHttpServerCallback() {
    // Unused in this application
}
