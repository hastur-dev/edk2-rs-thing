// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Human Interface Infrastructure (HII) Protocols
//!
//! The HII Database is used for managing strings, fonts, forms, keyboards, and images.

use crate::ffi::*;
use crate::runtime_services::Time;

/// EFI_HII_DATABASE_PROTOCOL_GUID
pub const HII_DATABASE_PROTOCOL_GUID: Guid = Guid::new(
    0xef9fc172,
    0xa1b2,
    0x4693,
    [0xb3, 0x27, 0x6d, 0x32, 0xfc, 0x41, 0x60, 0x42],
);

/// EFI_HII_STRING_PROTOCOL_GUID
pub const HII_STRING_PROTOCOL_GUID: Guid = Guid::new(
    0x0fd96974,
    0x23aa,
    0x4cdc,
    [0xb9, 0xcb, 0x98, 0xd1, 0x77, 0x50, 0x32, 0x2a],
);

/// EFI_HII_FONT_PROTOCOL_GUID
pub const HII_FONT_PROTOCOL_GUID: Guid = Guid::new(
    0xe9ca4775,
    0x8657,
    0x47fc,
    [0x97, 0xe7, 0x7e, 0xd6, 0x5a, 0x08, 0x43, 0x24],
);

/// EFI_HII_IMAGE_PROTOCOL_GUID
pub const HII_IMAGE_PROTOCOL_GUID: Guid = Guid::new(
    0x31a6406a,
    0x6bdf,
    0x4e46,
    [0xb2, 0xa2, 0xeb, 0xaa, 0x89, 0xc4, 0x09, 0x20],
);

/// EFI_HII_CONFIG_ACCESS_PROTOCOL_GUID
pub const HII_CONFIG_ACCESS_PROTOCOL_GUID: Guid = Guid::new(
    0x330d4706,
    0xf2a0,
    0x4e4f,
    [0xa3, 0x69, 0xb6, 0x6f, 0xa8, 0xd5, 0x43, 0x85],
);

/// EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID
pub const HII_CONFIG_ROUTING_PROTOCOL_GUID: Guid = Guid::new(
    0x587e72d7,
    0xcc50,
    0x4f79,
    [0x82, 0x09, 0xca, 0x29, 0x1f, 0xc1, 0xa1, 0x0f],
);

/// HII Handle type
pub type HiiHandle = *mut core::ffi::c_void;

/// HII Database Package List Header
#[repr(C)]
pub struct HiiPackageListHeader {
    pub package_list_guid: Guid,
    pub package_length: Uint32,
}

/// HII Package Header
#[repr(C)]
pub struct HiiPackageHeader {
    pub length: Uint32,
    pub package_type: Uint8,
    // Followed by package data
}

/// HII Package Types
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HiiPackageType {
    All = 0x00,
    Guid = 0x01,
    Forms = 0x02,
    Strings = 0x04,
    Fonts = 0x05,
    Images = 0x06,
    SimpleFonts = 0x07,
    DevicePath = 0x08,
    KeyboardLayout = 0x09,
    Animations = 0x0A,
    End = 0xDF,
    SystemBegin = 0xE0,
    SystemEnd = 0xFF,
}

/// EFI_HII_DATABASE_PROTOCOL
#[repr(C)]
pub struct HiiDatabaseProtocol {
    pub new_package_list: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        package_list: *const HiiPackageListHeader,
        driver_handle: Handle,
        handle: *mut HiiHandle,
    ) -> Status,
    pub remove_package_list:
        unsafe extern "efiapi" fn(this: *mut HiiDatabaseProtocol, handle: HiiHandle) -> Status,
    pub update_package_list: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        handle: HiiHandle,
        package_list: *const HiiPackageListHeader,
    ) -> Status,
    pub list_package_lists: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        package_type: Uint8,
        package_guid: *const Guid,
        buffer_length: *mut Uintn,
        handle: *mut HiiHandle,
    ) -> Status,
    pub export_package_lists: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        handle: HiiHandle,
        buffer_size: *mut Uintn,
        buffer: *mut HiiPackageListHeader,
    ) -> Status,
    pub register_package_notify: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        package_type: Uint8,
        package_guid: *const Guid,
        package_notify_fn: *mut core::ffi::c_void,
        notify_type: Uintn,
        notify_handle: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub unregister_package_notify: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        notification_handle: *mut core::ffi::c_void,
    ) -> Status,
    pub find_keyboard_layouts: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        key_guid_buffer_length: *mut Uint16,
        key_guid_buffer: *mut Guid,
    ) -> Status,
    pub get_keyboard_layout: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        key_guid: *const Guid,
        keyboard_layout_length: *mut Uint16,
        keyboard_layout: *mut core::ffi::c_void,
    ) -> Status,
    pub set_keyboard_layout:
        unsafe extern "efiapi" fn(this: *mut HiiDatabaseProtocol, key_guid: *const Guid) -> Status,
    pub get_package_list_handle: unsafe extern "efiapi" fn(
        this: *mut HiiDatabaseProtocol,
        package_list_handle: HiiHandle,
        driver_handle: *mut Handle,
    ) -> Status,
}

