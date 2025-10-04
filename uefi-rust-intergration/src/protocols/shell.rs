// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Shell Protocol
//!
//! This module provides bindings for the UEFI Shell protocols, allowing
//! applications to interact with the UEFI Shell environment.

use crate::ffi::*;
use crate::runtime_services::Time;

/// EFI_SHELL_PROTOCOL_GUID
pub const SHELL_PROTOCOL_GUID: Guid = Guid::new(
    0x6302d008,
    0x7f9b,
    0x4f30,
    [0x87, 0xac, 0x60, 0xc9, 0xfe, 0xf5, 0xda, 0x4e],
);

/// EFI_SHELL_PARAMETERS_PROTOCOL_GUID
pub const SHELL_PARAMETERS_PROTOCOL_GUID: Guid = Guid::new(
    0x752f3136,
    0x4e16,
    0x4fdc,
    [0xa2, 0x2a, 0xe5, 0xf4, 0x68, 0x12, 0xf4, 0xca],
);

/// Shell File Handle
pub type ShellFileHandle = *mut core::ffi::c_void;

/// Shell File Open Modes
pub const EFI_FILE_MODE_READ: Uint64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: Uint64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: Uint64 = 0x8000000000000000;

/// Shell File Attributes
pub const EFI_FILE_READ_ONLY: Uint64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: Uint64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: Uint64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: Uint64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: Uint64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: Uint64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: Uint64 = 0x0000000000000037;

/// Shell File Information
#[repr(C)]
pub struct ShellFileInfo {
    pub link: ListEntry,
    pub status: Status,
    pub full_name: *const Char16,
    pub file_name: *const Char16,
    pub handle: ShellFileHandle,
    pub info: *mut FileInfo,
}

/// File Information structure
#[repr(C)]
pub struct FileInfo {
    pub size: Uint64,
    pub file_size: Uint64,
    pub physical_size: Uint64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: Uint64,
    pub file_name: [Char16; 1], // Variable length
}

/// List Entry for linked lists
#[repr(C)]
pub struct ListEntry {
    pub forward_link: *mut ListEntry,
    pub back_link: *mut ListEntry,
}

/// Device Path Type
pub type DevicePath = *mut core::ffi::c_void;

/// Shell Execute Return Codes
pub type ShellStatus = Uintn;

pub const SHELL_SUCCESS: ShellStatus = 0;
pub const SHELL_LOAD_ERROR: ShellStatus = 1;
pub const SHELL_INVALID_PARAMETER: ShellStatus = 2;
pub const SHELL_UNSUPPORTED: ShellStatus = 3;
pub const SHELL_BAD_BUFFER_SIZE: ShellStatus = 4;
pub const SHELL_BUFFER_TOO_SMALL: ShellStatus = 5;
pub const SHELL_NOT_READY: ShellStatus = 6;
pub const SHELL_DEVICE_ERROR: ShellStatus = 7;
pub const SHELL_WRITE_PROTECTED: ShellStatus = 8;
pub const SHELL_OUT_OF_RESOURCES: ShellStatus = 9;
pub const SHELL_VOLUME_CORRUPTED: ShellStatus = 10;
pub const SHELL_VOLUME_FULL: ShellStatus = 11;
pub const SHELL_NO_MEDIA: ShellStatus = 12;
pub const SHELL_MEDIA_CHANGED: ShellStatus = 13;
pub const SHELL_NOT_FOUND: ShellStatus = 14;
pub const SHELL_ACCESS_DENIED: ShellStatus = 15;
pub const SHELL_TIMEOUT: ShellStatus = 16;
pub const SHELL_NOT_STARTED: ShellStatus = 17;
pub const SHELL_ALREADY_STARTED: ShellStatus = 18;
pub const SHELL_ABORTED: ShellStatus = 19;

