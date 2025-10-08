// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Driver Binding Protocol - Driver Model Support

use crate::ffi::*;

use alloc::vec::Vec;

/// EFI_DRIVER_BINDING_PROTOCOL_GUID
pub const DRIVER_BINDING_PROTOCOL_GUID: Guid = Guid::new(
    0x18A031AB,
    0xB443,
    0x4D1A,
    [0xA5, 0xC0, 0x0C, 0x09, 0x26, 0x1E, 0x9F, 0x71],
);

/// EFI_DRIVER_BINDING_PROTOCOL
#[repr(C)]
pub struct DriverBindingProtocol {
    pub supported: unsafe extern "efiapi" fn(
        this: *mut DriverBindingProtocol,
        controller_handle: *mut Handle,
        remaining_device_path: *mut core::ffi::c_void,
    ) -> Status,
    pub start: unsafe extern "efiapi" fn(
        this: *mut DriverBindingProtocol,
        controller_handle: *mut Handle,
        remaining_device_path: *mut core::ffi::c_void,
    ) -> Status,
    pub stop: unsafe extern "efiapi" fn(
        this: *mut DriverBindingProtocol,
        controller_handle: *mut Handle,
        number_of_children: Uintn,
        child_handle_buffer: *mut *mut Handle,
    ) -> Status,
    pub version: Uint32,
    pub image_handle: *mut Handle,
    pub driver_binding_handle: *mut Handle,
}

impl DriverBindingProtocol {
    /// Test if this driver supports the given controller
    pub unsafe fn supported(
        &mut self,
        controller_handle: *mut Handle,
        remaining_device_path: *mut core::ffi::c_void,
    ) -> Status {
        (self.supported)(self, controller_handle, remaining_device_path)
    }

    /// Start managing a controller
    pub unsafe fn start(
        &mut self,
        controller_handle: *mut Handle,
        remaining_device_path: *mut core::ffi::c_void,
    ) -> Status {
        (self.start)(self, controller_handle, remaining_device_path)
    }

    /// Stop managing a controller
    pub unsafe fn stop(
        &mut self,
        controller_handle: *mut Handle,
        child_handles: &[*mut Handle],
    ) -> Status {
        (self.stop)(
            self,
            controller_handle,
            child_handles.len(),
            child_handles.as_ptr() as *mut *mut Handle,
        )
    }
}

