// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Intrinsics documented at http://infocenter.arm.com/help/topic/com.arm.doc.ihi0043d/IHI0043D_rtabi.pdf

#[no_mangle]
pub unsafe extern "C" fn __aeabi_dcmpeq(arg1: f64, arg2: f64) -> i32 {
    if arg1 == arg2 {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_dcmplt(arg1: f64, arg2: f64) -> i32 {
    if arg1 < arg2 {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_dcmple(arg1: f64, arg2: f64) -> i32 {
    if arg1 <= arg2 {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_dcmpgt(arg1: f64, arg2: f64) -> i32 {
    if arg1 > arg2 {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_i2d(arg: i32) -> f64 {
    arg as f64
}

#[no_mangle]
pub unsafe extern "C" fn pow(x: f64, y: f64) -> f64 {
    x + y
}