/// EFI_SHELL_PROTOCOL
#[repr(C)]
pub struct ShellProtocol {
    pub execute: unsafe extern "efiapi" fn(
        parent_image_handle: *mut Handle,
        command_line: *const Char16,
        environment: *mut *const Char16,
        status_code: *mut Status,
    ) -> Status,
    pub get_env: unsafe extern "efiapi" fn(name: *const Char16) -> *const Char16,
    pub set_env: unsafe extern "efiapi" fn(
        name: *const Char16,
        value: *const Char16,
        volatile: Boolean,
    ) -> Status,
    pub get_alias: unsafe extern "efiapi" fn(
        alias: *const Char16,
        volatile: *mut Boolean,
    ) -> *const Char16,
    pub set_alias: unsafe extern "efiapi" fn(
        command: *const Char16,
        alias: *const Char16,
        replace: Boolean,
        volatile: Boolean,
    ) -> Status,
    pub get_help_text: unsafe extern "efiapi" fn(
        command: *const Char16,
        sections: *const Char16,
    ) -> *mut Char16,
    pub get_device_path_from_map: unsafe extern "efiapi" fn(mapping: *const Char16) -> DevicePath,
    pub get_map_from_device_path: unsafe extern "efiapi" fn(
        device_path: *mut DevicePath,
    ) -> *const Char16,
    pub get_device_path_from_file_path:
        unsafe extern "efiapi" fn(path: *const Char16) -> DevicePath,
    pub get_file_path_from_device_path:
        unsafe extern "efiapi" fn(device_path: DevicePath) -> *const Char16,
    pub set_map: unsafe extern "efiapi" fn(
        device_path: DevicePath,
        mapping: *const Char16,
    ) -> Status,
    pub get_cur_dir: unsafe extern "efiapi" fn(file_system_mapping: *const Char16) -> *const Char16,
    pub set_cur_dir: unsafe extern "efiapi" fn(
        file_system: *const Char16,
        dir: *const Char16,
    ) -> Status,
    pub open_file_list: unsafe extern "efiapi" fn(
        path: *const Char16,
        open_mode: Uint64,
        file_list: *mut *mut ShellFileInfo,
    ) -> Status,
    pub free_file_list: unsafe extern "efiapi" fn(file_list: *mut *mut ShellFileInfo) -> Status,
    pub remove_dup_in_file_list:
        unsafe extern "efiapi" fn(file_list: *mut *mut ShellFileInfo) -> Status,
    pub batch_is_active: unsafe extern "efiapi" fn() -> Boolean,
    pub is_root_shell: unsafe extern "efiapi" fn() -> Boolean,
    pub enable_page_break: unsafe extern "efiapi" fn(),
    pub disable_page_break: unsafe extern "efiapi" fn(),
    pub get_page_break: unsafe extern "efiapi" fn() -> Boolean,
    pub get_device_name: unsafe extern "efiapi" fn(
        device_handle: Handle,
        flags: Uint32,
        language: *const Char8,
        best_device_name: *mut *mut Char16,
    ) -> Status,
    pub get_file_info: unsafe extern "efiapi" fn(file_handle: ShellFileHandle) -> *mut FileInfo,
    pub set_file_info: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        file_info: *mut FileInfo,
    ) -> Status,
    pub open_file_by_name: unsafe extern "efiapi" fn(
        file_name: *const Char16,
        file_handle: *mut ShellFileHandle,
        open_mode: Uint64,
    ) -> Status,
    pub close_file: unsafe extern "efiapi" fn(file_handle: ShellFileHandle) -> Status,
    pub create_file: unsafe extern "efiapi" fn(
        file_name: *const Char16,
        file_attribs: Uint64,
        file_handle: *mut ShellFileHandle,
    ) -> Status,
    pub read_file: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        buffer_size: *mut Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write_file: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        buffer_size: *mut Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub delete_file: unsafe extern "efiapi" fn(file_handle: ShellFileHandle) -> Status,
    pub delete_file_by_name: unsafe extern "efiapi" fn(file_name: *const Char16) -> Status,
    pub get_file_position: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        position: *mut Uint64,
    ) -> Status,
    pub set_file_position: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        position: Uint64,
    ) -> Status,
    pub flush_file: unsafe extern "efiapi" fn(file_handle: ShellFileHandle) -> Status,
    pub find_files: unsafe extern "efiapi" fn(
        file_pattern: *const Char16,
        file_list: *mut *mut ShellFileInfo,
    ) -> Status,
    pub find_files_in_dir: unsafe extern "efiapi" fn(
        file_dir_handle: ShellFileHandle,
        file_list: *mut *mut ShellFileInfo,
    ) -> Status,
    pub get_file_size: unsafe extern "efiapi" fn(
        file_handle: ShellFileHandle,
        size: *mut Uint64,
    ) -> Status,
    pub open_root: unsafe extern "efiapi" fn(
        device_handle: Handle,
        file_handle: *mut ShellFileHandle,
    ) -> Status,
    pub open_root_by_handle: unsafe extern "efiapi" fn(
        device_handle: Handle,
        file_handle: *mut ShellFileHandle,
    ) -> Status,
    pub execute_in_shell: *mut core::ffi::c_void, // Reserved
    pub get_env_ex: unsafe extern "efiapi" fn(
        name: *const Char16,
        attributes: *mut Uint32,
    ) -> *const Char16,
}

/// EFI_SHELL_PARAMETERS_PROTOCOL
#[repr(C)]
pub struct ShellParametersProtocol {
    pub argv: *mut *mut Char16,
    pub argc: Uintn,
    pub std_in: ShellFileHandle,
    pub std_out: ShellFileHandle,
    pub std_err: ShellFileHandle,
}

