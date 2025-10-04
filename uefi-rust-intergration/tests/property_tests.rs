// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Property-Based Tests for Critical Functions

#![cfg(test)]

use uefi_rust_intergration::ffi::*;

/// Test property: All valid status codes should correctly identify as error/success
#[test]
fn property_status_classification() {
    let success_codes = [EFI_SUCCESS];
    let error_codes = [
        EFI_LOAD_ERROR,
        EFI_INVALID_PARAMETER,
        EFI_UNSUPPORTED,
        EFI_BAD_BUFFER_SIZE,
        EFI_BUFFER_TOO_SMALL,
        EFI_NOT_READY,
        EFI_DEVICE_ERROR,
        EFI_OUT_OF_RESOURCES,
        EFI_NOT_FOUND,
    ];
    let warning_codes = [
        EFI_WARN_UNKNOWN_GLYPH,
        EFI_WARN_DELETE_FAILURE,
        EFI_WARN_WRITE_FAILURE,
    ];

    for &code in &success_codes {
        assert!(is_success(code), "Code {:x} should be success", code);
        assert!(!is_error(code), "Code {:x} should not be error", code);
    }

    for &code in &error_codes {
        assert!(is_error(code), "Code {:x} should be error", code);
        assert!(!is_success(code), "Code {:x} should not be success", code);
    }

    for &code in &warning_codes {
        assert!(!is_error(code), "Warning code {:x} should not be error", code);
        assert!(!is_success(code), "Warning code {:x} should not be success", code);
    }
}

/// Test property: GUID equality is reflexive, symmetric, and transitive
#[test]
fn property_guid_equality_reflexive() {
    let guid = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(guid, guid);
}

#[test]
fn property_guid_equality_symmetric() {
    let guid1 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let guid2 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(guid1, guid2);
    assert_eq!(guid2, guid1);
}

#[test]
fn property_guid_equality_transitive() {
    let guid1 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let guid2 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let guid3 = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(guid1, guid2);
    assert_eq!(guid2, guid3);
    assert_eq!(guid1, guid3);
}

