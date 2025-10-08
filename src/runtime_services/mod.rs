// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Runtime Services Table

use crate::ffi::*;

pub mod safe_wrappers;
pub mod time;
pub mod variables;

pub use safe_wrappers::RuntimeServicesWrapper;
pub use time::{Time, TimeCapabilities, TimeService};
pub use variables::*;

/// EFI_RESET_TYPE
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResetType {
    EfiResetCold = 0,
    EfiResetWarm = 1,
    EfiResetShutdown = 2,
    EfiResetPlatformSpecific = 3,
}

/// EFI_RUNTIME_SERVICES Table
#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,

    // Time Services
    pub get_time:
        unsafe extern "efiapi" fn(time: *mut Time, capabilities: *mut TimeCapabilities) -> Status,
    pub set_time: unsafe extern "efiapi" fn(time: *const Time) -> Status,
    pub get_wakeup_time: unsafe extern "efiapi" fn(
        enabled: *mut Boolean,
        pending: *mut Boolean,
        time: *mut Time,
    ) -> Status,
    pub set_wakeup_time: unsafe extern "efiapi" fn(enable: Boolean, time: *const Time) -> Status,

    // Virtual Memory Services
    pub set_virtual_address_map: unsafe extern "efiapi" fn(
        memory_map_size: Uintn,
        descriptor_size: Uintn,
        descriptor_version: Uint32,
        virtual_map: *mut MemoryDescriptor,
    ) -> Status,
    pub convert_pointer: unsafe extern "efiapi" fn(
        debug_disposition: Uintn,
        address: *mut *mut core::ffi::c_void,
    ) -> Status,

    // Variable Services
    pub get_variable: unsafe extern "efiapi" fn(
        variable_name: *const Char16,
        vendor_guid: *const Guid,
        attributes: *mut Uint32,
        data_size: *mut Uintn,
        data: *mut core::ffi::c_void,
    ) -> Status,
    pub get_next_variable_name: unsafe extern "efiapi" fn(
        variable_name_size: *mut Uintn,
        variable_name: *mut Char16,
        vendor_guid: *mut Guid,
    ) -> Status,
    pub set_variable: unsafe extern "efiapi" fn(
        variable_name: *const Char16,
        vendor_guid: *const Guid,
        attributes: Uint32,
        data_size: Uintn,
        data: *const core::ffi::c_void,
    ) -> Status,

    // Miscellaneous Services
    pub get_next_high_mono_count: unsafe extern "efiapi" fn(high_count: *mut Uint32) -> Status,
    pub reset_system: unsafe extern "efiapi" fn(
        reset_type: ResetType,
        reset_status: Status,
        data_size: Uintn,
        reset_data: *const core::ffi::c_void,
    ) -> !,

    // UEFI 2.0 Capsule Services
    pub update_capsule: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut core::ffi::c_void,
        capsule_count: Uintn,
        scatter_gather_list: PhysicalAddress,
    ) -> Status,
    pub query_capsule_capabilities: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut core::ffi::c_void,
        capsule_count: Uintn,
        maximum_capsule_size: *mut Uint64,
        reset_type: *mut ResetType,
    ) -> Status,

    // Miscellaneous UEFI 2.0 Service
    pub query_variable_info: unsafe extern "efiapi" fn(
        attributes: Uint32,
        maximum_variable_storage_size: *mut Uint64,
        remaining_variable_storage_size: *mut Uint64,
        maximum_variable_size: *mut Uint64,
    ) -> Status,
}

// Runtime Services signature
pub const EFI_RUNTIME_SERVICES_SIGNATURE: u64 = 0x56524553544e5552;