/// EFI_COMPONENT_NAME_PROTOCOL_GUID
pub const COMPONENT_NAME_PROTOCOL_GUID: Guid = Guid::new(
    0x107A772C,
    0xD5E1,
    0x11D4,
    [0x9A, 0x46, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// EFI_COMPONENT_NAME2_PROTOCOL_GUID
pub const COMPONENT_NAME2_PROTOCOL_GUID: Guid = Guid::new(
    0x6A7A5CFF,
    0xE8D9,
    0x4F70,
    [0xBA, 0xDA, 0x75, 0xAB, 0x30, 0x25, 0xCE, 0x14],
);

/// EFI_COMPONENT_NAME2_PROTOCOL
#[repr(C)]
pub struct ComponentName2Protocol {
    pub get_driver_name: unsafe extern "efiapi" fn(
        this: *mut ComponentName2Protocol,
        language: *const Char8,
        driver_name: *mut *mut Char16,
    ) -> Status,
    pub get_controller_name: unsafe extern "efiapi" fn(
        this: *mut ComponentName2Protocol,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        language: *const Char8,
        controller_name: *mut *mut Char16,
    ) -> Status,
    pub supported_languages: *const Char8,
}

impl ComponentName2Protocol {
    /// Get the driver name
    pub unsafe fn get_driver_name(&mut self, language: &[u8]) -> Result<*mut Char16, Status> {
        let mut driver_name = core::ptr::null_mut();
        let status = (self.get_driver_name)(self, language.as_ptr(), &mut driver_name);

        if status == EFI_SUCCESS {
            Ok(driver_name)
        } else {
            Err(status)
        }
    }

    /// Get the controller name
    pub unsafe fn get_controller_name(
        &mut self,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        language: &[u8],
    ) -> Result<*mut Char16, Status> {
        let mut controller_name = core::ptr::null_mut();
        let status = (self.get_controller_name)(
            self,
            controller_handle,
            child_handle,
            language.as_ptr(),
            &mut controller_name,
        );

        if status == EFI_SUCCESS {
            Ok(controller_name)
        } else {
            Err(status)
        }
    }
}

/// EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID
pub const DRIVER_DIAGNOSTICS_PROTOCOL_GUID: Guid = Guid::new(
    0x0784924F,
    0xE296,
    0x11D4,
    [0x9A, 0x49, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID
pub const DRIVER_DIAGNOSTICS2_PROTOCOL_GUID: Guid = Guid::new(
    0x4D330321,
    0x025F,
    0x4AAC,
    [0x90, 0xD8, 0x5E, 0xD9, 0x00, 0x17, 0x3B, 0x63],
);

/// Diagnostics Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiagnosticsType {
    Standard = 0,
    Extended = 1,
    Manufacturing = 2,
}

/// EFI_DRIVER_DIAGNOSTICS2_PROTOCOL
#[repr(C)]
pub struct DriverDiagnostics2Protocol {
    pub run_diagnostics: unsafe extern "efiapi" fn(
        this: *mut DriverDiagnostics2Protocol,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        diagnostic_type: DiagnosticsType,
        language: *const Char8,
        error_type: *mut *mut Guid,
        buffer_size: *mut Uintn,
        buffer: *mut *mut Char16,
    ) -> Status,
    pub supported_languages: *const Char8,
}

impl DriverDiagnostics2Protocol {
    /// Run diagnostics on a controller
    pub unsafe fn run_diagnostics(
        &mut self,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        diagnostic_type: DiagnosticsType,
        language: &[u8],
    ) -> Result<(Guid, Vec<u16>), Status> {
        let mut error_type = core::ptr::null_mut();
        let mut buffer_size = 0;
        let mut buffer = core::ptr::null_mut();

        let status = (self.run_diagnostics)(
            self,
            controller_handle,
            child_handle,
            diagnostic_type,
            language.as_ptr(),
            &mut error_type,
            &mut buffer_size,
            &mut buffer,
        );

        if status == EFI_SUCCESS && !error_type.is_null() && !buffer.is_null() {
            let guid = *error_type;
            let result_buffer = core::slice::from_raw_parts(buffer, buffer_size).to_vec();
            Ok((guid, result_buffer))
        } else {
            Err(status)
        }
    }
}

/// EFI_DRIVER_CONFIGURATION_PROTOCOL_GUID
pub const DRIVER_CONFIGURATION_PROTOCOL_GUID: Guid = Guid::new(
    0x107A772B,
    0xD5E1,
    0x11D4,
    [0x9A, 0x46, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Configuration Action
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConfigurationAction {
    ConfigurationActionNone = 0,
    ConfigurationActionStopController = 1,
    ConfigurationActionRestartController = 2,
    ConfigurationActionRestartPlatform = 3,
    ConfigurationActionMaximum = 4,
}

/// EFI_DRIVER_CONFIGURATION2_PROTOCOL
#[repr(C)]
pub struct DriverConfiguration2Protocol {
    pub set_options: unsafe extern "efiapi" fn(
        this: *mut DriverConfiguration2Protocol,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        language: *const Char8,
        action_required: *mut ConfigurationAction,
    ) -> Status,
    pub options_valid: unsafe extern "efiapi" fn(
        this: *mut DriverConfiguration2Protocol,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
    ) -> Status,
    pub force_defaults: unsafe extern "efiapi" fn(
        this: *mut DriverConfiguration2Protocol,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        default_type: Uint32,
        action_required: *mut ConfigurationAction,
    ) -> Status,
    pub supported_languages: *const Char8,
}

impl DriverConfiguration2Protocol {
    /// Set driver configuration options
    pub unsafe fn set_options(
        &mut self,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        language: &[u8],
    ) -> Result<ConfigurationAction, Status> {
        let mut action_required = ConfigurationAction::ConfigurationActionNone;

        let status = (self.set_options)(
            self,
            controller_handle,
            child_handle,
            language.as_ptr(),
            &mut action_required,
        );

        if status == EFI_SUCCESS {
            Ok(action_required)
        } else {
            Err(status)
        }
    }

    /// Check if options are valid
    pub unsafe fn options_valid(
        &mut self,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
    ) -> Status {
        (self.options_valid)(self, controller_handle, child_handle)
    }

    /// Force default configuration
    pub unsafe fn force_defaults(
        &mut self,
        controller_handle: *mut Handle,
        child_handle: *mut Handle,
        default_type: u32,
    ) -> Result<ConfigurationAction, Status> {
        let mut action_required = ConfigurationAction::ConfigurationActionNone;

        let status = (self.force_defaults)(
            self,
            controller_handle,
            child_handle,
            default_type,
            &mut action_required,
        );

        if status == EFI_SUCCESS {
            Ok(action_required)
        } else {
            Err(status)
        }
    }
}
