// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Firmware Management Protocol

use crate::ffi::*;

/// EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID
pub const FIRMWARE_MANAGEMENT_PROTOCOL_GUID: Guid = Guid::new(
    0x86c77a67,
    0x0b97,
    0x4633,
    [0xa1, 0x87, 0x49, 0x10, 0x4d, 0x06, 0x85, 0xc7],
);

/// Image Attribute Definitions
pub const IMAGE_ATTRIBUTE_IMAGE_UPDATABLE: u64 = 0x0000000000000001;
pub const IMAGE_ATTRIBUTE_RESET_REQUIRED: u64 = 0x0000000000000002;
pub const IMAGE_ATTRIBUTE_AUTHENTICATION_REQUIRED: u64 = 0x0000000000000004;
pub const IMAGE_ATTRIBUTE_IN_USE: u64 = 0x0000000000000008;
pub const IMAGE_ATTRIBUTE_UEFI_IMAGE: u64 = 0x0000000000000010;
pub const IMAGE_ATTRIBUTE_DEPENDENCY: u64 = 0x0000000000000020;

/// Image Compatibility Definitions
pub const IMAGE_COMPATIBILITY_CHECK_SUPPORTED: u32 = 0x00000001;

/// Image Update Constants
pub const IMAGE_UPDATABLE_VALID: u64 = 0x0000000000000001;
pub const IMAGE_UPDATABLE_INVALID: u64 = 0x0000000000000002;
pub const IMAGE_UPDATABLE_INVALID_TYPE: u64 = 0x0000000000000004;
pub const IMAGE_UPDATABLE_INVALID_OLD: u64 = 0x0000000000000008;
pub const IMAGE_UPDATABLE_VALID_WITH_VENDOR_CODE: u64 = 0x0000000000000010;

/// Package Attribute Definitions
pub const PACKAGE_ATTRIBUTE_VERSION_UPDATABLE: u32 = 0x00000001;
pub const PACKAGE_ATTRIBUTE_RESET_REQUIRED: u32 = 0x00000002;
pub const PACKAGE_ATTRIBUTE_AUTHENTICATION_REQUIRED: u32 = 0x00000004;

/// EFI_FIRMWARE_IMAGE_DESCRIPTOR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FirmwareImageDescriptor {
    pub image_index: Uint8,
    pub image_type_id: Guid,
    pub image_id: Uint64,
    pub image_id_name: *const Char16,
    pub version: Uint32,
    pub version_name: *const Char16,
    pub size: Uintn,
    pub attributes_supported: Uint64,
    pub attributes_setting: Uint64,
    pub compatibilities: Uint64,
    pub lowest_supported_image_version: Uint32,
    pub last_attempt_version: Uint32,
    pub last_attempt_status: Uint32,
    pub hardware_instance: Uint64,
}

/// EFI_FIRMWARE_MANAGEMENT_PROTOCOL
#[repr(C)]
pub struct FirmwareManagementProtocol {
    pub get_image_info: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        image_info_size: *mut Uintn,
        image_info: *mut FirmwareImageDescriptor,
        descriptor_version: *mut Uint32,
        descriptor_count: *mut Uint8,
        descriptor_size: *mut Uintn,
        package_version: *mut Uint32,
        package_version_name: *mut *mut Char16,
    ) -> Status,
    pub get_image: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        image_index: Uint8,
        image: *mut core::ffi::c_void,
        image_size: *mut Uintn,
    ) -> Status,
    pub set_image: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        image_index: Uint8,
        image: *const core::ffi::c_void,
        image_size: Uintn,
        vendor_code: *const core::ffi::c_void,
        progress: Option<unsafe extern "efiapi" fn(completion: Uintn) -> Status>,
        abort_reason: *mut *mut Char16,
    ) -> Status,
    pub check_image: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        image_index: Uint8,
        image: *const core::ffi::c_void,
        image_size: Uintn,
        image_updatable: *mut Uint32,
    ) -> Status,
    pub get_package_info: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        package_version: *mut Uint32,
        package_version_name: *mut *mut Char16,
        package_version_name_max_len: *mut Uint32,
        attributes_supported: *mut Uint64,
        attributes_setting: *mut Uint64,
    ) -> Status,
    pub set_package_info: unsafe extern "efiapi" fn(
        this: *mut FirmwareManagementProtocol,
        image: *const core::ffi::c_void,
        image_size: Uintn,
        vendor_code: *const core::ffi::c_void,
        package_version: Uint32,
        package_version_name: *const Char16,
    ) -> Status,
}

