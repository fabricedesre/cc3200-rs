use simplelink::SlSecParams;

pub const SSID: &'static str = "YOUR-SSID-HERE";

pub fn security_params() -> SlSecParams {
    SlSecParams::wpa2("YOUR-PASSWORD-HERE")
}

