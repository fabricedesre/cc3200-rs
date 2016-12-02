use core::convert::TryFrom;
use core::fmt;

#[derive(Debug)]
pub enum SimpleLinkError {
    Wlan(self::WlanError),
    Osi(self::OsiError),
    FileSystem(self::FileSystemError),
    ValueError(&'static str, i32),
}

impl From<self::WlanError> for SimpleLinkError {
    fn from(err: self::WlanError) -> SimpleLinkError {
        SimpleLinkError::Wlan(err)
    }
}

impl From<self::OsiError> for SimpleLinkError {
    fn from(err: self::OsiError) -> SimpleLinkError {
        SimpleLinkError::Osi(err)
    }
}

impl From<self::FileSystemError> for SimpleLinkError {
    fn from(err: self::FileSystemError) -> SimpleLinkError {
        SimpleLinkError::FileSystem(err)
    }
}

impl fmt::Display for SimpleLinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimpleLinkError::Wlan(e) => write!(formatter, "WlanError: {:?}", e),
            SimpleLinkError::Osi(e) => write!(formatter, "OsiError: {:?}", e),
            SimpleLinkError::FileSystem(e) => write!(formatter, "FileSystemError: {:?}", e),
            SimpleLinkError::ValueError(ref enum_name, n) => {
                write!(formatter,
                       "ValueError: Unknown enum value: {} for {}",
                       n,
                       enum_name)
            }
        }
    }
}

// Should this go in a macros.rs file?
#[macro_export]
macro_rules! c_like_enum {
    ( $name: ident { $($variant: ident = $value: expr),+ } ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant = $value,)+
        }

        impl TryFrom<i16> for $name {
            type Err = SimpleLinkError;
            fn try_from(value: i16) -> Result<Self, Self::Err> {
                match value {
                    $($value => Ok($name::$variant),)+
                    n => Err(SimpleLinkError::ValueError(stringify!($name), n as i32))
                }
            }
        }
    }
}

// Rust has a weird bug? in the macro stuff where the c_like_enum macro can't
// parse negative values. This variant works if all of the values in the enum
// are negative, and that's fine for our purposes.
#[macro_export]
macro_rules! c_like_enum_neg {
    ( $name: ident { $($variant: ident = -$value: expr),+ } ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant = -$value,)+
        }

        impl TryFrom<i16> for $name {
            type Err = SimpleLinkError;
            fn try_from(value: i16) -> Result<Self, Self::Err> {
                $name::try_from(value as i32)
            }
        }

        impl TryFrom<i32> for $name {
            type Err = SimpleLinkError;
            fn try_from(value: i32) -> Result<Self, Self::Err> {
                match value {
                    $($value => Ok($name::$variant),)+
                    n => Err(SimpleLinkError::ValueError(stringify!($name), n as i32))
                }
            }
        }
    }
}

c_like_enum! {
    WlanMode {
        ROLE_STA = 0,
        ROLE_UNKNOWN = 1,
        ROLE_AP = 2,
        ROLE_P2P = 3
    }
}

c_like_enum_neg! {
    WlanError {
        ROLE_STA_ERR = -1,
        ROLE_AP_ERR = -2,
        KEY_ERROR = -3,
        INVALID_ROLE = -71,
        INVALID_SECURITY_TYPE = -84,
        PASSPHRASE_TOO_LONG = -85,
        WPS_NO_PIN_OR_WRONG_PIN_LEN = -87,
        EAP_WRONG_METHOD = -88,
        PASSWORD_ERROR = -89,
        EAP_ANONYMOUS_LEN_ERROR = -90,
        SSID_LEN_ERROR = -91,
        USER_ID_LEN_ERROR = -92,
        ILLEGAL_WEP_KEY_INDEX = -95,
        INVALID_DWELL_TIME_VALUES = -96,
        INVALID_POLICY_TYPE = -97,
        PM_POLICY_INVALID_OPTION = -98,
        PM_POLICY_INVALID_PARAMS = -99,
        WIFI_ALREADY_DISCONNECTED = -129,
        WIFI_NOT_CONNECTED = -59
    }
}

pub const SPAWN_TASK_PRIORITY: u32 = 9;
pub const SL_STOP_TIMEOUT: u16 = 200;

c_like_enum_neg! {
    OsiError {
        OSI_FAILURE = -1,
        OSI_OPERATION_FAILED = -2,
        OSI_ABORTED = -3,
        OSI_INVALID_PARAMS = -4,
        OSI_MEMORY_ALLOCATION_FAILURE = -5,
        OSI_TIMEOUT = -6,
        OSI_EVENTS_IN_USE = -7,
        OSI_EVENT_OPEARTION_FAILURE = -8
    }
}

c_like_enum! {
    SocketFamily {
        AF_INET = 2,
        AF_INET6 = 3,
        AF_INET6_EUI_48 = 9,
        AF_RF = 6,
        AF_PACKET = 17
    }
}