/// String ID type
pub type StringId = Uint16;

/// Language type (RFC 4646 format)
pub type Language = *mut Char8;

/// EFI_HII_STRING_PROTOCOL
#[repr(C)]
pub struct HiiStringProtocol {
    pub new_string: unsafe extern "efiapi" fn(
        this: *mut HiiStringProtocol,
        package_list: HiiHandle,
        string_id: *mut StringId,
        language: Language,
        language_name: *const Char16,
        string: *const Char16,
        string_font_info: *const HiiFontInfo,
    ) -> Status,
    pub get_string: unsafe extern "efiapi" fn(
        this: *mut HiiStringProtocol,
        language: Language,
        package_list: HiiHandle,
        string_id: StringId,
        string: *mut Char16,
        string_size: *mut Uintn,
        string_font_info: *mut *mut HiiFontInfo,
    ) -> Status,
    pub set_string: unsafe extern "efiapi" fn(
        this: *mut HiiStringProtocol,
        package_list: HiiHandle,
        string_id: StringId,
        language: Language,
        string: *const Char16,
        string_font_info: *const HiiFontInfo,
    ) -> Status,
    pub get_languages: unsafe extern "efiapi" fn(
        this: *mut HiiStringProtocol,
        package_list: HiiHandle,
        languages: *mut Char8,
        languages_size: *mut Uintn,
    ) -> Status,
    pub get_secondary_languages: unsafe extern "efiapi" fn(
        this: *mut HiiStringProtocol,
        package_list: HiiHandle,
        primary_language: Language,
        secondary_languages: *mut Char8,
        secondary_languages_size: *mut Uintn,
    ) -> Status,
}

/// HII Font Information
#[repr(C)]
pub struct HiiFontInfo {
    pub font_style: HiiFontStyle,
    pub font_size: Uint16,
    pub font_name: [Char16; 1], // Variable length
}

/// HII Font Style flags
pub type HiiFontStyle = Uint32;

pub const HII_FONT_STYLE_NORMAL: HiiFontStyle = 0x00000000;
pub const HII_FONT_STYLE_BOLD: HiiFontStyle = 0x00000001;
pub const HII_FONT_STYLE_ITALIC: HiiFontStyle = 0x00000002;
pub const HII_FONT_STYLE_EMBOSS: HiiFontStyle = 0x00010000;
pub const HII_FONT_STYLE_OUTLINE: HiiFontStyle = 0x00020000;
pub const HII_FONT_STYLE_SHADOW: HiiFontStyle = 0x00040000;
pub const HII_FONT_STYLE_UNDERLINE: HiiFontStyle = 0x00080000;
pub const HII_FONT_STYLE_DBL_UNDER: HiiFontStyle = 0x00100000;

/// Image ID type
pub type ImageId = Uint16;