impl ShellProtocol {
    /// Execute a shell command
    pub unsafe fn execute(
        &mut self,
        parent_handle: Handle,
        command: &[u16],
    ) -> Result<Status, Status> {
        let mut status_code = EFI_SUCCESS;
        let handle_ptr = parent_handle.as_ptr() as *mut Handle;
        let result = (self.execute)(
            handle_ptr,
            command.as_ptr() as *const _,
            core::ptr::null_mut(),
            &mut status_code,
        );

        if result == EFI_SUCCESS {
            Ok(status_code)
        } else {
            Err(result)
        }
    }

    /// Get environment variable
    pub unsafe fn get_env(&mut self, name: &[u16]) -> Option<*const Char16> {
        let result = (self.get_env)(name.as_ptr() as *const _);
        if result.is_null() {
            None
        } else {
            Some(result)
        }
    }

    /// Set environment variable
    pub unsafe fn set_env(&mut self, name: &[u16], value: &[u16], volatile: bool) -> Status {
        (self.set_env)(
            name.as_ptr() as *const _,
            value.as_ptr() as *const _,
            volatile as Boolean,
        )
    }

    /// Get current directory
    pub unsafe fn get_cur_dir(&mut self, filesystem: Option<&[u16]>) -> Option<*const Char16> {
        let fs_ptr = filesystem
            .map(|s| s.as_ptr() as *const _)
            .unwrap_or(core::ptr::null());
        let result = (self.get_cur_dir)(fs_ptr);
        if result.is_null() {
            None
        } else {
            Some(result)
        }
    }

    /// Set current directory
    pub unsafe fn set_cur_dir(
        &mut self,
        filesystem: Option<&[u16]>,
        dir: &[u16],
    ) -> Status {
        let fs_ptr = filesystem
            .map(|s| s.as_ptr() as *const _)
            .unwrap_or(core::ptr::null());
        (self.set_cur_dir)(fs_ptr, dir.as_ptr() as *const _)
    }

    /// Open a file by name
    pub unsafe fn open_file_by_name(
        &mut self,
        filename: &[u16],
        mode: Uint64,
    ) -> Result<ShellFileHandle, Status> {
        let mut handle: ShellFileHandle = core::ptr::null_mut();
        let status =
            (self.open_file_by_name)(filename.as_ptr() as *const _, &mut handle, mode);

        if status == EFI_SUCCESS {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Close a file
    pub unsafe fn close_file(&mut self, handle: ShellFileHandle) -> Status {
        (self.close_file)(handle)
    }

    /// Read from a file
    pub unsafe fn read_file(
        &mut self,
        handle: ShellFileHandle,
        buffer: &mut [u8],
    ) -> Result<usize, Status> {
        let mut size = buffer.len();
        let status = (self.read_file)(handle, &mut size, buffer.as_mut_ptr() as *mut _);

        if status == EFI_SUCCESS {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Write to a file
    pub unsafe fn write_file(
        &mut self,
        handle: ShellFileHandle,
        buffer: &[u8],
    ) -> Result<usize, Status> {
        let mut size = buffer.len();
        let status = (self.write_file)(handle, &mut size, buffer.as_ptr() as *mut _);

        if status == EFI_SUCCESS {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Get file size
    pub unsafe fn get_file_size(&mut self, handle: ShellFileHandle) -> Result<u64, Status> {
        let mut size: u64 = 0;
        let status = (self.get_file_size)(handle, &mut size);

        if status == EFI_SUCCESS {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Check if running in batch mode
    pub unsafe fn batch_is_active(&self) -> bool {
        (self.batch_is_active)() != 0
    }

    /// Check if this is the root shell
    pub unsafe fn is_root_shell(&self) -> bool {
        (self.is_root_shell)() != 0
    }

    /// Enable page break
    pub unsafe fn enable_page_break(&self) {
        (self.enable_page_break)()
    }

    /// Disable page break
    pub unsafe fn disable_page_break(&self) {
        (self.disable_page_break)()
    }

    /// Get page break status
    pub unsafe fn get_page_break(&self) -> bool {
        (self.get_page_break)() != 0
    }
}

impl ShellParametersProtocol {
    /// Get command line argument at index
    pub unsafe fn get_arg(&self, index: usize) -> Option<*const Char16> {
        if index < self.argc {
            Some(*self.argv.add(index))
        } else {
            None
        }
    }

    /// Get all arguments as a slice
    pub unsafe fn get_args(&self) -> &[*mut Char16] {
        core::slice::from_raw_parts(self.argv, self.argc)
    }

    /// Get argument count
    pub fn arg_count(&self) -> usize {
        self.argc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_status_codes() {
        assert_eq!(SHELL_SUCCESS, 0);
        assert_eq!(SHELL_LOAD_ERROR, 1);
        assert_eq!(SHELL_INVALID_PARAMETER, 2);
    }

    #[test]
    fn test_file_modes() {
        assert_eq!(EFI_FILE_MODE_READ, 0x01);
        assert_eq!(EFI_FILE_MODE_WRITE, 0x02);
        assert!(EFI_FILE_MODE_CREATE > 0);
    }
}