c_like_enum! {
    StatusBit {
        // If this bit is set: Network Processor is powered up
        STATUS_BIT_NWP_INIT = 0,

        // If this bit is set: the device is connected to the AP or
        // client is connected to device (AP)
        STATUS_BIT_CONNECTION = 1,

        // If this bit is set: the device has leased IP to any connected client
        STATUS_BIT_IP_LEASED = 2,

        // If this bit is set: the device has acquired an IP
        STATUS_BIT_IP_AQUIRED = 3,

        // If this bit is set: the SmartConfiguration process is
        // started from SmartConfig app
        STATUS_BIT_SMARTCONFIG_START = 4,

        // If this bit is set: the device (P2P mode) found any p2p-device in scan
        STATUS_BIT_P2P_DEV_FOUND = 5,

        // If this bit is set: the device (P2P mode) found any p2p-negotiation request
        STATUS_BIT_P2P_REQ_RECEIVED = 6,

        // If this bit is set: the device(P2P mode) connection to client(or reverse way) is failed
        STATUS_BIT_CONNECTION_FAILED = 7,

        // If this bit is set: the device has completed the ping operation
        STATUS_BIT_PING_DONE = 8
    }
}

// Encode the PolicyType and Policy into a single value. The policy type is
// in bits 8-15, and the policy in bits 0-7.
c_like_enum! {
    Policy {
        // Need to figure out a better way to encode SL_CONNECTION_POLICY from wlan.h
        // 0x01 - Auto
        // 0x02 - Fast
        // 0x04 - Open
        // 0x08 - anyP2P
        // 0x10 - autoSmartConfig
        ConnectionDefault = 0x1011,

        ScanDisable = 0x2000,
        ScanEnable = 0x2001,

        PowerNormal = 0x3000,
        PowerLowLatency = 0x3001,
        PowerLowPower = 0x3002,
        PowerAlwaysOn = 0x3003,
        PowerLongSleepInterval = 0x3004
    }
}

// Encodes the ConfigId and ConfigOpt into a single value.
// The ConfigId is in  in bits 8-15 and ConfigOpt is in bits 0-7
c_like_enum! {
    NetConfigSet {
        MacAddress = 0x0101,
        Ipv4StaP2pClientDhcpEnable = 0x0401,
        Ipv4StaP2pClientStaticEnable = 0x0501,
        Ipv4ApP2pGoStaticEnable = 0x0701,
        SetHostRxAggr = 0x0800,
        Ipv4DnsClient = 0x0a00,
        Ipv4ArpFlush = 0x0b00
    }
}

pub const SL_MAC_ADDR_LEN: usize = 6;

// NetConfigGet only has an Id.
c_like_enum! {
    NetConfigGet {
        MacAddress = 0x02,
        Ipv4StaP2pClientGetInfo = 0x03,
        Ipv4ApP2pGoGetInfo = 0x06,
        Ipv4DhcpClient = 0x09,
        Ipv4DnsClient = 0x0a
    }
}

// Encodes the ConfigId and ConfigOpt into a single value.
// The ConfigId is in  in bits 8-15 and ConfigOpt is in bits 0-7
c_like_enum! {
    WlanConfig {
        ApSsid = 0x0000,
        ApChannel = 0x0003,
        ApHiddenSsid = 0x0004,
        ApSecurityType = 0x0006,
        ApPassword = 0x0007,
        GeneralCountryCode = 0x0109,
        GeneralStaTxPower = 0x010a,
        GeneralApTxPower  = 0x010b,
        P2pDevName = 0x020c,
        P2pDevType = 0x020d,
        P2pChannelRegs = 0x020e,
        GeneralInfoElement = 0x0110,
        GeneralScanParams = 0x0112      // Change scan channels and RSSI threshold using this config
    }
}

c_like_enum! {
    WlanRxFilterOp {    // The following ops take a WlanRxFilterOpBuf argument
        EnableDisable = 0,
        Remove = 1,
        Store = 2
    }
}

#[repr(C)]
pub struct WlanRxFilterOpBuf {
    pub mask: [u8; 128 / 8],
    padding: [u8; 4],
}

impl WlanRxFilterOpBuf {
    pub fn all_filters() -> Self {
        WlanRxFilterOpBuf {
            mask: [0xff; 16],
            padding: [0; 4],
        }
    }
}

c_like_enum! {
    SecurityType {
        Open = 0,
        Wep = 1,
        Wpa2 = 2,
        WpsPushButtonConfig = 3,
        WpsPin = 4,
        WpaEnterprise = 5,
        P2pPushButtonConfig = 6,
        P2pPinKeypad = 7,
        P2pPinDisplay = 8
    }
}

#[repr(C)]
pub struct SlSecParams {
    pub sec_type: u8,
    pub key: *const u8,
    pub key_len: u8,
}

