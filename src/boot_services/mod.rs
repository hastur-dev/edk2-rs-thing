// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Boot Services Table

use crate::ffi::*;

pub mod events;
pub mod safe_wrappers;
pub mod tpl;

pub use events::*;
pub use safe_wrappers::BootServicesWrapper;
pub use tpl::*;

/// EFI_BOOT_SERVICES Table
#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,

    // Task Priority Services
    pub raise_tpl: unsafe extern "efiapi" fn(new_tpl: Tpl) -> Tpl,
    pub restore_tpl: unsafe extern "efiapi" fn(old_tpl: Tpl),

    // Memory Services
    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_type: AllocateType,
        memory_type: MemoryType,
        pages: Uintn,
        memory: *mut PhysicalAddress,
    ) -> Status,
    pub free_pages: unsafe extern "efiapi" fn(memory: PhysicalAddress, pages: Uintn) -> Status,
    pub get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: *mut Uintn,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut Uintn,
        descriptor_size: *mut Uintn,
        descriptor_version: *mut Uint32,
    ) -> Status,
    pub allocate_pool: unsafe extern "efiapi" fn(
        pool_type: MemoryType,
        size: Uintn,
        buffer: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub free_pool: unsafe extern "efiapi" fn(buffer: *mut core::ffi::c_void) -> Status,

    // Event & Timer Services
    pub create_event: unsafe extern "efiapi" fn(
        event_type: Uint32,
        notify_tpl: Tpl,
        notify_function: *mut core::ffi::c_void,
        notify_context: *mut core::ffi::c_void,
        event: *mut Event,
    ) -> Status,
    pub set_timer:
        unsafe extern "efiapi" fn(event: Event, timer_type: Uint32, trigger_time: Uint64) -> Status,
    pub wait_for_event: unsafe extern "efiapi" fn(
        number_of_events: Uintn,
        event: *mut Event,
        index: *mut Uintn,
    ) -> Status,
    pub signal_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    pub close_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    pub check_event: unsafe extern "efiapi" fn(event: Event) -> Status,

    // Protocol Handler Services
    pub install_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut *mut Handle,
        protocol: *const Guid,
        interface_type: Uint32,
        interface: *mut core::ffi::c_void,
    ) -> Status,
    pub reinstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        old_interface: *mut core::ffi::c_void,
        new_interface: *mut core::ffi::c_void,
    ) -> Status,
    pub uninstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        interface: *mut core::ffi::c_void,
    ) -> Status,
    pub handle_protocol: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        interface: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub reserved: *mut core::ffi::c_void,
    pub register_protocol_notify: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        event: Event,
        registration: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub locate_handle: unsafe extern "efiapi" fn(
        search_type: Uint32,
        protocol: *const Guid,
        search_key: *mut core::ffi::c_void,
        buffer_size: *mut Uintn,
        buffer: *mut *mut Handle,
    ) -> Status,
    pub locate_device_path: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        device_path: *mut *mut core::ffi::c_void,
        device: *mut *mut Handle,
    ) -> Status,
    pub install_configuration_table:
        unsafe extern "efiapi" fn(guid: *const Guid, table: *mut core::ffi::c_void) -> Status,

    // Image Services
    pub load_image: unsafe extern "efiapi" fn(
        boot_policy: Boolean,
        parent_image_handle: *mut Handle,
        device_path: *mut core::ffi::c_void,
        source_buffer: *mut core::ffi::c_void,
        source_size: Uintn,
        image_handle: *mut *mut Handle,
    ) -> Status,
    pub start_image: unsafe extern "efiapi" fn(
        image_handle: *mut Handle,
        exit_data_size: *mut Uintn,
        exit_data: *mut *mut Char16,
    ) -> Status,
    pub exit: unsafe extern "efiapi" fn(
        image_handle: *mut Handle,
        exit_status: Status,
        exit_data_size: Uintn,
        exit_data: *mut Char16,
    ) -> Status,
    pub unload_image: unsafe extern "efiapi" fn(image_handle: *mut Handle) -> Status,
    pub exit_boot_services:
        unsafe extern "efiapi" fn(image_handle: *mut Handle, map_key: Uintn) -> Status,

    // Miscellaneous Services
    pub get_next_monotonic_count: unsafe extern "efiapi" fn(count: *mut Uint64) -> Status,
    pub stall: unsafe extern "efiapi" fn(microseconds: Uintn) -> Status,
    pub set_watchdog_timer: unsafe extern "efiapi" fn(
        timeout: Uintn,
        watchdog_code: Uint64,
        data_size: Uintn,
        watchdog_data: *mut Char16,
    ) -> Status,

    // DriverSupport Services
    pub connect_controller: unsafe extern "efiapi" fn(
        controller_handle: *mut Handle,
        driver_image_handle: *mut *mut Handle,
        remaining_device_path: *mut core::ffi::c_void,
        recursive: Boolean,
    ) -> Status,
    pub disconnect_controller: unsafe extern "efiapi" fn(
        controller_handle: *mut Handle,
        driver_image_handle: *mut Handle,
        child_handle: *mut Handle,
    ) -> Status,

    // Open and Close Protocol Services
    pub open_protocol: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        interface: *mut *mut core::ffi::c_void,
        agent_handle: *mut Handle,
        controller_handle: *mut Handle,
        attributes: Uint32,
    ) -> Status,
    pub close_protocol: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        agent_handle: *mut Handle,
        controller_handle: *mut Handle,
    ) -> Status,
    pub open_protocol_information: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        entry_buffer: *mut *mut core::ffi::c_void,
        entry_count: *mut Uintn,
    ) -> Status,

    // Library Services
    pub protocols_per_handle: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol_buffer: *mut *mut *const Guid,
        protocol_buffer_count: *mut Uintn,
    ) -> Status,
    pub locate_handle_buffer: unsafe extern "efiapi" fn(
        search_type: Uint32,
        protocol: *const Guid,
        search_key: *mut core::ffi::c_void,
        no_handles: *mut Uintn,
        buffer: *mut *mut *mut Handle,
    ) -> Status,
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        registration: *mut core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub install_multiple_protocol_interfaces: *mut core::ffi::c_void,
    pub uninstall_multiple_protocol_interfaces: *mut core::ffi::c_void,

    // CRC Services
    pub calculate_crc32: unsafe extern "efiapi" fn(
        data: *mut core::ffi::c_void,
        data_size: Uintn,
        crc32: *mut Uint32,
    ) -> Status,

    // Miscellaneous Services
    pub copy_mem: unsafe extern "efiapi" fn(
        destination: *mut core::ffi::c_void,
        source: *mut core::ffi::c_void,
        length: Uintn,
    ),
    pub set_mem:
        unsafe extern "efiapi" fn(buffer: *mut core::ffi::c_void, size: Uintn, value: Uint8),
    pub create_event_ex: unsafe extern "efiapi" fn(
        event_type: Uint32,
        notify_tpl: Tpl,
        notify_function: *mut core::ffi::c_void,
        notify_context: *mut core::ffi::c_void,
        event_group: *const Guid,
        event: *mut Event,
    ) -> Status,
}

// Boot Services signature
pub const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;
