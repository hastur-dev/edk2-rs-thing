// SPDX-License-Identifier: BSD-2-Clause-Patent
//! FFI Type Tests

#![cfg(test)]

use uefi_rust_intergration::ffi::*;

#[test]
fn test_guid_equality() {
    let guid1 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let guid2 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let guid3 = Guid::new(0x87654321, 0x4321, 0x8765, [8, 7, 6, 5, 4, 3, 2, 1]);

    assert_eq!(guid1, guid2);
    assert_ne!(guid1, guid3);
}

#[test]
fn test_guid_size() {
    // GUID must be exactly 16 bytes (128 bits)
    assert_eq!(core::mem::size_of::<Guid>(), 16);
}

#[test]
fn test_table_header_size() {
    // EFI_TABLE_HEADER size verification
    assert_eq!(core::mem::size_of::<TableHeader>(), 24);
}

#[test]
fn test_memory_descriptor_alignment() {
    // MemoryDescriptor must be properly aligned
    assert_eq!(core::mem::align_of::<MemoryDescriptor>(), 8);
}

#[test]
fn test_status_codes() {
    // Verify error bit is set correctly
    assert!(is_error(EFI_INVALID_PARAMETER));
    assert!(is_error(EFI_NOT_FOUND));
    assert!(is_error(EFI_OUT_OF_RESOURCES));

    // Verify success
    assert!(is_success(EFI_SUCCESS));
    assert!(!is_error(EFI_SUCCESS));

    // Warning codes should not be errors
    assert!(!is_error(EFI_WARN_UNKNOWN_GLYPH));
}

#[test]
fn test_boolean_values() {
    assert_eq!(FALSE, 0);
    assert_eq!(TRUE, 1);
}

#[test]
fn test_memory_type_values() {
    // Ensure memory types have correct discriminants
    assert_eq!(MemoryType::ReservedMemoryType as u32, 0);
    assert_eq!(MemoryType::LoaderCode as u32, 1);
    assert_eq!(MemoryType::LoaderData as u32, 2);
    assert_eq!(MemoryType::BootServicesCode as u32, 3);
    assert_eq!(MemoryType::BootServicesData as u32, 4);
    assert_eq!(MemoryType::ConventionalMemory as u32, 7);
}

#[test]
fn test_allocate_type_values() {
    assert_eq!(AllocateType::AllocateAnyPages as u32, 0);
    assert_eq!(AllocateType::AllocateMaxAddress as u32, 1);
    assert_eq!(AllocateType::AllocateAddress as u32, 2);
}

#[test]
fn test_tpl_levels() {
    // Verify TPL level ordering
    assert!(TPL_APPLICATION < TPL_CALLBACK);
    assert!(TPL_CALLBACK < TPL_NOTIFY);
    assert!(TPL_NOTIFY < TPL_HIGH_LEVEL);
}

#[test]
fn test_pointer_sizes() {
    // Ensure we're targeting 64-bit
    assert_eq!(core::mem::size_of::<PhysicalAddress>(), 8);
    assert_eq!(core::mem::size_of::<VirtualAddress>(), 8);
    assert_eq!(core::mem::size_of::<Uintn>(), 8);
}

#[test]
fn test_char16_size() {
    // CHAR16 must be 16-bit for UCS-2
    assert_eq!(core::mem::size_of::<Char16>(), 2);
}

#[test]
fn test_table_header_signature_verification() {
    let header = TableHeader {
        signature: 0x5453595320494249, // EFI_SYSTEM_TABLE_SIGNATURE
        revision: 0x00020064,
        header_size: 24,
        crc32: 0,
        reserved: 0,
    };

    assert!(header.verify_signature(0x5453595320494249));
    assert!(!header.verify_signature(0x0));
}

#[test]
fn test_memory_attributes() {
    // Test memory attribute bits don't overlap
    assert_eq!(EFI_MEMORY_UC & EFI_MEMORY_WC, 0);
    assert_eq!(EFI_MEMORY_WT & EFI_MEMORY_WB, 0);

    // Test runtime bit
    assert_eq!(EFI_MEMORY_RUNTIME, 0x8000000000000000);
}
