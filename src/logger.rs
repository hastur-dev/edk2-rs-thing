// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Logging Framework

use crate::ffi::*;
use crate::protocols::SimpleTextOutputProtocol;
use core::fmt::{self, Write};

/// Log level
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

static mut LOGGER: Option<Logger> = None;

/// UEFI Logger
pub struct Logger {
    console: Option<*mut SimpleTextOutputProtocol>,
    level: LogLevel,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    /// Create a new logger
    pub const fn new() -> Self {
        Logger {
            console: None,
            level: LogLevel::Info,
        }
    }

    /// Initialize the logger with a console output protocol
    ///
    /// # Safety
    /// The console pointer must be valid for the lifetime of the logger
    pub unsafe fn init(console: *mut SimpleTextOutputProtocol, level: LogLevel) {
        LOGGER = Some(Logger {
            console: Some(console),
            level,
        });
    }

    /// Set log level
    pub fn set_level(level: LogLevel) {
        unsafe {
            if let Some(ref mut logger) = LOGGER {
                logger.level = level;
            }
        }
    }

    /// Get the global logger
    fn get() -> Option<&'static mut Logger> {
        unsafe { (&raw mut LOGGER).as_mut().and_then(|x| x.as_mut()) }
    }

    /// Log a message
    pub fn log(level: LogLevel, args: fmt::Arguments) {
        if let Some(logger) = Self::get() {
            if level <= logger.level {
                logger.write_log(level, args);
            }
        }
    }

    fn write_log(&mut self, level: LogLevel, args: fmt::Arguments) {
        if let Some(console) = self.console {
            let _ = write!(ConsoleWriter { console }, "[{}] ", level);
            let _ = ConsoleWriter { console }.write_fmt(args);
            let _ = write!(ConsoleWriter { console }, "\r\n");
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN "),
            LogLevel::Info => write!(f, "INFO "),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

struct ConsoleWriter {
    console: *mut SimpleTextOutputProtocol,
}

impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let ucs2: Vec<u16> = s.encode_utf16().chain(core::iter::once(0)).collect();
        unsafe {
            let status = ((*self.console).output_string)(self.console, ucs2.as_ptr());
            if status == EFI_SUCCESS {
                Ok(())
            } else {
                Err(fmt::Error)
            }
        }
    }
}

/// Log macros
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::logger::LogLevel::Error, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::logger::LogLevel::Warn, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::logger::LogLevel::Info, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::logger::LogLevel::Debug, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::logger::LogLevel::Trace, format_args!($($arg)*))
    };
}
