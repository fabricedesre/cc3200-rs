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

pub fn format_int_into(buf: &mut [u8], num: i32, fill: char) {
    let mut overflow = false;
    if buf.len() < 1 {
        return;
    }

    // Introduce a block so that we limit the lifetime of the borrow (of buf)
    // otherwise we can't do the overflow fill at the end of the function.
    {
        let digit: &'static [u8; 10] = b"0123456789";
        let mut abs_num = num.abs();

        // Write the digits into the buffer from right to left
        let mut buf_iter = buf.iter_mut().rev();
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
                    overflow = true;
                    break;
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
                    overflow = true;
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
        // Special case for non-space fill and negative numbers. We'll
        // overwrite the leftmost fill character with the sign.
        if buf[0] == (fill as u8) {
            buf[0] = b'-';
        } else {
            overflow = true;
        }
    }
    if overflow {
        // If we overflowed then fill the entire field with stars.
        fill_buf(buf, b'*');
    }
}

pub fn format_float_into(buf: &mut [u8], num: f64, digits_after_decimal: u32) {
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

    if num_as_int < 0 && num_as_int > -factor {
        // numbers between 0 and -1 need to be treated specially since we need
        // to have num_before_decimal be -0
        format_int_into(&mut buf[0..digits_before_decimal], 0, ' ');
        buf[digits_before_decimal - 2] = b'-';
    } else {
        format_int_into(&mut buf[0..digits_before_decimal], num_before_decimal, ' ');
    }
    if buf[0] == b'*' {
        // Integer portion overflowed. Overflow the whole thing.
        fill_buf(buf, b'*');
    } else {
        buf[digits_before_decimal] = b'.';
        format_int_into(&mut buf[digits_before_decimal + 1..buf_len],
                        num_after_decimal,
                        '0');
    }
}

#[cfg(test)]
mod test {

    use ::*;

    fn format_int_into_ref(buf: &mut [u8], num: i32, fill: char) {

        let s = if fill == '0' {
            format!("{:01$}", num, buf.len())
        } else {
            format!("{:1$}", num, buf.len())
        };
        let s_len = s.len();
        let buf_len = buf.len();

        // The width parameter is a minimum, but our buffer is constrained.
        // So we copy the rightmost buf_len characters.
        if s_len > buf_len {
            // Number didn't fit
            fill_buf(buf, b'*');
        } else {
            buf.copy_from_slice(&(s.into_bytes())[0..buf_len]);
        }
    }

    #[test]
    fn test_int() {
        let test_nums = vec![123456, 12345, 1234, 123, 12, 1, 0, -1, -12, -123, -1234, -12345,
                             -123456];

        let mut int_buf: [u8; 5] = [0; 5];
        let mut ref_buf: [u8; 5] = [0; 5];

        for num in test_nums.iter() {
            format_int_into(&mut int_buf[..], *num, ' ');
            format_int_into_ref(&mut ref_buf[..], *num, ' ');

            assert_eq!(int_buf, ref_buf);
        }

        for num in test_nums.iter() {
            format_int_into(&mut int_buf[..], *num, '0');
            format_int_into_ref(&mut ref_buf[..], *num, '0');

            assert_eq!(int_buf, ref_buf);
        }
    }

    fn format_float_into_ref(buf: &mut [u8], num: f64, digits_after_decimal: u32) {
        let buf_len = buf.len();
        let s = format!("{0:1$.2$}", num, buf_len, digits_after_decimal as usize);
        let s_len = s.len();

        if s_len > buf_len {
            // Number didn't fit
            fill_buf(buf, b'*');
        } else {
            buf.copy_from_slice(&(s.into_bytes())[0..buf_len]);
        }
    }

    #[test]
    fn test_float() {
        let test_floats = vec![123456.7890,
                               12345.6789,
                               1234.5678,
                               123.4567,
                               12.3456,
                               1.2345,
                               1.0000,
                               0.9999,
                               0.1000,
                               0.0000,
                               -0.1000,
                               -0.9999,
                               -1.0000,
                               -1.2345,
                               -12.3456,
                               -123.4567,
                               -1234.5678,
                               -12345.6789];

        let mut int_buf: [u8; 8] = [0; 8];
        let mut ref_buf: [u8; 8] = [0; 8];

        for num in test_floats.iter() {
            format_float_into(&mut int_buf[..], *num, 2);
            format_float_into_ref(&mut ref_buf[..], *num, 2);

            assert_eq!(int_buf, ref_buf);
        }
    }
}
