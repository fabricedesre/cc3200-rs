extern crate numeric_utils;

use numeric_utils::{format_float_into, format_int_into};

fn format_int_into_ref(buf: &mut [u8], num: i32, fill: char) -> bool {

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
        false
    } else {
        buf.copy_from_slice(&(s.into_bytes())[0..buf_len]);
        true
    }
}

#[test]
fn test_int() {
    let test_nums = vec![123456, 12345, 1234, 123, 12, 1, 0, -1, -12, -123, -1234, -12345,
                         -123456];

    let mut int_buf: [u8; 5] = [0; 5];
    let mut ref_buf: [u8; 5] = [0; 5];

    for num in test_nums.iter() {
        let ok1 = format_int_into(&mut int_buf[..], *num, ' ');
        let ok2 = format_int_into_ref(&mut ref_buf[..], *num, ' ');

        assert_eq!(ok1, ok2);
        if ok1 {
            assert_eq!(int_buf, ref_buf);
        }
    }

    for num in test_nums.iter() {
        let ok1 = format_int_into(&mut int_buf[..], *num, '0');
        let ok2 = format_int_into_ref(&mut ref_buf[..], *num, '0');

        assert_eq!(ok1, ok2);
        if ok1 {
            assert_eq!(int_buf, ref_buf);
        }
    }
}

fn format_float_into_ref(buf: &mut [u8], num: f64, digits_after_decimal: u32) -> bool {
    let buf_len = buf.len();
    let s = format!("{0:1$.2$}", num, buf_len, digits_after_decimal as usize);
    let s_len = s.len();

    if s_len > buf_len {
        // Number didn't fit
        false
    } else {
        buf.copy_from_slice(&(s.into_bytes())[0..buf_len]);
        true
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

    let mut flt_buf: [u8; 8] = [0; 8];
    let mut ref_buf: [u8; 8] = [0; 8];

    for num in test_floats.iter() {
        let ok1 = format_float_into(&mut flt_buf[..], *num, 2);
        let ok2 = format_float_into_ref(&mut ref_buf[..], *num, 2);

        assert_eq!(ok1, ok2);
        if ok1 {
            assert_eq!(flt_buf, ref_buf);
        }
    }
}


