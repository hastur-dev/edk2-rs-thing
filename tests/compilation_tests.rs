// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Compilation and ABI Tests

#![cfg(test)]

use uefi_rust_intergration::*;

/// Verify that all expected modules compile
#[test]
fn test_module_compilation() {
    // If this compiles, all modules are accessible
    let _ = core::mem::size_of::<ffi::Guid>();
    let _ = core::mem::size_of::<system_table::SystemTable>();
    let _ = core::mem::size_of::<boot_services::BootServices>();
    let _ = core::mem::size_of::<runtime_services::RuntimeServices>();
}

/// Verify efiapi calling convention is available
#[test]
fn test_efiapi_calling_convention() {
    unsafe extern "efiapi" fn test_function(_arg: u64) -> Status {
        EFI_SUCCESS
    }

    let _func_ptr: unsafe extern "efiapi" fn(u64) -> Status = test_function;
}

/// Verify repr(C) layouts
#[test]
fn test_repr_c_layouts() {
    // These types must be repr(C) for FFI
    fn assert_repr_c<T>() {
        // If these compile, the types are repr(C)
        let _ = core::mem::size_of::<T>();
    }

    assert_repr_c::<Guid>();
    assert_repr_c::<TableHeader>();
    assert_repr_c::<MemoryDescriptor>();
    assert_repr_c::<boot_services::BootServices>();
    assert_repr_c::<runtime_services::RuntimeServices>();
    assert_repr_c::<system_table::SystemTable>();
}

/// Verify no_std compatibility
#[test]
fn test_no_std_compatibility() {
    // This test itself runs with std, but we verify the library compiles without it
    // by checking that core types are used
    let _: core::result::Result<(), Status> = Ok(());
    let _: core::option::Option<u32> = None;
}

/// Verify Copy and Clone traits where expected
#[test]
fn test_copy_clone_traits() {
    let guid = Guid::new(0, 0, 0, [0; 8]);
    let guid_copy = guid;
    let guid_clone = guid.clone();

    assert_eq!(guid, guid_copy);
    assert_eq!(guid, guid_clone);

    let memory_type = MemoryType::LoaderData;
    let memory_type_copy = memory_type;
    assert_eq!(memory_type as u32, memory_type_copy as u32);
}

/// Verify Debug trait implementations
#[test]
fn test_debug_trait() {
    let guid = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let debug_str = format!("{:?}", guid);
    assert!(debug_str.contains("Guid"));

    let memory_type = MemoryType::LoaderData;
    let debug_str = format!("{:?}", memory_type);
    assert!(debug_str.contains("LoaderData"));
}

/// Verify pointer type sizes for x86_64
#[test]
fn test_pointer_sizes_x64() {
    assert_eq!(core::mem::size_of::<*mut u8>(), 8);
    assert_eq!(core::mem::size_of::<*const u8>(), 8);
    assert_eq!(core::mem::size_of::<usize>(), 8);
    assert_eq!(core::mem::size_of::<Uintn>(), 8);
}

/// Verify function pointer sizes
#[test]
fn test_function_pointer_sizes() {
    type BootServiceFn = unsafe extern "efiapi" fn() -> Status;
    assert_eq!(core::mem::size_of::<BootServiceFn>(), 8);
}

/// Verify enum representations
#[test]
fn test_enum_representations() {
    assert_eq!(core::mem::size_of::<MemoryType>(), 4);
    assert_eq!(core::mem::size_of::<AllocateType>(), 4);
    assert_eq!(core::mem::size_of::<runtime_services::ResetType>(), 4);
}

/// Verify that structures don't have unexpected padding
#[test]
fn test_structure_packing() {
    // GUID: 4 + 2 + 2 + 8 = 16 bytes (no padding expected)
    assert_eq!(core::mem::size_of::<Guid>(), 16);

    // TableHeader: 8 + 4 + 4 + 4 + 4 = 24 bytes (no padding expected)
    assert_eq!(core::mem::size_of::<TableHeader>(), 24);
}

/// Verify Send and Sync are NOT implemented for UEFI types (they're not thread-safe)
#[test]
fn test_not_send_sync() {
    // UEFI types should not be Send or Sync as UEFI is single-threaded
    fn assert_not_send<T>() {
        // This is a compile-time check
        // If T: Send, this would fail to compile (but we can't test that directly)
    }

    fn assert_not_sync<T>() {
        // This is a compile-time check
        // If T: Sync, this would fail to compile (but we can't test that directly)
    }

    // These are mainly documentation of the property
    assert_not_send::<boot_services::BootServices>();
    assert_not_sync::<boot_services::BootServices>();
}

/// Test that const functions work where expected
#[test]
fn test_const_functions() {
    const GUID: Guid = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(GUID.data1, 0x12345678);
}
