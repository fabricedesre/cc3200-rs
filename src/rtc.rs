// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use cc3200_sys;

// When the system is reset, we set the RTC to RTC_UNSET_EPOCH, which
// corresponds to Jan 1, 2010. We can timestamp samples using this and
// once we connect to the server we can correct the timestamps prior to
// sending.
//
// By using Jan 1, 2010, this allows our RTC to wrap (which will happen
// in 2038) and we can get an additional 40 years. So this means we can
// represent all times up until 2078 using a 32-bit number.
//
// 0x00000000 thru 1262303999 map to dates from 2038 thru 2078
// 1262304000 thru 0xffffffff map to dates from Jan 1, 2010 thru to 2038
const RTC_UNSET_EPOCH: u32 = 1262304000;

pub struct RTC {}

impl RTC {
    pub fn init() {
        unsafe {
            cc3200_sys::PRCMRTCInUseSet();
        } // Indicate that we're using the RTC

        RTC::set(RTC_UNSET_EPOCH as u64);
    }

    pub fn set(seconds: u64) {
        unsafe {
            cc3200_sys::PRCMRTCSet((seconds & 0xffffffff) as u32, 0);
        }
    }

    pub fn get() -> u64 {
        let mut seconds: u32 = 0;
        let mut msecs: u16 = 0;

        unsafe {
            cc3200_sys::PRCMRTCGet(&mut seconds, &mut msecs);
        }

        if seconds < RTC_UNSET_EPOCH {
            0x100000000 + (seconds as u64)
        } else {
            seconds as u64
        }
    }
}
