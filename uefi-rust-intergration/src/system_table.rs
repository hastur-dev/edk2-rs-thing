// SPDX-License-Identifier: BSD-2-Clause-Patent
//! EFI System Table

use crate::ffi::*;
use crate::boot_services::BootServices;
use crate::runtime_services::RuntimeServices;

pub use crate::tables::configuration::ConfigurationTable;

/// Simple Text Output Protocol (minimal definition)
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: *mut SimpleTextOutputProtocol, extended_verification: Boolean) -> Status,
    pub output_string: unsafe extern "efiapi" fn(this: *mut SimpleTextOutputProtocol, string: *const Char16) -> Status,
    // Additional fields omitted for brevity
}

/// Simple Text Input Protocol (minimal definition)
#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: *mut SimpleTextInputProtocol, extended_verification: Boolean) -> Status,
    // Additional fields omitted for brevity
}

/// EFI System Table
#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const Char16,
    pub firmware_revision: Uint32,
    pub console_in_handle: *mut Handle,
    pub con_in: *mut SimpleTextInputProtocol,
    pub console_out_handle: *mut Handle,
    pub con_out: *mut SimpleTextOutputProtocol,
    pub standard_error_handle: *mut Handle,
    pub std_err: *mut SimpleTextOutputProtocol,
    pub runtime_services: *mut RuntimeServices,
    pub boot_services: *mut BootServices,
    pub number_of_table_entries: Uintn,
    pub configuration_table: *mut ConfigurationTable,
}

impl SystemTable {
    /// Get a reference to boot services
    pub unsafe fn boot_services(&self) -> &BootServices {
        &*self.boot_services
    }

    /// Get a reference to runtime services
    pub unsafe fn runtime_services(&self) -> &RuntimeServices {
        &*self.runtime_services
    }

    /// Get console output protocol
    pub unsafe fn stdout(&self) -> &mut SimpleTextOutputProtocol {
        &mut *self.con_out
    }
}

// System Table signature
pub const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;
