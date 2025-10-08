// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Loaded Image Protocol

use crate::ffi::*;
use crate::protocols::DevicePathProtocol;
use crate::system_table::SystemTable;

/// EFI_LOADED_IMAGE_PROTOCOL_GUID
pub const LOADED_IMAGE_PROTOCOL_GUID: Guid = Guid::new(
    0x5B1B31A1,
    0x9562,
    0x11d2,
    [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

pub const EFI_LOADED_IMAGE_PROTOCOL_REVISION: Uint32 = 0x1000;

/// EFI_LOADED_IMAGE_PROTOCOL
#[repr(C)]
pub struct LoadedImageProtocol {
    pub revision: Uint32,
    pub parent_handle: *mut Handle,
    pub system_table: *mut SystemTable,
    pub device_handle: *mut Handle,
    pub file_path: *mut DevicePathProtocol,
    pub reserved: *mut core::ffi::c_void,
    pub load_options_size: Uint32,
    pub load_options: *mut core::ffi::c_void,
    pub image_base: *mut core::ffi::c_void,
    pub image_size: Uint64,
    pub image_code_type: MemoryType,
    pub image_data_type: MemoryType,
    pub unload: unsafe extern "efiapi" fn(image_handle: *mut Handle) -> Status,
}

impl LoadedImageProtocol {
    /// Get parent handle
    pub fn parent_handle(&self) -> Option<&Handle> {
        unsafe {
            if self.parent_handle.is_null() {
                None
            } else {
                Some(&*self.parent_handle)
            }
        }
    }

    /// Get system table
    pub fn system_table(&self) -> Option<&SystemTable> {
        unsafe {
            if self.system_table.is_null() {
                None
            } else {
                Some(&*self.system_table)
            }
        }
    }

    /// Get device handle
    pub fn device_handle(&self) -> Option<&Handle> {
        unsafe {
            if self.device_handle.is_null() {
                None
            } else {
                Some(&*self.device_handle)
            }
        }
    }

    /// Get file path
    pub fn file_path(&self) -> Option<&DevicePathProtocol> {
        unsafe {
            if self.file_path.is_null() {
                None
            } else {
                Some(&*self.file_path)
            }
        }
    }

    /// Get load options
    pub fn load_options(&self) -> Option<&[u8]> {
        unsafe {
            if self.load_options.is_null() || self.load_options_size == 0 {
                None
            } else {
                Some(core::slice::from_raw_parts(
                    self.load_options as *const u8,
                    self.load_options_size as usize,
                ))
            }
        }
    }

    /// Get image base and size
    pub fn image_location(&self) -> (*mut core::ffi::c_void, u64) {
        (self.image_base, self.image_size)
    }

    /// Unload the image
    pub unsafe fn unload(&self, image_handle: *mut Handle) -> Status {
        (self.unload)(image_handle)
    }
}