/// EFI_HII_IMAGE_PROTOCOL
#[repr(C)]
pub struct HiiImageProtocol {
    pub new_image: unsafe extern "efiapi" fn(
        this: *mut HiiImageProtocol,
        package_list: HiiHandle,
        image_id: *mut ImageId,
        image: *const crate::protocols::GraphicsOutputBltPixel,
    ) -> Status,
    pub get_image: unsafe extern "efiapi" fn(
        this: *mut HiiImageProtocol,
        package_list: HiiHandle,
        image_id: ImageId,
        image: *mut crate::protocols::GraphicsOutputBltPixel,
    ) -> Status,
    pub set_image: unsafe extern "efiapi" fn(
        this: *mut HiiImageProtocol,
        package_list: HiiHandle,
        image_id: ImageId,
        image: *const crate::protocols::GraphicsOutputBltPixel,
    ) -> Status,
    pub draw_image: unsafe extern "efiapi" fn(
        this: *mut HiiImageProtocol,
        flags: Uint32,
        image: *const crate::protocols::GraphicsOutputBltPixel,
        blt: *mut *mut crate::protocols::GraphicsOutputBltPixel,
        blt_x: Uintn,
        blt_y: Uintn,
    ) -> Status,
    pub draw_image_id: unsafe extern "efiapi" fn(
        this: *mut HiiImageProtocol,
        flags: Uint32,
        package_list: HiiHandle,
        image_id: ImageId,
        blt: *mut *mut crate::protocols::GraphicsOutputBltPixel,
        blt_x: Uintn,
        blt_y: Uintn,
    ) -> Status,
}

/// EFI_HII_FONT_PROTOCOL
#[repr(C)]
pub struct HiiFontProtocol {
    pub string_to_image: unsafe extern "efiapi" fn(
        this: *mut HiiFontProtocol,
        flags: Uint32,
        string: *const Char16,
        string_info: *const HiiFontDisplayInfo,
        blt: *mut *mut crate::protocols::GraphicsOutputBltPixel,
        blt_x: Uintn,
        blt_y: Uintn,
        row_info_array: *mut *mut HiiRowInfo,
        row_info_array_size: *mut Uintn,
        column_info_array: *mut Uintn,
    ) -> Status,
    pub string_id_to_image: unsafe extern "efiapi" fn(
        this: *mut HiiFontProtocol,
        flags: Uint32,
        package_list: HiiHandle,
        string_id: StringId,
        language: Language,
        string_info: *const HiiFontDisplayInfo,
        blt: *mut *mut crate::protocols::GraphicsOutputBltPixel,
        blt_x: Uintn,
        blt_y: Uintn,
        row_info_array: *mut *mut HiiRowInfo,
        row_info_array_size: *mut Uintn,
        column_info_array: *mut Uintn,
    ) -> Status,
    pub get_glyph: unsafe extern "efiapi" fn(
        this: *mut HiiFontProtocol,
        char: Char16,
        string_info: *const HiiFontDisplayInfo,
        blt: *mut *mut crate::protocols::GraphicsOutputBltPixel,
        baseline: *mut Uintn,
    ) -> Status,
    pub get_font_info: unsafe extern "efiapi" fn(
        this: *mut HiiFontProtocol,
        font_handle: *mut HiiFontHandle,
        string_info_in: *const HiiFontDisplayInfo,
        string_info_out: *mut *mut HiiFontDisplayInfo,
        string: *const Char16,
    ) -> Status,
}

/// HII Font Display Info
#[repr(C)]
pub struct HiiFontDisplayInfo {
    pub foreground_color: crate::protocols::GraphicsOutputBltPixel,
    pub background_color: crate::protocols::GraphicsOutputBltPixel,
    pub font_info_mask: HiiFontInfoMask,
    pub font_info: HiiFontInfo,
}

/// HII Font Info Mask
pub type HiiFontInfoMask = Uint32;

pub const HII_FONT_INFO_SYS_FONT: HiiFontInfoMask = 0x00000001;
pub const HII_FONT_INFO_SYS_SIZE: HiiFontInfoMask = 0x00000002;
pub const HII_FONT_INFO_SYS_STYLE: HiiFontInfoMask = 0x00000004;
pub const HII_FONT_INFO_SYS_FORE_COLOR: HiiFontInfoMask = 0x00000010;
pub const HII_FONT_INFO_SYS_BACK_COLOR: HiiFontInfoMask = 0x00000020;
pub const HII_FONT_INFO_RESIZE: HiiFontInfoMask = 0x00001000;
pub const HII_FONT_INFO_RESTYLE: HiiFontInfoMask = 0x00002000;
pub const HII_FONT_INFO_ANY_FONT: HiiFontInfoMask = 0x00010000;
pub const HII_FONT_INFO_ANY_SIZE: HiiFontInfoMask = 0x00020000;
pub const HII_FONT_INFO_ANY_STYLE: HiiFontInfoMask = 0x00040000;

/// HII Font Handle
pub type HiiFontHandle = *mut core::ffi::c_void;

