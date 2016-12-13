use core;

#[derive(Debug)]
pub struct ParseFloatError {
    kind: FloatErrorKind,
}

#[derive(Debug)]
enum FloatErrorKind {
    Empty,
    Invalid,
}

fn pfe_empty() -> ParseFloatError {
    ParseFloatError { kind: FloatErrorKind::Empty }
}

fn pfe_invalid() -> ParseFloatError {
    ParseFloatError { kind: FloatErrorKind::Invalid }
}

pub fn parse_f64(string: &str) -> Result<f64, ParseFloatError> {
    let mut string_it = string.chars();
    let mut maybe_ch = string_it.next();
    let mut num: f64 = 0.0;
    let mut neg_found = false;

    match maybe_ch {
        Some('-') => {
            neg_found = true;
            maybe_ch = string_it.next();
        }
        Some('+') => {
            maybe_ch = string_it.next();
        }
        None => {
            return Err(pfe_empty());
        }
        _ => {}
    }

    // Process digits before the decimal point.
    while let Some(ch) = maybe_ch {
        match ch {
            '0'...'9' => {
                num *= 10.0;
                num += ((ch as u8) - ('0' as u8)) as f64;
            }
            '.' | 'e' | 'E' => {
                break;
            }
            _ => {
                return Err(pfe_invalid());
            }
        }
        maybe_ch = string_it.next();
    }

    // Process digits after the decimal point
    if maybe_ch == Some('.') {
        let mut mul = 1.0;
        maybe_ch = string_it.next();    // get first character after '.'
        while let Some(ch) = maybe_ch {
            match ch {
                '0'...'9' => {
                    mul *= 0.1;
                    num += (((ch as u8) - ('0' as u8)) as f64) * mul;
                }
                'e' | 'E' => {
                    break;
                }
                _ => {
                    return Err(pfe_invalid());
                }
            }
            maybe_ch = string_it.next();
        }
    }

    // Process exponent
    if maybe_ch == Some('e') || maybe_ch == Some('E') {
        let mut neg_exp_found = false;
        maybe_ch = string_it.next();    // Get character after 'e'
        match maybe_ch {
            Some('-') => {
                neg_exp_found = true;
                maybe_ch = string_it.next();
            }
            Some('+') => {
                maybe_ch = string_it.next();
            }
            _ => {}
        }
        let mut exp = 0;
        while let Some(ch) = maybe_ch {
            match ch {
                '0'...'9' => {
                    exp *= 10;
                    exp += ((ch as u8) - ('0' as u8)) as i32;
                }
                _ => {
                    return Err(pfe_invalid());
                }
            }
            maybe_ch = string_it.next();
        }
        if neg_exp_found {
            exp = -exp;
        }
        num *= core::num::Float::powi(10.0_f64, exp);
    }

    if maybe_ch.is_none() {
        if neg_found {
            num = -num;
        }
        Ok(num)
    } else {
        Err(pfe_invalid())
    }
}
