// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Safe wrappers around Runtime Services

use crate::ffi::*;
use crate::runtime_services::{ResetType, RuntimeServices, Time};

/// Result type for UEFI operations
pub type Result<T> = core::result::Result<T, Status>;

/// Safe wrapper for Runtime Services
pub struct RuntimeServicesWrapper<'a> {
    rs: &'a RuntimeServices,
}

impl<'a> RuntimeServicesWrapper<'a> {
    /// Create a new wrapper from a reference to Runtime Services
    pub fn new(rs: &'a RuntimeServices) -> Self {
        Self { rs }
    }

    /// Get the current time
    pub fn get_time(&self) -> Result<Time> {
        let mut time = Time {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            pad1: 0,
            nanosecond: 0,
            time_zone: 0,
            daylight: 0,
            pad2: 0,
        };

        let status = unsafe { (self.rs.get_time)(&mut time, core::ptr::null_mut()) };

        if status == EFI_SUCCESS {
            Ok(time)
        } else {
            Err(status)
        }
    }

    /// Set the current time
    pub fn set_time(&self, time: &Time) -> Result<()> {
        let status = unsafe { (self.rs.set_time)(time) };

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reset the system
    pub fn reset_system(&self, reset_type: ResetType, reset_status: Status, data: &[u8]) -> ! {
        unsafe {
            (self.rs.reset_system)(
                reset_type,
                reset_status,
                data.len(),
                data.as_ptr() as *const core::ffi::c_void,
            )
        }
    }
}