impl SlSecParams {
    pub fn wpa2(key: &str) -> Self {
        SlSecParams {
            sec_type: SecurityType::Wpa2 as u8,
            key: key.as_ptr(),
            key_len: key.len() as u8,
        }
    }
}

#[repr(C)]
pub struct SlSecParamsExt {
    pub user: *const u8,
    pub user_len: u8,
    pub anon_user: *const u8,
    pub anon_user_len: u8,
    pub cert_index: u8, // not supported
    pub eap_method: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct SlVersionFull {
    pub chip_id: u32,
    pub fw_version: [u32; 4],
    pub phy_version: [u8; 4],
    pub nwp_version: [u32; 4],
    pub rom_version: u16,
    padding: u16,
}

#[repr(C)]
#[derive(Default)]
pub struct SlPingReport {
    pub packets_sent: u32,
    pub packets_rcvd: u32,
    pub min_round_time: u16,
    pub max_round_time: u16,
    pub avg_round_time: u16,
    pub test_time: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct SlPingStartCommand {
    pub ping_interval_time: u32, // delay between pings, in milliseconds
    pub ping_size: u16, // ping packet size in bytes
    pub ping_request_timeout: u16, // timeout time for every ping in milliseconds
    pub total_number_of_attempts: u32, // max number of ping requests. 0 = forever
    pub flags: u32, /* flag - 0 report only when finished, 1 - return response for
                     * every ping, 2 - stop after 1 successful ping. */
    pub ip: u32, // IPv4 address
    pub ip1_or_padding: u32,
    pub ip2_or_padding: u32,
    pub ip3_or_padding: u32,
}

c_like_enum_neg! {
    FileSystemError {
        NOT_SUPPORTED = -1,
        FAILED_TO_READ = -2,
        INVALID_MAGIC_NUM = -3,
        DEVICE_NOT_LOADED = -4,
        FAILED_TO_CREATE_LOCK_OBJ = -5,
        UNKNOWN = -6,
        FS_ALREADY_LOADED = -7,
        FAILED_TO_CREATE_FILE = -8,
        INVALID_ARGS = -9,
        EMPTY_ERROR = -10,
        FILE_NOT_EXISTS = -11,
        INVALID_FILE_ID = -12,
        READ_DATA_LENGTH = -13,
        ALLOC = -14,
        OFFSET_OUT_OF_RANGE = -15,
        FAILED_TO_WRITE = -16,
        INVALID_HANDLE = -17,
        FAILED_LOAD_FILE = -18,
        CONTINUE_WRITE_MUST_BE_MOD_4 = -19,
        FAILED_INIT_STORAGE = -20,
        FAILED_READ_NVFILE = -21,
        BAD_FILE_MODE = -22,
        FILE_ACCESS_IS_DIFFERENT = -23,
        NO_ENTRIES_AVAILABLE = -24,
        PROGRAM = -25,
        FILE_ALREADY_EXISTS = -26,
        INVALID_ACCESS_TYPE = -27,
        FILE_EXISTS_ON_DIFFERENT_DEVICE_ID = -28,
        FILE_MAX_SIZE_BIGGER_THAN_EXISTING_FILE = -29,
        NO_AVAILABLE_BLOCKS = -30,
        FAILED_TO_READ_INTEGRITY_HEADER_1 = -31,
        FAILED_TO_READ_INTEGRITY_HEADER_2 = -32,
        FAILED_TO_ALLOCATE_MEM = -33,
        NO_AVAILABLE_NV_INDEX = -34,
        FAILED_WRITE_NVMEM_HEADER = -35,
        DEVICE_IS_NOT_FORMATTED = -36,
        WARNING_FILE_NAME_NOT_KEPT = -37,
        SIZE_OF_FILE_EXT_EXCEEDED = -38,
        FILE_IMAGE_IS_CORRUPTED = -39,
        INVALID_BUFFER_FOR_WRITE = -40,
        INVALID_BUFFER_FOR_READ = -41,
        FILE_MAX_SIZE_EXCEEDED = -42,
        MAX_FS_FILES_IS_SMALLER = -43,
        MAX_FS_FILES_IS_LARGER = -44,
        FILE_HAS_RESERVED_NV_INDEX = -45,
        OVERLAP_DETECTION_THRESHHOLD = -46,
        DATA_IS_NOT_ALIGNED = -47,
        DATA_ADDRESS_SHOUD_BE_IN_DATA_RAM = -48,
        NO_DEVICE_IS_LOADED = -49,
        TOKEN_IS_NOT_VALID = -50,
        FILE_UNVALID_FILE_SIZE = -51,
        SECURITY_ALLERT = -52,
        FILE_SYSTEM_IS_LOCKED = -53,
        WRONG_FILE_NAME = -54,
        FAILED_READ_NVMEM_HEADER = -55,
        INCORRECT_OFFSET_ALIGNMENT = -56,
        SECURE_FILE_MUST_BE_COMMIT = -57,
        SECURITY_BUF_ALREADY_ALLOC = -58,
        FILE_NAME_EXIST = -59,
        CERT_CHAIN_ERROR = -60,
        NOT_16_ALIGNED = -61,
        WRONG_SIGNATURE_OR_CERTIFIC_NAME_LENGTH = -62,
        WRONG_SIGNATURE = -63,
        FILE_HAS_NOT_BEEN_CLOSE_CORRECTLY = -64,
        ERASING_FLASH = -65,
        FILE_IS_NOT_SECURE_AND_SIGN = -66,
        EMPTY_SFLASH = -67
    }
}

#[repr(C)]
pub struct SlFsFileInfo {
    pub flags: u16,
    pub file_length: u32,
    pub allocated_length: u32,
    pub token: [u32; 4]
}

extern "C" {
    // From simplelink/device.h

    pub fn sl_Start(handle: *const i32,
                    dev_name: *const i8,
                    callback: Option<extern "C" fn(status: u32)>)
                    -> i16;
    pub fn sl_Stop(timeout: u16) -> i16;

    // From simplelink/socket.h

    // From simplelink/wlan.h

    pub fn sl_WlanSet(config_id: u16, config_opt: u16, len: u16, val: *const u8) -> i16;
    pub fn sl_WlanSetMode(mode: u8) -> i16;
    pub fn sl_WlanPolicySet(typ: u8, policy: u8, val: *const u8, len: u8) -> i16;
    pub fn sl_WlanProfileDel(index: i16) -> i16;
    pub fn sl_WlanDisconnect() -> i16;

    pub fn sl_WlanConnect(ssid: *const u8,
                          ssid_len: i16,
                          mac_addr: *const u8,
                          sec_params: *const SlSecParams,
                          sec_params_ext: *const SlSecParamsExt)
                          -> i16;

    // From simplelink/wlan_rx_filter.h

    pub fn sl_WlanRxFilterSet(op: u8, buf: *const u8, len: u16) -> i16;

    // From simplelink/netcfg.h

    pub fn sl_NetCfgSet(config_id: u8, config_opt: u8, len: u8, val: *const u8) -> i32;
    pub fn sl_NetCfgGet(config_id: u8, config_opt: *mut u8, len: *mut u8, val: *mut u8) -> i32;

    // From simplelink/netapp.h

    pub fn sl_NetAppDnsGetHostByName(name: *const u8,
                                     name_len: u16,
                                     out_ip_addr: *mut u32,
                                     family: u8)
                                     -> i16;
    pub fn sl_NetAppMDNSUnRegisterService(name: *const u8, len: u8) -> i16;
    pub fn sl_NetAppPingStart(ping_params: *const SlPingStartCommand,
                              famliy: u8,
                              report: *mut SlPingReport,
                              callback: Option<unsafe extern "C" fn(report: *mut SlPingReport)>)
                              -> i16;

    // From oslink/osi_freetros.c

    pub fn VStartSimpleLinkSpawnTask(priority: u32) -> i32;

    // From simplelink.c (not in SDK)

    pub fn simplelink_init_app_variables();

    pub fn simplelink_get_status_bit(bit: u32) -> bool;
    pub fn simplelink_set_status_bit(bit: u32);
    pub fn simplelink_clear_status_bit(bit: u32);

    pub fn simplelink_get_version(version: *mut SlVersionFull);
    pub fn simplelink_get_driver_version(len: *mut u32) -> *const u8;

    pub fn simplelink_gateway_ip() -> u32;
    pub fn simplelink_ping_packets_received() -> u32;

    pub fn SimpleLinkPingReport(report: *mut SlPingReport);

    // File System
    //

    // From simplelink/fs.h

    pub fn sl_FsOpen(file_name: *const u8,
                     mode: u32,
                     token: *const u32,
                     file_handle: *mut i32) -> i32;
    pub fn sl_FsClose(file_handle: i32,
                      certificate_file_name: *const u8,
                      signature: *const u8,
                      signature_length: u32) -> i16;
    pub fn sl_FsRead(file_handle: i32,
                     offset: u32,
                     data: *mut u8,
                     len: u32) -> i32;
    pub fn sl_FsWrite(file_handle: i32,
                      offset: u32,
                      data: *const u8,
                      len: u32) -> i32;
    pub fn sl_FsGetInfo(filename: *const u8,
                        token: u32,
                        file_info: *mut SlFsFileInfo) -> i16;
    pub fn sl_FsDel(filename: *const u8, token: u32) -> i16;

    // simplelink.c

    pub fn sl_FsMode(write: bool, create: bool, failsafe: bool, max_size: u32) -> u32;
}
