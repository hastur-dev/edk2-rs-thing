// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Time Services

use crate::ffi::*;
use crate::runtime_services::RuntimeServices;

/// EFI_TIME
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub year: Uint16,  // 1900 - 9999
    pub month: Uint8,  // 1 - 12
    pub day: Uint8,    // 1 - 31
    pub hour: Uint8,   // 0 - 23
    pub minute: Uint8, // 0 - 59
    pub second: Uint8, // 0 - 59
    pub pad1: Uint8,
    pub nanosecond: Uint32, // 0 - 999,999,999
    pub time_zone: Int16,   // -1440 to 1440 or 2047
    pub daylight: Uint8,
    pub pad2: Uint8,
}

/// EFI_TIME_CAPABILITIES
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeCapabilities {
    pub resolution: Uint32,
    pub accuracy: Uint32,
    pub sets_to_zero: Boolean,
}

// Time flags
pub const EFI_TIME_ADJUST_DAYLIGHT: u8 = 0x01;
pub const EFI_TIME_IN_DAYLIGHT: u8 = 0x02;
pub const EFI_UNSPECIFIED_TIMEZONE: i16 = 0x07FF;

impl Time {
    /// Create a new time
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Time {
            year,
            month,
            day,
            hour,
            minute,
            second,
            pad1: 0,
            nanosecond: 0,
            time_zone: EFI_UNSPECIFIED_TIMEZONE,
            daylight: 0,
            pad2: 0,
        }
    }

    /// Validate the time
    pub fn is_valid(&self) -> bool {
        if self.year < 1900 || self.year > 9999 {
            return false;
        }
        if self.month < 1 || self.month > 12 {
            return false;
        }
        if self.day < 1 || self.day > 31 {
            return false;
        }
        if self.hour > 23 {
            return false;
        }
        if self.minute > 59 {
            return false;
        }
        if self.second > 59 {
            return false;
        }
        if self.nanosecond > 999_999_999 {
            return false;
        }
        if self.time_zone != EFI_UNSPECIFIED_TIMEZONE
            && (self.time_zone < -1440 || self.time_zone > 1440)
        {
            return false;
        }
        true
    }
}

/// Time service wrapper
pub struct TimeService<'a> {
    rt: &'a RuntimeServices,
}

impl<'a> TimeService<'a> {
    /// Create a new time service wrapper
    pub fn new(rt: &'a RuntimeServices) -> Self {
        TimeService { rt }
    }

    /// Get the current time
    pub unsafe fn get_time(&self) -> Result<(Time, TimeCapabilities), Status> {
        let mut time = core::mem::zeroed();
        let mut capabilities = core::mem::zeroed();

        let status = (self.rt.get_time)(&mut time, &mut capabilities);

        if status == EFI_SUCCESS {
            Ok((time, capabilities))
        } else {
            Err(status)
        }
    }

    /// Set the current time
    pub unsafe fn set_time(&self, time: &Time) -> Status {
        (self.rt.set_time)(time as *const _)
    }

    /// Get wakeup time
    pub unsafe fn get_wakeup_time(&self) -> Result<(bool, bool, Time), Status> {
        let mut enabled = 0u8;
        let mut pending = 0u8;
        let mut time = core::mem::zeroed();

        let status = (self.rt.get_wakeup_time)(&mut enabled, &mut pending, &mut time);

        if status == EFI_SUCCESS {
            Ok((enabled != 0, pending != 0, time))
        } else {
            Err(status)
        }
    }

    /// Set wakeup time
    pub unsafe fn set_wakeup_time(&self, enable: bool, time: Option<&Time>) -> Status {
        let time_ptr = time.map_or(core::ptr::null(), |t| t as *const _);
        (self.rt.set_wakeup_time)(enable as Boolean, time_ptr)
    }
}
