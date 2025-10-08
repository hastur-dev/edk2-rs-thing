// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Allocator Tests

#![cfg(test)]

mod mock_uefi;

use mock_uefi::*;

#[test]
fn test_mock_pool_initialization() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let (count, total) = get_pool_stats();
    assert_eq!(count, 0);
    assert_eq!(total, 0);
    clear_mock_pool();
}

#[test]
fn test_pool_allocation_tracking() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let bs = create_mock_boot_services();

    let mut ptr1: *mut core::ffi::c_void = core::ptr::null_mut();
    let status1 = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            256,
            &mut ptr1,
        )
    };

    assert_eq!(status1, uefi_rust_intergration::EFI_SUCCESS);
    assert!(!ptr1.is_null());

    let (count, total) = get_pool_stats();
    assert_eq!(count, 1);
    assert_eq!(total, 256);

    clear_mock_pool();
}

#[test]
fn test_pool_deallocation() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let bs = create_mock_boot_services();

    let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    let _ = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            128,
            &mut ptr,
        )
    };

    let (count_before, _) = get_pool_stats();
    assert_eq!(count_before, 1);

    let status = unsafe { (bs.free_pool)(ptr) };
    assert_eq!(status, uefi_rust_intergration::EFI_SUCCESS);

    let (count_after, total_after) = get_pool_stats();
    assert_eq!(count_after, 0);
    assert_eq!(total_after, 0);

    clear_mock_pool();
}

#[test]
fn test_multiple_pool_allocations() {
    // Use a lock to prevent parallel test execution from interfering
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool(); // Clear any previous state
    init_mock_pool();
    let bs = create_mock_boot_services();

    let sizes = [64, 128, 256, 512, 1024];
    let mut ptrs = vec![];

    for &size in &sizes {
        let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let status = unsafe {
            (bs.allocate_pool)(
                uefi_rust_intergration::MemoryType::LoaderData,
                size,
                &mut ptr,
            )
        };
        assert_eq!(status, uefi_rust_intergration::EFI_SUCCESS);
        ptrs.push(ptr);
    }

    let (count, total) = get_pool_stats();
    assert_eq!(count, 5, "Expected 5 allocations, got {}", count);
    assert_eq!(
        total,
        64 + 128 + 256 + 512 + 1024,
        "Expected total of 1984 bytes, got {}",
        total
    );

    for ptr in ptrs {
        let _ = unsafe { (bs.free_pool)(ptr) };
    }

    let (count_final, _) = get_pool_stats();
    assert_eq!(
        count_final, 0,
        "Expected 0 allocations after free, got {}",
        count_final
    );

    clear_mock_pool();
}

#[test]
fn test_allocation_size_limits() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let bs = create_mock_boot_services();

    // Test small allocation
    let mut small_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    let status = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            1,
            &mut small_ptr,
        )
    };
    assert_eq!(status, uefi_rust_intergration::EFI_SUCCESS);

    // Test large allocation
    let mut large_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    let status = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            1024 * 1024,
            &mut large_ptr,
        )
    };
    assert_eq!(status, uefi_rust_intergration::EFI_SUCCESS);

    let _ = unsafe { (bs.free_pool)(small_ptr) };
    let _ = unsafe { (bs.free_pool)(large_ptr) };

    clear_mock_pool();
}

#[test]
fn test_double_free_detection() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let bs = create_mock_boot_services();

    let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    let _ = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            256,
            &mut ptr,
        )
    };

    let status1 = unsafe { (bs.free_pool)(ptr) };
    assert_eq!(status1, uefi_rust_intergration::EFI_SUCCESS);

    // Second free should fail
    let status2 = unsafe { (bs.free_pool)(ptr) };
    assert_eq!(status2, uefi_rust_intergration::EFI_INVALID_PARAMETER);

    clear_mock_pool();
}

#[test]
fn test_allocation_alignment() {
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool();
    init_mock_pool();
    let bs = create_mock_boot_services();

    let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    let status = unsafe {
        (bs.allocate_pool)(
            uefi_rust_intergration::MemoryType::LoaderData,
            100,
            &mut ptr,
        )
    };

    assert_eq!(status, uefi_rust_intergration::EFI_SUCCESS);

    // Check that pointer is properly aligned (at least 8-byte for 64-bit)
    let addr = ptr as usize;
    assert_eq!(addr % 8, 0, "Allocation should be 8-byte aligned");

    let _ = unsafe { (bs.free_pool)(ptr) };
    clear_mock_pool();
}

#[test]
fn test_pool_stress_test() {
    // Use a lock to prevent parallel test execution from interfering
    use std::sync::Mutex;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let _guard = TEST_LOCK.lock().unwrap();

    clear_mock_pool(); // Clear any previous state
    init_mock_pool();
    let bs = create_mock_boot_services();

    let iterations = 100;
    let mut ptrs = vec![];

    // Allocate many small blocks
    for i in 0..iterations {
        let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let size = (i % 10 + 1) * 16;
        let status = unsafe {
            (bs.allocate_pool)(
                uefi_rust_intergration::MemoryType::LoaderData,
                size,
                &mut ptr,
            )
        };
        assert_eq!(
            status,
            uefi_rust_intergration::EFI_SUCCESS,
            "Failed to allocate block {}",
            i
        );
        ptrs.push(ptr);
    }

    let (count, _) = get_pool_stats();
    assert_eq!(
        count, iterations,
        "Expected {} allocations, got {}",
        iterations, count
    );

    // Free all blocks
    for (i, ptr) in ptrs.into_iter().enumerate() {
        let status = unsafe { (bs.free_pool)(ptr) };
        assert_eq!(
            status,
            uefi_rust_intergration::EFI_SUCCESS,
            "Failed to free block {}",
            i
        );
    }

    let (count_final, total_final) = get_pool_stats();
    assert_eq!(
        count_final, 0,
        "Expected 0 allocations after free, got {}",
        count_final
    );
    assert_eq!(
        total_final, 0,
        "Expected 0 bytes after free, got {}",
        total_final
    );

    clear_mock_pool();
}
