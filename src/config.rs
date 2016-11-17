use simplelink::SlSecParams;

// Eg. OpenWireless.org
pub const SSID: &'static str = "YOUR-SSID-HERE";

pub fn security_params() -> Option<SlSecParams> {
    // If using an open access point, just return None
    Some(SlSecParams::wpa2("YOUR-PASSWORD-HERE"))
}
