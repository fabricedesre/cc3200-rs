// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate core;

use core::cmp::min;

pub fn fill_buf(buf: &mut [u8], fill_byte: u8) {
    for ch in buf.iter_mut() {
        *ch = fill_byte;
    }
}

pub fn format_hex_into(buf: &mut [u8], num: u32) -> bool {
    if buf.len() < 1 {
        return false;
    }

    let digit: &'static [u8; 16] = b"0123456789abcdef";
    let mut buf_iter = buf.iter_mut().rev();
    let mut num = num;
    while let Some(ch) = buf_iter.next() {
        *ch = digit[(num & 0x0f) as usize];
        num >>= 4;
    }
    return true;
}

pub const FMT_MAC_ADDR_LEN: usize = 17;

pub fn format_mac_addr_into(buf: &mut [u8; FMT_MAC_ADDR_LEN], mac_addr: [u8; 6]) -> bool {
    buf.copy_from_slice(b"xx:xx:xx:xx:xx:xx");
    format_hex_into(&mut buf[0..2], mac_addr[0] as u32);
    format_hex_into(&mut buf[3..5], mac_addr[1] as u32);
    format_hex_into(&mut buf[6..8], mac_addr[2] as u32);
    format_hex_into(&mut buf[9..11], mac_addr[3] as u32);
    format_hex_into(&mut buf[12..14], mac_addr[4] as u32);
    format_hex_into(&mut buf[15..17], mac_addr[5] as u32);

    return true;
}

pub fn format_int_into(buf: &mut [u8], num: i32, fill: char) -> bool {
    if buf.len() < 1 {
        return false;
    }

    // Introduce a block so that we limit the lifetime of the borrow (of buf)
    // otherwise we can't do the overflow fill at the end of the function.
    {
        let digit: &'static [u8; 10] = b"0123456789";
        let mut abs_num = num.abs();

        // Write the digits into the buffer from right to left
        let mut buf_iter = if num < 0 && fill != ' ' {
            // If we're not using a space fill, then the sign will go in the
            // very first column.
            buf[1..].iter_mut().rev()
        } else {
            buf.iter_mut().rev()
        };
        if abs_num == 0 {
            if let Some(ch) = buf_iter.next() {
                *ch = b'0';
            }
        } else {
            while abs_num > 0 {
                if let Some(ch) = buf_iter.next() {
                    *ch = digit[(abs_num % 10) as usize];
                    abs_num /= 10;
                } else {
                    return false;
                }
            }
        }
        if fill == ' ' {
            if num < 0 {
                // When using a space fill, the sign goes next
                // to the last digit
                if let Some(ch) = buf_iter.next() {
                    *ch = b'-';
                } else {
                    return false;
                }
            }
            // Space fill
            while let Some(ch) = buf_iter.next() {
                *ch = b' ';
            }
        } else {
            // non-space fill on the left.
            while let Some(ch) = buf_iter.next() {
                *ch = fill as u8;
            }
        }
    }
    if num < 0 && fill != ' ' {
        buf[0] = b'-';
    }
    true
}

pub fn format_float_into(buf: &mut [u8], num: f64, digits_after_decimal: u32) -> bool {
    let buf_len = buf.len();

    let mut factor = 1;
    for _ in 0..digits_after_decimal {
        factor *= 10;
    }
    let mut num = num * factor as f64;
    if num < 0.0 {
        num -= 0.5;
    } else {
        num += 0.5
    }
    let num_as_int: i32 = num as i32;

    let num_before_decimal = num_as_int / factor;
    let num_after_decimal = num_as_int.abs() % factor;

    let digits_after_decimal = min(buf_len - 2, digits_after_decimal as usize);
    let digits_before_decimal = buf_len - digits_after_decimal - 1; // -1 for decimal point

    let mut ok = true;
    if num_as_int < 0 && num_as_int > -factor {
        // numbers between 0 and -1 need to be treated specially since we need
        // to have num_before_decimal be -0
        ok &= format_int_into(&mut buf[0..digits_before_decimal], 0, ' ');
        buf[digits_before_decimal - 2] = b'-';
    } else {
        ok &= format_int_into(&mut buf[0..digits_before_decimal], num_before_decimal, ' ');
    }
    if ok {
        buf[digits_before_decimal] = b'.';
        ok &= format_int_into(&mut buf[digits_before_decimal + 1..buf_len],
                              num_after_decimal,
                              '0');
    }
    ok
}