/// HII Row Info
#[repr(C)]
pub struct HiiRowInfo {
    pub start_index: Uintn,
    pub end_index: Uintn,
    pub line_height: Uintn,
    pub line_width: Uintn,
    pub baseline_offset: Uintn,
}

/// EFI_HII_CONFIG_ACCESS_PROTOCOL
#[repr(C)]
pub struct HiiConfigAccessProtocol {
    pub extract_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigAccessProtocol,
        request: *const Char16,
        progress: *mut *mut Char16,
        results: *mut *mut Char16,
    ) -> Status,
    pub route_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigAccessProtocol,
        configuration: *const Char16,
        progress: *mut *mut Char16,
    ) -> Status,
    pub callback: unsafe extern "efiapi" fn(
        this: *mut HiiConfigAccessProtocol,
        action: Uintn,
        question_id: Uint16,
        form_type: Uint8,
        value: *mut HiiConfigAccessValue,
        action_request: *mut Uintn,
    ) -> Status,
}

/// HII Config Access Value Union
#[repr(C)]
pub union HiiConfigAccessValue {
    pub u8_value: Uint8,
    pub u16_value: Uint16,
    pub u32_value: Uint32,
    pub u64_value: Uint64,
    pub bool_value: Boolean,
    pub time_value: Time,
    pub date_value: Time,
    pub string_value: StringId,
    pub buffer: *mut core::ffi::c_void,
}

/// EFI_HII_CONFIG_ROUTING_PROTOCOL
#[repr(C)]
pub struct HiiConfigRoutingProtocol {
    pub extract_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        request: *const Char16,
        progress: *mut *mut Char16,
        results: *mut *mut Char16,
    ) -> Status,
    pub export_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        results: *mut *mut Char16,
    ) -> Status,
    pub route_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        configuration: *const Char16,
        progress: *mut *mut Char16,
    ) -> Status,
    pub block_to_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        config_request: *const Char16,
        block: *const Uint8,
        block_size: Uintn,
        config: *mut *mut Char16,
        progress: *mut *mut Char16,
    ) -> Status,
    pub config_to_block: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        config_resp: *const Char16,
        block: *mut Uint8,
        block_size: *mut Uintn,
        progress: *mut *mut Char16,
    ) -> Status,
    pub get_alt_config: unsafe extern "efiapi" fn(
        this: *mut HiiConfigRoutingProtocol,
        config_resp: *const Char16,
        guid: *const Guid,
        name: *const Char16,
        device_path: *const core::ffi::c_void,
        alt_cfg_id: *const Char16,
        alt_cfg_resp: *mut *mut Char16,
    ) -> Status,
}

impl HiiStringProtocol {
    /// Create a new string in the database
    pub unsafe fn new_string(
        &mut self,
        package_list: HiiHandle,
        language: &[u8],
        string: &[u16],
    ) -> Result<StringId, Status> {
        let mut string_id: StringId = 0;
        let status = (self.new_string)(
            self,
            package_list,
            &mut string_id,
            language.as_ptr() as *mut _,
            core::ptr::null(),
            string.as_ptr() as *const _,
            core::ptr::null(),
        );

        if status == EFI_SUCCESS {
            Ok(string_id)
        } else {
            Err(status)
        }
    }

    /// Get a string from the database
    pub unsafe fn get_string(
        &mut self,
        package_list: HiiHandle,
        string_id: StringId,
        language: &[u8],
        buffer: &mut [u16],
    ) -> Result<usize, Status> {
        let mut size = buffer.len() * 2;
        let status = (self.get_string)(
            self,
            language.as_ptr() as *mut _,
            package_list,
            string_id,
            buffer.as_mut_ptr() as *mut _,
            &mut size,
            core::ptr::null_mut(),
        );

        if status == EFI_SUCCESS {
            Ok(size / 2)
        } else {
            Err(status)
        }
    }
}

impl HiiDatabaseProtocol {
    /// Register a new package list
    pub unsafe fn new_package_list(
        &mut self,
        package_list: &HiiPackageListHeader,
        driver_handle: Handle,
    ) -> Result<HiiHandle, Status> {
        let mut handle: HiiHandle = core::ptr::null_mut();
        let status =
            (self.new_package_list)(self, package_list as *const _, driver_handle, &mut handle);

        if status == EFI_SUCCESS {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Remove a package list
    pub unsafe fn remove_package_list(&mut self, handle: HiiHandle) -> Status {
        (self.remove_package_list)(self, handle)
    }
}
