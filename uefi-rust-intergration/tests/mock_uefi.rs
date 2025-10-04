// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Mock UEFI Environment for Testing

#![cfg(test)]
#![allow(dead_code)]

use uefi_rust_intergration::ffi::*;
use uefi_rust_intergration::boot_services::*;
use uefi_rust_intergration::runtime_services::*;
use uefi_rust_intergration::system_table::*;
use core::ptr::null_mut;
use std::collections::HashMap;
use std::sync::Mutex;

/// Mock memory pool for testing allocator
static MOCK_POOL: Mutex<Option<HashMap<usize, Vec<u8>>>> = Mutex::new(None);

/// Initialize mock pool
pub fn init_mock_pool() {
    let mut pool = MOCK_POOL.lock().unwrap();
    // Always create a fresh HashMap
    *pool = Some(HashMap::new());
}

/// Clear mock pool
pub fn clear_mock_pool() {
    let mut pool = MOCK_POOL.lock().unwrap();
    // Reset to None to fully clear state
    *pool = None;
}

/// Get mock pool statistics
pub fn get_pool_stats() -> (usize, usize) {
    let pool = MOCK_POOL.lock().unwrap();
    if let Some(ref p) = *pool {
        let allocations = p.len();
        let total_bytes: usize = p.values().map(|v| v.len()).sum();
        (allocations, total_bytes)
    } else {
        (0, 0)
    }
}

// Mock function implementations
unsafe extern "efiapi" fn mock_raise_tpl(_new_tpl: Tpl) -> Tpl {
    TPL_APPLICATION
}

unsafe extern "efiapi" fn mock_restore_tpl(_old_tpl: Tpl) {}

unsafe extern "efiapi" fn mock_allocate_pages(
    _alloc_type: AllocateType,
    _memory_type: MemoryType,
    pages: Uintn,
    memory: *mut PhysicalAddress,
) -> Status {
    if pages == 0 || memory.is_null() {
        return EFI_INVALID_PARAMETER;
    }

    // Simulate page allocation
    let addr = 0x100000 + (pages * 0x1000);
    *memory = addr as PhysicalAddress;
    EFI_SUCCESS
}

unsafe extern "efiapi" fn mock_free_pages(_memory: PhysicalAddress, _pages: Uintn) -> Status {
    EFI_SUCCESS
}

unsafe extern "efiapi" fn mock_get_memory_map(
    memory_map_size: *mut Uintn,
    _memory_map: *mut MemoryDescriptor,
    map_key: *mut Uintn,
    descriptor_size: *mut Uintn,
    descriptor_version: *mut Uint32,
) -> Status {
    if memory_map_size.is_null() {
        return EFI_INVALID_PARAMETER;
    }

    if !map_key.is_null() {
        *map_key = 0x12345678;
    }
    if !descriptor_size.is_null() {
        *descriptor_size = core::mem::size_of::<MemoryDescriptor>();
    }
    if !descriptor_version.is_null() {
        *descriptor_version = 1;
    }

    *memory_map_size = 4096;
    EFI_BUFFER_TOO_SMALL
}

unsafe extern "efiapi" fn mock_allocate_pool(
    _pool_type: MemoryType,
    size: Uintn,
    buffer: *mut *mut core::ffi::c_void,
) -> Status {
    if size == 0 || buffer.is_null() {
        return EFI_INVALID_PARAMETER;
    }

    let mut pool = MOCK_POOL.lock().unwrap();
    if pool.is_none() {
        return EFI_OUT_OF_RESOURCES;
    }

    // Allocate memory using standard allocator
    let mut vec = vec![0u8; size];
    let ptr = vec.as_mut_ptr() as usize;

    if let Some(ref mut p) = *pool {
        p.insert(ptr, vec);
        *buffer = ptr as *mut core::ffi::c_void;
        EFI_SUCCESS
    } else {
        EFI_OUT_OF_RESOURCES
    }
}

unsafe extern "efiapi" fn mock_free_pool(buffer: *mut core::ffi::c_void) -> Status {
    if buffer.is_null() {
        return EFI_INVALID_PARAMETER;
    }

    let mut pool = MOCK_POOL.lock().unwrap();
    if let Some(ref mut p) = *pool {
        let ptr = buffer as usize;
        if p.remove(&ptr).is_some() {
            EFI_SUCCESS
        } else {
            EFI_INVALID_PARAMETER
        }
    } else {
        EFI_OUT_OF_RESOURCES
    }
}

unsafe extern "efiapi" fn mock_stall(microseconds: Uintn) -> Status {
    if microseconds > 10_000_000 {
        return EFI_INVALID_PARAMETER;
    }
    EFI_SUCCESS
}

unsafe extern "efiapi" fn mock_locate_protocol(
    protocol: *const Guid,
    _registration: *mut core::ffi::c_void,
    interface: *mut *mut core::ffi::c_void,
) -> Status {
    if protocol.is_null() || interface.is_null() {
        return EFI_INVALID_PARAMETER;
    }
    *interface = null_mut();
    EFI_NOT_FOUND
}

unsafe extern "efiapi" fn mock_exit_boot_services(
    _image_handle: *mut Handle,
    map_key: Uintn,
) -> Status {
    if map_key == 0x12345678 {
        EFI_SUCCESS
    } else {
        EFI_INVALID_PARAMETER
    }
}

// Stub implementations for other functions
unsafe extern "efiapi" fn stub_not_implemented() -> Status {
    EFI_UNSUPPORTED
}

unsafe extern "efiapi" fn mock_copy_mem(
    _destination: *mut core::ffi::c_void,
    _source: *mut core::ffi::c_void,
    _length: Uintn,
) {
    // Mock implementation - does nothing
}

unsafe extern "efiapi" fn mock_set_mem(
    _buffer: *mut core::ffi::c_void,
    _size: Uintn,
    _value: Uint8,
) {
    // Mock implementation - does nothing
}

/// Create a mock Boot Services table for testing
pub fn create_mock_boot_services() -> BootServices {
    BootServices {
        hdr: TableHeader {
            signature: EFI_BOOT_SERVICES_SIGNATURE,
            revision: 0x00020064,
            header_size: core::mem::size_of::<BootServices>() as u32,
            crc32: 0,
            reserved: 0,
        },
        raise_tpl: mock_raise_tpl,
        restore_tpl: mock_restore_tpl,
        allocate_pages: mock_allocate_pages,
        free_pages: mock_free_pages,
        get_memory_map: mock_get_memory_map,
        allocate_pool: mock_allocate_pool,
        free_pool: mock_free_pool,
        create_event: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        set_timer: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        wait_for_event: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        signal_event: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        close_event: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        check_event: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        install_protocol_interface: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        reinstall_protocol_interface: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        uninstall_protocol_interface: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        handle_protocol: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        reserved: null_mut(),
        register_protocol_notify: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        locate_handle: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        locate_device_path: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        install_configuration_table: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        load_image: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        start_image: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        exit: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        unload_image: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        exit_boot_services: mock_exit_boot_services,
        get_next_monotonic_count: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        stall: mock_stall,
        set_watchdog_timer: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        connect_controller: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        disconnect_controller: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        open_protocol: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        close_protocol: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        open_protocol_information: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        protocols_per_handle: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        locate_handle_buffer: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        locate_protocol: mock_locate_protocol,
        install_multiple_protocol_interfaces: null_mut(),
        uninstall_multiple_protocol_interfaces: null_mut(),
        calculate_crc32: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
        copy_mem: mock_copy_mem,
        set_mem: mock_set_mem,
        create_event_ex: unsafe { core::mem::transmute(stub_not_implemented as *const ()) },
    }
}
