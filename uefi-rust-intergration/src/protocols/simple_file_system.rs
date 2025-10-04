// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Simple File System Protocol

use crate::ffi::*;

/// EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID
pub const SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: Guid = Guid::new(
    0x964e5b22,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_FILE_PROTOCOL_GUID (Revision 2)
pub const FILE_PROTOCOL_GUID: Guid = Guid::new(
    0x09576e93,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_REVISION: Uint64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION: Uint64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: Uint64 = 0x00020000;
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: Uint64 = EFI_FILE_PROTOCOL_REVISION2;

/// EFI_SIMPLE_FILE_SYSTEM_PROTOCOL
#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: Uint64,
    pub open_volume: unsafe extern "efiapi" fn(
        this: *mut SimpleFileSystemProtocol,
        root: *mut *mut FileProtocol,
    ) -> Status,
}

/// EFI_FILE_PROTOCOL
#[repr(C)]
pub struct FileProtocol {
    pub revision: Uint64,
    pub open: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        new_handle: *mut *mut FileProtocol,
        file_name: *const Char16,
        open_mode: Uint64,
        attributes: Uint64,
    ) -> Status,
    pub close: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
    ) -> Status,
    pub delete: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
    ) -> Status,
    pub read: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        buffer_size: *mut Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        buffer_size: *mut Uintn,
        buffer: *const core::ffi::c_void,
    ) -> Status,
    pub get_position: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        position: *mut Uint64,
    ) -> Status,
    pub set_position: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        position: Uint64,
    ) -> Status,
    pub get_info: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        information_type: *const Guid,
        buffer_size: *mut Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub set_info: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
        information_type: *const Guid,
        buffer_size: Uintn,
        buffer: *const core::ffi::c_void,
    ) -> Status,
    pub flush: unsafe extern "efiapi" fn(
        this: *mut FileProtocol,
    ) -> Status,
}

// File open modes
pub const EFI_FILE_MODE_READ: u64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: u64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: u64 = 0x8000000000000000;

// File attributes
pub const EFI_FILE_READ_ONLY: u64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: u64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: u64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: u64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: u64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: u64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: u64 = 0x0000000000000037;

/// EFI_FILE_INFO_GUID
pub const FILE_INFO_GUID: Guid = Guid::new(
    0x09576e92,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_FILE_INFO
#[repr(C)]
pub struct FileInfo {
    pub size: Uint64,
    pub file_size: Uint64,
    pub physical_size: Uint64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: Uint64,
    // Followed by FileName[variable length]
}

/// EFI_TIME
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub year: Uint16,
    pub month: Uint8,
    pub day: Uint8,
    pub hour: Uint8,
    pub minute: Uint8,
    pub second: Uint8,
    pub pad1: Uint8,
    pub nanosecond: Uint32,
    pub time_zone: Int16,
    pub daylight: Uint8,
    pub pad2: Uint8,
}

impl SimpleFileSystemProtocol {
    /// Open the root directory
    pub unsafe fn open_volume(&mut self) -> Result<*mut FileProtocol, Status> {
        let mut root = core::ptr::null_mut();
        let status = (self.open_volume)(self, &mut root);
        if status == EFI_SUCCESS {
            Ok(root)
        } else {
            Err(status)
        }
    }
}

impl FileProtocol {
    /// Open a file
    pub unsafe fn open(
        &mut self,
        file_name: *const Char16,
        open_mode: u64,
        attributes: u64,
    ) -> Result<*mut FileProtocol, Status> {
        let mut new_handle = core::ptr::null_mut();
        let status = (self.open)(self, &mut new_handle, file_name, open_mode, attributes);
        if status == EFI_SUCCESS {
            Ok(new_handle)
        } else {
            Err(status)
        }
    }

    /// Close the file
    pub unsafe fn close(&mut self) -> Status {
        (self.close)(self)
    }

    /// Delete the file
    pub unsafe fn delete(&mut self) -> Status {
        (self.delete)(self)
    }

    /// Read from file
    pub unsafe fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Status> {
        let mut buffer_size = buffer.len();
        let status = (self.read)(self, &mut buffer_size, buffer.as_mut_ptr() as *mut _);
        if status == EFI_SUCCESS {
            Ok(buffer_size)
        } else {
            Err(status)
        }
    }

    /// Write to file
    pub unsafe fn write(&mut self, buffer: &[u8]) -> Result<usize, Status> {
        let mut buffer_size = buffer.len();
        let status = (self.write)(self, &mut buffer_size, buffer.as_ptr() as *const _);
        if status == EFI_SUCCESS {
            Ok(buffer_size)
        } else {
            Err(status)
        }
    }

    /// Get current position
    pub unsafe fn get_position(&mut self) -> Result<u64, Status> {
        let mut position = 0;
        let status = (self.get_position)(self, &mut position);
        if status == EFI_SUCCESS {
            Ok(position)
        } else {
            Err(status)
        }
    }

    /// Set position
    pub unsafe fn set_position(&mut self, position: u64) -> Status {
        (self.set_position)(self, position)
    }

    /// Get file information
    pub unsafe fn get_info(
        &mut self,
        information_type: &Guid,
        buffer_size: &mut usize,
        buffer: *mut core::ffi::c_void,
    ) -> Status {
        (self.get_info)(self, information_type as *const _, buffer_size, buffer)
    }

    /// Set file information
    pub unsafe fn set_info(
        &mut self,
        information_type: &Guid,
        buffer_size: usize,
        buffer: *const core::ffi::c_void,
    ) -> Status {
        (self.set_info)(self, information_type as *const _, buffer_size, buffer)
    }

    /// Flush file
    pub unsafe fn flush(&mut self) -> Status {
        (self.flush)(self)
    }
}
