// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use collections::string::String;
use format::format_int_into;

pub type Seconds = i64;

// LEAPEPOCH corresponds to 2000-03-01, which is a mod-400 year, immediately
// after Feb 29. We calculate seconds as a signed integer relative to that.
//
// Our timebase is relative to 1970-01-01.
//
// 30 * 365 calculates the number of years from Jan 1, 1970 until Jan 1, 2000
// 7 corresponds to the 7 leap years between 1970 and 2000
// 31 is the number of days in January 2000
// 29 is the number of days in February 2000
// 86400 is the number of seconds in a day.

const LEAP_EPOCH: Seconds = (30 * 365 + 7 + 31 + 29) * 86400;   // Mar 1, 2000

const DAYS_PER_4Y: i32 = 365 * 4 + 1;               // Every 4th year is a leap year
const DAYS_PER_100Y: i32 = DAYS_PER_4Y * 25 - 1;    // Every 100th is not a leap year
const DAYS_PER_400Y: i32 = DAYS_PER_100Y * 4 + 1;   // Every 400th is a leap year

#[derive(Debug, Default)]
pub struct Tm {
    /// Seconds after the minute - [0, 60]
    pub tm_sec: i32,

    /// Minutes after the hour - [0, 59]
    pub tm_min: i32,

    /// Hours after midnight - [0, 23]
    pub tm_hour: i32,

    /// Day of the month - [1, 31]
    pub tm_mday: i32,

    /// Months since January - [0, 11]
    pub tm_mon: i32,

    /// Years since 1900
    pub tm_year: i32,

    /// Days since Sunday - [0, 6]. 0 = Sunday, 1 = Monday, ..., 6 = Saturday.
    pub tm_wday: i32,

    /// Days since January 1 - [0, 365]
    pub tm_yday: i32,

    /// Daylight Saving Time flag.
    ///
    /// This value is positive if Daylight Saving Time is in effect, zero if Daylight Saving Time
    /// is not in effect, and negative if this information is not available.
    pub tm_isdst: i32,

    /// Identifies the time zone that was used to compute this broken-down time value, including any
    /// adjustment for Daylight Saving Time. This is the number of seconds east of UTC. For example,
    /// for U.S. Pacific Daylight Time, the value is -7*60*60 = -25200.
    pub tm_utcoff: i32,

    /// Nanoseconds after the second - [0, 10<sup>9</sup> - 1]
    pub tm_nsec: i32,
}

impl Tm {
    pub fn new() -> Self {
        Tm { ..Default::default() }
    }

    pub fn gmtime(t: Seconds) -> Self {
        let mut tm = Tm::new();
        let mut seconds = t - LEAP_EPOCH;
        let mut days = (seconds / 86400) as i32;
        seconds %= 86400;
        if seconds < 0 {
            seconds += 86400;
            days -= 1;
        }
        tm.tm_hour = (seconds / 3600) as i32;
        tm.tm_min = (seconds / 60 % 60) as i32;
        tm.tm_sec = (seconds % 60) as i32;

        tm.tm_wday = ((days + 3) % 7) as i32;  // Mar 1, 2000 was a Wednesday (3)
        if tm.tm_wday < 0 {
            tm.tm_wday += 7;
        }

        let mut qc_cycles = days / DAYS_PER_400Y;
        days %= DAYS_PER_400Y;
        if days < 0 {
            days += DAYS_PER_400Y;
            qc_cycles -= 1;
        }

        let mut c_cycles = days / DAYS_PER_100Y;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        days -= c_cycles * DAYS_PER_100Y;

        let mut q_cycles = days / DAYS_PER_4Y;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        days -= q_cycles * DAYS_PER_4Y;

        let mut years = days / 365;
        if years == 4 {
            years -= 1;
        }
        days -= years * 365;

        let leap = ((years == 0) && ((q_cycles != 0) || (c_cycles == 0))) as i32;
        tm.tm_yday = days + 31 + 28 + leap;
        if tm.tm_yday >= (365 + leap) {
            tm.tm_yday -= 365 + leap;
        }

        // tm_year is the year minus 1900. So 100 corresponds to 2000.
        tm.tm_year = 100 + years + (4 * q_cycles) + (100 * c_cycles) + (400 * qc_cycles);

        // Note: days_in_month[0] corresponds to March
        const DAYS_IN_MONTH: [u8; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
        let mut month = 0;
        while (DAYS_IN_MONTH[month] as i32) <= days {
            days -= DAYS_IN_MONTH[month] as i32;
            month += 1;
        }

        tm.tm_mon = month as i32 + 2;
        if tm.tm_mon >= 12 {
            tm.tm_mon -= 12;
            tm.tm_year += 1;
        }

        tm.tm_mday = days + 1;  // Make 1 based

        tm
    }

    pub fn ctime(&self) -> String {
        const WEEK_DAY: [&'static str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        const MONTH: [&'static str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug",
                                           "Sep", "Oct", "Nov", "Dec"];

        format!("{} {} {:2} {:02}:{:02}:{:02} xxx {:04}",
                WEEK_DAY[self.tm_wday as usize],
                MONTH[self.tm_mon as usize],
                self.tm_mday,
                self.tm_hour,
                self.tm_min,
                self.tm_sec,
                self.tm_year + 1900)
    }

    // Outputs the time represented by the tm buffer in an ISO compatible format.
    // In particular, this will use the format YYYY-MM-DDTHH:MM:SS.sssZ

    pub fn format_iso_into(&self, buf: &mut [u8]) -> bool {
        if buf.len() < 24 {
            return false;
        }

        //                    012345678901234567890123
        buf.copy_from_slice(b"YYYY-MM-DDTHH:MM:SS.000Z");

        format_int_into(&mut buf[0..4], self.tm_year + 1900, '0');
        format_int_into(&mut buf[5..7], self.tm_mon + 1, '0');
        format_int_into(&mut buf[8..10], self.tm_mday, '0');
        format_int_into(&mut buf[11..13], self.tm_hour, '0');
        format_int_into(&mut buf[14..16], self.tm_min, '0');
        format_int_into(&mut buf[17..19], self.tm_sec, '0');

        true
    }
}

#[cfg(test)]
mod tests {

