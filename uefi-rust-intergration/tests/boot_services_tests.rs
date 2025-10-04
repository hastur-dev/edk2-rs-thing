// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Boot Services Integration Tests

#![cfg(test)]

mod mock_uefi;

use uefi_rust_intergration::ffi::*;
use uefi_rust_intergration::boot_services::*;
use mock_uefi::*;

#[test]
fn test_boot_services_signature() {
    let bs = create_mock_boot_services();
    assert_eq!(bs.hdr.signature, EFI_BOOT_SERVICES_SIGNATURE);
}

#[test]
fn test_allocate_pages_success() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::LoaderData,
        1,
    );

    assert!(result.is_ok());
    let addr = result.unwrap();
    assert!(addr > 0);
}

#[test]
fn test_allocate_pages_zero_pages() {
    let bs = create_mock_boot_services();

    let mut addr: PhysicalAddress = 0;
    let status = unsafe {
        (bs.allocate_pages)(
            AllocateType::AllocateAnyPages,
            MemoryType::LoaderData,
            0,
            &mut addr,
        )
    };

    assert_eq!(status, EFI_INVALID_PARAMETER);
}

#[test]
fn test_free_pages_success() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let addr = wrapper.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::LoaderData,
        1,
    ).unwrap();

    let result = wrapper.free_pages(addr, 1);
    assert!(result.is_ok());
}

#[test]
fn test_allocate_pool_success() {
    init_mock_pool();
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.allocate_pool(MemoryType::LoaderData, 1024);
    assert!(result.is_ok());

    let ptr = result.unwrap();
    assert!(!ptr.is_null());

    // Clean up
    let _ = wrapper.free_pool(ptr);
    clear_mock_pool();
}

#[test]
fn test_allocate_pool_zero_size() {
    init_mock_pool();
    let bs = create_mock_boot_services();

    let mut buffer: *mut core::ffi::c_void = core::ptr::null_mut();
    let status = unsafe {
        (bs.allocate_pool)(MemoryType::LoaderData, 0, &mut buffer)
    };

    assert_eq!(status, EFI_INVALID_PARAMETER);
    clear_mock_pool();
}

#[test]
fn test_free_pool_success() {
    init_mock_pool();
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let ptr = wrapper.allocate_pool(MemoryType::LoaderData, 512).unwrap();
    let result = wrapper.free_pool(ptr);
    assert!(result.is_ok());

    clear_mock_pool();
}

#[test]
fn test_free_pool_null_pointer() {
    let bs = create_mock_boot_services();

    let status = unsafe {
        (bs.free_pool)(core::ptr::null_mut())
    };

    assert_eq!(status, EFI_INVALID_PARAMETER);
}

#[test]
fn test_multiple_allocations() {
    init_mock_pool();
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let ptr1 = wrapper.allocate_pool(MemoryType::LoaderData, 100).unwrap();
    let ptr2 = wrapper.allocate_pool(MemoryType::LoaderData, 200).unwrap();
    let ptr3 = wrapper.allocate_pool(MemoryType::LoaderData, 300).unwrap();

    assert_ne!(ptr1, ptr2);
    assert_ne!(ptr2, ptr3);
    assert_ne!(ptr1, ptr3);

    let (count, total) = get_pool_stats();
    assert_eq!(count, 3);
    assert_eq!(total, 600);

    let _ = wrapper.free_pool(ptr1);
    let _ = wrapper.free_pool(ptr2);
    let _ = wrapper.free_pool(ptr3);

    let (count_after, _) = get_pool_stats();
    assert_eq!(count_after, 0);

    clear_mock_pool();
}

#[test]
fn test_stall_success() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.stall(1000);
    assert!(result.is_ok());
}

#[test]
fn test_stall_excessive_time() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.stall(20_000_000);
    assert!(result.is_err());
}

#[test]
fn test_locate_protocol_not_found() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let guid = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let result = wrapper.locate_protocol(&guid);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EFI_NOT_FOUND);
}

#[test]
fn test_exit_boot_services_valid_key() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.exit_boot_services(core::ptr::null_mut(), 0x12345678);
    assert!(result.is_ok());
}

#[test]
fn test_exit_boot_services_invalid_key() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    let result = wrapper.exit_boot_services(core::ptr::null_mut(), 0xDEADBEEF);
    assert!(result.is_err());
}

#[test]
fn test_get_memory_map() {
    let bs = create_mock_boot_services();

    let mut map_size: Uintn = 0;
    let mut map_key: Uintn = 0;
    let mut descriptor_size: Uintn = 0;
    let mut descriptor_version: Uint32 = 0;

    let status = unsafe {
        (bs.get_memory_map)(
            &mut map_size,
            core::ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        )
    };

    assert_eq!(status, EFI_BUFFER_TOO_SMALL);
    assert_eq!(map_size, 4096);
    assert_eq!(map_key, 0x12345678);
    assert_eq!(descriptor_size, core::mem::size_of::<MemoryDescriptor>());
    assert_eq!(descriptor_version, 1);
}

#[test]
fn test_boot_services_wrapper_creation() {
    let bs = create_mock_boot_services();
    let wrapper = BootServicesWrapper::new(&bs);

    // Just ensure wrapper can be created
    let _ = wrapper;
}