/// Test property: Different GUIDs should not be equal
#[test]
fn property_guid_inequality() {
    let base_guid = Guid::new(0x12345678, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);

    let diff_data1 = Guid::new(0xFFFFFFFF, 0x1234, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let diff_data2 = Guid::new(0x12345678, 0xFFFF, 0x5678, [1, 2, 3, 4, 5, 6, 7, 8]);
    let diff_data3 = Guid::new(0x12345678, 0x1234, 0xFFFF, [1, 2, 3, 4, 5, 6, 7, 8]);
    let diff_data4 = Guid::new(0x12345678, 0x1234, 0x5678, [9, 9, 9, 9, 9, 9, 9, 9]);

    assert_ne!(base_guid, diff_data1);
    assert_ne!(base_guid, diff_data2);
    assert_ne!(base_guid, diff_data3);
    assert_ne!(base_guid, diff_data4);
}

/// Test property: Memory type enum values should be contiguous from 0
#[test]
fn property_memory_type_contiguous() {
    assert_eq!(MemoryType::ReservedMemoryType as u32, 0);
    assert_eq!(MemoryType::LoaderCode as u32, 1);
    assert_eq!(MemoryType::LoaderData as u32, 2);
    assert_eq!(MemoryType::BootServicesCode as u32, 3);
    assert_eq!(MemoryType::BootServicesData as u32, 4);
    assert_eq!(MemoryType::RuntimeServicesCode as u32, 5);
    assert_eq!(MemoryType::RuntimeServicesData as u32, 6);
    assert_eq!(MemoryType::ConventionalMemory as u32, 7);
}

/// Test property: TPL levels should be strictly ordered
#[test]
fn property_tpl_ordering() {
    let tpl_levels = [
        TPL_APPLICATION,
        TPL_CALLBACK,
        TPL_NOTIFY,
        TPL_HIGH_LEVEL,
    ];

    for i in 0..tpl_levels.len() - 1 {
        assert!(
            tpl_levels[i] < tpl_levels[i + 1],
            "TPL level {} should be less than {}",
            tpl_levels[i],
            tpl_levels[i + 1]
        );
    }
}

/// Test property: Memory attribute bits should be independent (no overlap)
#[test]
fn property_memory_attributes_independent() {
    let attributes = [
        EFI_MEMORY_UC,
        EFI_MEMORY_WC,
        EFI_MEMORY_WT,
        EFI_MEMORY_WB,
        EFI_MEMORY_UCE,
        EFI_MEMORY_WP,
        EFI_MEMORY_RP,
        EFI_MEMORY_XP,
        EFI_MEMORY_RUNTIME,
    ];

    for i in 0..attributes.len() {
        for j in i + 1..attributes.len() {
            assert_eq!(
                attributes[i] & attributes[j],
                0,
                "Memory attributes {:x} and {:x} should not overlap",
                attributes[i],
                attributes[j]
            );
        }
    }
}

/// Test property: Each memory attribute should have exactly one bit set
#[test]
fn property_memory_attributes_single_bit() {
    let attributes = [
        EFI_MEMORY_UC,
        EFI_MEMORY_WC,
        EFI_MEMORY_WT,
        EFI_MEMORY_WB,
        EFI_MEMORY_UCE,
        EFI_MEMORY_WP,
        EFI_MEMORY_RP,
        EFI_MEMORY_XP,
        EFI_MEMORY_RUNTIME,
    ];

    for &attr in &attributes {
        let bit_count = attr.count_ones();
        assert_eq!(
            bit_count, 1,
            "Memory attribute {:x} should have exactly 1 bit set, has {}",
            attr, bit_count
        );
    }
}

/// Test property: Type sizes should match UEFI specification
#[test]
fn property_type_sizes() {
    assert_eq!(core::mem::size_of::<Boolean>(), 1);
    assert_eq!(core::mem::size_of::<Char8>(), 1);
    assert_eq!(core::mem::size_of::<Char16>(), 2);
    assert_eq!(core::mem::size_of::<Int8>(), 1);
    assert_eq!(core::mem::size_of::<Uint8>(), 1);
    assert_eq!(core::mem::size_of::<Int16>(), 2);
    assert_eq!(core::mem::size_of::<Uint16>(), 2);
    assert_eq!(core::mem::size_of::<Int32>(), 4);
    assert_eq!(core::mem::size_of::<Uint32>(), 4);
    assert_eq!(core::mem::size_of::<Int64>(), 8);
    assert_eq!(core::mem::size_of::<Uint64>(), 8);
}

/// Test property: Error bit should be consistent across all error codes
#[test]
fn property_error_bit_consistency() {
    const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

    let error_codes = [
        EFI_LOAD_ERROR,
        EFI_INVALID_PARAMETER,
        EFI_UNSUPPORTED,
        EFI_OUT_OF_RESOURCES,
        EFI_NOT_FOUND,
    ];

    for &code in &error_codes {
        assert_eq!(
            code & ERROR_BIT,
            ERROR_BIT,
            "Error code {:x} should have error bit set",
            code
        );
    }

    assert_eq!(
        EFI_SUCCESS & ERROR_BIT,
        0,
        "Success code should not have error bit set"
    );
}

/// Test property: MemoryDescriptor should be properly aligned for DMA
#[test]
fn property_memory_descriptor_alignment() {
    let alignment = core::mem::align_of::<MemoryDescriptor>();
    assert!(
        alignment >= 8,
        "MemoryDescriptor alignment {} should be at least 8",
        alignment
    );
}

/// Test property: TableHeader size should be 24 bytes per spec
#[test]
fn property_table_header_size() {
    assert_eq!(core::mem::size_of::<TableHeader>(), 24);
}
