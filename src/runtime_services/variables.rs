// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Variable Services

use crate::ffi::*;
use crate::runtime_services::RuntimeServices;

// Variable attributes
pub const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;
pub const EFI_VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002;
pub const EFI_VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004;
pub const EFI_VARIABLE_HARDWARE_ERROR_RECORD: u32 = 0x00000008;
pub const EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000010;
pub const EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: u32 = 0x00000020;
pub const EFI_VARIABLE_APPEND_WRITE: u32 = 0x00000040;

/// Variable wrapper for safe access
pub struct Variable<'a> {
    rt: &'a RuntimeServices,
}

impl<'a> Variable<'a> {
    /// Create a new variable wrapper
    pub fn new(rt: &'a RuntimeServices) -> Self {
        Variable { rt }
    }

    /// Get a variable
    pub unsafe fn get(
        &self,
        variable_name: *const Char16,
        vendor_guid: &Guid,
        data: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        let mut attributes = 0u32;
        let mut data_size = data.len();

        let status = (self.rt.get_variable)(
            variable_name,
            vendor_guid as *const _,
            &mut attributes,
            &mut data_size,
            data.as_mut_ptr() as *mut core::ffi::c_void,
        );

        if status == EFI_SUCCESS {
            Ok((attributes, data_size))
        } else {
            Err(status)
        }
    }

    /// Get next variable name
    pub unsafe fn get_next_variable_name(
        &self,
        variable_name: *mut Char16,
        name_size: &mut usize,
        vendor_guid: &mut Guid,
    ) -> Status {
        (self.rt.get_next_variable_name)(
            name_size as *mut usize,
            variable_name,
            vendor_guid as *mut _,
        )
    }

    /// Set a variable
    pub unsafe fn set(
        &self,
        variable_name: *const Char16,
        vendor_guid: &Guid,
        attributes: u32,
        data: &[u8],
    ) -> Status {
        (self.rt.set_variable)(
            variable_name,
            vendor_guid as *const _,
            attributes,
            data.len(),
            data.as_ptr() as *const core::ffi::c_void,
        )
    }

    /// Delete a variable (set with size 0)
    pub unsafe fn delete(&self, variable_name: *const Char16, vendor_guid: &Guid) -> Status {
        (self.rt.set_variable)(
            variable_name,
            vendor_guid as *const _,
            0,
            0,
            core::ptr::null(),
        )
    }

    /// Query variable info
    pub unsafe fn query_variable_info(&self, attributes: u32) -> Result<(u64, u64, u64), Status> {
        let mut max_storage = 0u64;
        let mut remaining_storage = 0u64;
        let mut max_variable_size = 0u64;

        let status = (self.rt.query_variable_info)(
            attributes,
            &mut max_storage,
            &mut remaining_storage,
            &mut max_variable_size,
        );

        if status == EFI_SUCCESS {
            Ok((max_storage, remaining_storage, max_variable_size))
        } else {
            Err(status)
        }
    }
}

// Well-known variable GUIDs
pub const EFI_GLOBAL_VARIABLE_GUID: Guid = Guid::new(
    0x8BE4DF61,
    0x93CA,
    0x11d2,
    [0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03, 0x2B, 0x8C],
);

pub const EFI_IMAGE_SECURITY_DATABASE_GUID: Guid = Guid::new(
    0xd719b2cb,
    0x3d3a,
    0x4596,
    [0xa3, 0xbc, 0xda, 0xd0, 0x0e, 0x67, 0x65, 0x6f],
);