impl FirmwareManagementProtocol {
    /// Get firmware image information
    pub unsafe fn get_image_info(
        &mut self,
    ) -> Result<(Vec<FirmwareImageDescriptor>, u32, u32), Status> {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let mut image_info_size = 0;
        let mut descriptor_version = 0u32;
        let mut descriptor_count = 0u8;
        let mut descriptor_size = 0;
        let mut package_version = 0u32;
        let mut package_version_name = core::ptr::null_mut();

        // First call to get size
        let status = (self.get_image_info)(
            self,
            &mut image_info_size,
            core::ptr::null_mut(),
            &mut descriptor_version,
            &mut descriptor_count,
            &mut descriptor_size,
            &mut package_version,
            &mut package_version_name,
        );

        if status != EFI_BUFFER_TOO_SMALL && status != EFI_SUCCESS {
            return Err(status);
        }

        // Allocate buffer
        let mut descriptors = Vec::with_capacity(descriptor_count as usize);
        descriptors.resize(descriptor_count as usize, core::mem::zeroed());

        // Second call to get data
        let status = (self.get_image_info)(
            self,
            &mut image_info_size,
            descriptors.as_mut_ptr(),
            &mut descriptor_version,
            &mut descriptor_count,
            &mut descriptor_size,
            &mut package_version,
            &mut package_version_name,
        );

        if status == EFI_SUCCESS {
            Ok((descriptors, descriptor_version, package_version))
        } else {
            Err(status)
        }
    }

    /// Get a firmware image
    pub unsafe fn get_image(
        &mut self,
        image_index: u8,
        buffer: &mut [u8],
    ) -> Result<usize, Status> {
        let mut image_size = buffer.len();

        let status = (self.get_image)(
            self,
            image_index,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
            &mut image_size,
        );

        if status == EFI_SUCCESS {
            Ok(image_size)
        } else {
            Err(status)
        }
    }

    /// Set (update) a firmware image
    pub unsafe fn set_image(
        &mut self,
        image_index: u8,
        image: &[u8],
        vendor_code: Option<&[u8]>,
        progress_callback: Option<unsafe extern "efiapi" fn(completion: Uintn) -> Status>,
    ) -> Result<(), Status> {
        let mut abort_reason = core::ptr::null_mut();

        let vendor_ptr = vendor_code.map_or(core::ptr::null(), |v| v.as_ptr() as *const _);

        let status = (self.set_image)(
            self,
            image_index,
            image.as_ptr() as *const core::ffi::c_void,
            image.len(),
            vendor_ptr,
            progress_callback,
            &mut abort_reason,
        );

        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Check if an image is valid for update
    pub unsafe fn check_image(
        &mut self,
        image_index: u8,
        image: &[u8],
    ) -> Result<u32, Status> {
        let mut image_updatable = 0u32;

        let status = (self.check_image)(
            self,
            image_index,
            image.as_ptr() as *const core::ffi::c_void,
            image.len(),
            &mut image_updatable,
        );

        if status == EFI_SUCCESS {
            Ok(image_updatable)
        } else {
            Err(status)
        }
    }

    /// Get package information
    pub unsafe fn get_package_info(&mut self) -> Result<(u32, u64, u64), Status> {
        let mut package_version = 0u32;
        let mut package_version_name = core::ptr::null_mut();
        let mut package_version_name_max_len = 0u32;
        let mut attributes_supported = 0u64;
        let mut attributes_setting = 0u64;

        let status = (self.get_package_info)(
            self,
            &mut package_version,
            &mut package_version_name,
            &mut package_version_name_max_len,
            &mut attributes_supported,
            &mut attributes_setting,
        );

        if status == EFI_SUCCESS {
            Ok((package_version, attributes_supported, attributes_setting))
        } else {
            Err(status)
        }
    }
}

impl FirmwareImageDescriptor {
    /// Check if image is updatable
    pub fn is_updatable(&self) -> bool {
        (self.attributes_supported & IMAGE_ATTRIBUTE_IMAGE_UPDATABLE) != 0
    }

    /// Check if reset is required after update
    pub fn requires_reset(&self) -> bool {
        (self.attributes_setting & IMAGE_ATTRIBUTE_RESET_REQUIRED) != 0
    }

    /// Check if authentication is required
    pub fn requires_authentication(&self) -> bool {
        (self.attributes_setting & IMAGE_ATTRIBUTE_AUTHENTICATION_REQUIRED) != 0
    }

    /// Check if image is currently in use
    pub fn is_in_use(&self) -> bool {
        (self.attributes_setting & IMAGE_ATTRIBUTE_IN_USE) != 0
    }

    /// Check if this is a UEFI image
    pub fn is_uefi_image(&self) -> bool {
        (self.attributes_setting & IMAGE_ATTRIBUTE_UEFI_IMAGE) != 0
    }
}
