extern crate cc3200_sys;

use core::convert::TryFrom;
use core::mem;
use core::ptr;
use core::slice;
use core::str;

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
        unsafe { simplelink_get_version(&mut version) };
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
        unsafe {
            simplelink_init_app_variables();
        }
    }

    pub fn is_ip_acquired() -> bool {
        unsafe { simplelink_get_status_bit(StatusBit::STATUS_BIT_IP_AQUIRED as u32) }
    }

    pub fn is_connected() -> bool {
        unsafe { simplelink_get_status_bit(StatusBit::STATUS_BIT_CONNECTION as u32) }
    }

    pub fn is_ping_done() -> bool {
        unsafe { simplelink_get_status_bit(StatusBit::STATUS_BIT_PING_DONE as u32) }
    }

    pub fn clear_ping_done() {
        unsafe { simplelink_clear_status_bit(StatusBit::STATUS_BIT_PING_DONE as u32) };
    }

    pub fn gateway_ip() -> u32 {
        unsafe { simplelink_gateway_ip() }
    }

    pub fn ping_packets_received() -> u32 {
        unsafe { simplelink_ping_packets_received() }
    }

    // Device

    pub fn start() -> Result<WlanMode, SimpleLinkError> {
        let rc = try_wlan!(sl_Start(ptr::null(), ptr::null(), None));
        Ok(try!(WlanMode::try_from(rc)))
    }

    pub fn stop(timeout_msecs: u16) -> Result<WlanMode, SimpleLinkError> {
        let rc = try_wlan!(sl_Stop(timeout_msecs));
        Ok(try!(WlanMode::try_from(rc)))
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
            ptr::null() as *const u8
        };
        let sec_params_ptr: *const SlSecParams = sec_params.map(|r| &r as *const SlSecParams)
            .unwrap_or(ptr::null() as *const SlSecParams);
        let sec_params_ext_ptr: *const SlSecParamsExt =
            sec_params_ext.map(|r| &r as *const SlSecParamsExt)
                .unwrap_or(ptr::null() as *const SlSecParamsExt);
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