    use ::{Seconds, Tm};

    fn spot_test(time: Seconds, tup: (i32, i32, i32, i32, i32, i32, i32, i32)) {
        println!("Testing time = {}", time);
        let tm = Tm::gmtime(time);
        println!("{:?}", tm);
        let (year, month, day, hour, minute, second, wday, yday) = tup;

        assert_eq!(tm.tm_year + 1900, year);
        assert_eq!(tm.tm_mon + 1, month);
        assert_eq!(tm.tm_mday, day);
        assert_eq!(tm.tm_hour, hour);
        assert_eq!(tm.tm_min, minute);
        assert_eq!(tm.tm_sec, second);
        assert_eq!(tm.tm_wday, wday);
        assert_eq!(tm.tm_yday + 1, yday);
    }


    #[test]
    fn spot_tests() {
        let tests = vec![
            (          0,  (1970,  1,  1,  0,  0,  0, 4,   1)),
            (         -1,  (1969, 12, 31, 23, 59, 59, 3, 365)),
            (          1,  (1970,  1,  1,  0,  0,  1, 4,   1)),
            (         59,  (1970,  1,  1,  0,  0, 59, 4,   1)),
            (         60,  (1970,  1,  1,  0,  1,  0, 4,   1)),
            (       3599,  (1970,  1,  1,  0, 59, 59, 4,   1)),
            (       3600,  (1970,  1,  1,  1,  0,  0, 4,   1)),
            (  447549467,  (1984,  3,  7, 23, 17, 47, 3,  67)),
            ( -940984933,  (1940,  3,  7, 23, 17, 47, 4,  67)),
            (-1073001599,  (1936,  1,  1,  0,  0,  1, 3,   1)),
            (-1073001600,  (1936,  1,  1,  0,  0,  0, 3,   1)),
            (-1073001601,  (1935, 12, 31, 23, 59, 59, 2, 365)),
        ];

        for test in tests {
            spot_test(test.0, test.1);
        }
    }

    #[test]
    fn more_exhaustive_test() {
        use std::str;

        let mut days_in_month: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut seconds: Seconds = 0;
        let mut wday = 4; // Jan 1, 1970 was a Thursday (4)
        for year in 1970..2076 {
            println!("Testing {}", year);
            let mut yday = 0;
            if year % 4 == 0 {
                days_in_month[1] = 29;
            } else {
                days_in_month[1] = 28;
            }
            for month in 0..12 {
                for mday in 1..days_in_month[month] + 1 {
                    let tm = Tm::gmtime(seconds);

                    assert_eq!(year, tm.tm_year + 1900);
                    assert_eq!(month as i32, tm.tm_mon);
                    assert_eq!(mday, tm.tm_mday);
                    assert_eq!(0, tm.tm_hour);
                    assert_eq!(0, tm.tm_min);
                    assert_eq!(0, tm.tm_sec);
                    assert_eq!(wday, tm.tm_wday);
                    assert_eq!(yday, tm.tm_yday);

                    let mut buf: [u8; 24] = *b"yyyy-mm-ddthh:mm:ss.sssz";
                    tm.format_iso_into(&mut buf);

                    assert_eq!(str::from_utf8(&buf).unwrap(),
                               format!("{:4}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
                                       tm.tm_year + 1900,
                                       tm.tm_mon + 1,
                                       tm.tm_mday,
                                       tm.tm_hour,
                                       tm.tm_min,
                                       tm.tm_sec));

                    seconds += 86400;
                    yday += 1;
                    wday = (wday + 1) % 7;
                }
            }
        }
    }
}
