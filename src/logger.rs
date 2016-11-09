// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// A simple logger that sets the max log level to Trace in debug builds and to Info in release ones.

use log::{self, LogRecord, LogLevelFilter, LogMetadata, SetLoggerError};

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        // Ignore logging errors. It's not worth killing the program because of
        // failed debug output. It would be nicer to save the error and report
        // it later, however.
        use core::fmt::Write;
        let mut console = $crate::cc3200::Console {};
        let _ = write!(console, $($args)*);
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr)               => ( print!(concat!($fmt, '\n')) );
    ($fmt:expr, $($args:tt)*) => ( print!(concat!($fmt, '\n'), $($args)*) );
}

pub struct SimpleLogger;

#[cfg(debug_assertions)]
static MAX_LOG_LEVEL: LogLevelFilter = LogLevelFilter::Trace;

#[cfg(not(debug_assertions))]
static MAX_LOG_LEVEL: LogLevelFilter = LogLevelFilter::Info;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        return metadata.level() <= MAX_LOG_LEVEL;
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{:5} [{}@{}] {}",
                     record.level(),
                     record.target(),
                     record.location().line(),
                     record.args());
        }
    }
}

impl SimpleLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        unsafe {
            log::set_logger_raw(|max_log_level| {
                max_log_level.set(MAX_LOG_LEVEL);
                &SimpleLogger
            })
        }
    }
}
