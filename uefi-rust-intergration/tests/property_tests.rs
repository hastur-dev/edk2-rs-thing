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
        assert!(
            !is_error(code),
            "Warning code {:x} should not be error",
            code
        );
        assert!(
            !is_success(code),
            "Warning code {:x} should not be success",
            code
        );
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
    let tpl_levels = [TPL_APPLICATION, TPL_CALLBACK, TPL_NOTIFY, TPL_HIGH_LEVEL];

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

// ============================================================================
// Storage Protocol Properties
// ============================================================================

use uefi_rust_intergration::protocols::storage::*;

/// Property: SCSI command opcodes should be unique
#[test]
fn property_scsi_opcodes_unique() {
    use scsi_commands::*;

    let opcodes = [
        SCSI_TEST_UNIT_READY,
        SCSI_REQUEST_SENSE,
        SCSI_INQUIRY,
        SCSI_MODE_SELECT_6,
        SCSI_MODE_SENSE_6,
        SCSI_START_STOP_UNIT,
        SCSI_READ_CAPACITY_10,
        SCSI_READ_10,
        SCSI_WRITE_10,
        SCSI_READ_CAPACITY_16,
        SCSI_READ_16,
        SCSI_WRITE_16,
    ];

    for i in 0..opcodes.len() {
        for j in i + 1..opcodes.len() {
            assert_ne!(
                opcodes[i], opcodes[j],
                "SCSI opcodes at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: Data direction values should be distinct
#[test]
fn property_scsi_data_direction_distinct() {
    assert_ne!(SCSI_DATA_IN, SCSI_DATA_OUT);
    assert_eq!(SCSI_DATA_IN, 0);
    assert_eq!(SCSI_DATA_OUT, 1);
}

/// Property: Storage protocol GUIDs should be unique
#[test]
fn property_storage_protocol_guids_unique() {
    let guids = [
        SCSI_PASS_THRU_PROTOCOL_GUID,
        EXT_SCSI_PASS_THRU_PROTOCOL_GUID,
        NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID,
        DISK_IO_PROTOCOL_GUID,
        DISK_IO2_PROTOCOL_GUID,
        PARTITION_INFO_PROTOCOL_GUID,
    ];

    for i in 0..guids.len() {
        for j in i + 1..guids.len() {
            assert_ne!(
                guids[i], guids[j],
                "Storage protocol GUIDs at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: GPT partition type GUIDs should be unique
#[test]
fn property_gpt_partition_guids_unique() {
    use gpt_partition_types::*;

    let guids = [
        EFI_SYSTEM_PARTITION_GUID,
        MICROSOFT_BASIC_DATA_GUID,
        LINUX_FILESYSTEM_DATA_GUID,
    ];

    for i in 0..guids.len() {
        for j in i + 1..guids.len() {
            assert_ne!(
                guids[i], guids[j],
                "GPT partition GUIDs at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: SCSI builder functions should produce valid packets
#[test]
fn property_scsi_builder_produces_valid_packets() {
    let lba_values = [0, 1, 100, 1000, u32::MAX];

    for &lba in &lba_values {
        let mut buffer = [0u8; 512];
        let (packet, cdb) = scsi_builder::build_read10(lba, &mut buffer, 1_000_000);

        // Packet properties
        assert_eq!(packet.cdb_length, 10);
        assert_eq!(packet.data_direction, SCSI_DATA_IN);
        assert_eq!(packet.in_transfer_length, 512);
        assert_eq!(packet.out_transfer_length, 0);

        // CDB properties
        assert_eq!(cdb[0], scsi_commands::SCSI_READ_10);

        // LBA should be correctly encoded
        let decoded_lba = ((cdb[2] as u32) << 24)
            | ((cdb[3] as u32) << 16)
            | ((cdb[4] as u32) << 8)
            | (cdb[5] as u32);
        assert_eq!(decoded_lba, lba);
    }
}

/// Property: Block count calculation should be consistent
#[test]
fn property_block_count_calculation() {
    let test_cases = [
        (512, 1),   // 1 block
        (1024, 2),  // 2 blocks
        (4096, 8),  // 8 blocks
        (8192, 16), // 16 blocks
    ];

    for &(buffer_size, expected_blocks) in &test_cases {
        let mut buffer = vec![0u8; buffer_size];
        let (_packet, cdb) = scsi_builder::build_read10(0, &mut buffer, 1_000_000);

        let blocks = ((cdb[7] as u16) << 8) | (cdb[8] as u16);
        assert_eq!(
            blocks, expected_blocks,
            "Buffer size {} should give {} blocks",
            buffer_size, expected_blocks
        );
    }
}

/// Property: Partition type discriminants should be sequential
#[test]
fn property_partition_type_sequential() {
    assert_eq!(PartitionType::Other as u32, 0);
    assert_eq!(PartitionType::Mbr as u32, 1);
    assert_eq!(PartitionType::Gpt as u32, 2);
}

/// Property: MBR and GPT structures should have correct sizes
#[test]
fn property_partition_structure_sizes() {
    assert_eq!(core::mem::size_of::<MbrPartitionRecord>(), 16);
    assert_eq!(core::mem::size_of::<GptPartitionEntry>(), 128);

    // Union should be at least as large as largest member
    let union_size = core::mem::size_of::<PartitionInfoUnion>();
    assert!(union_size >= 128);
}

// ============================================================================
// Security Protocol Properties
// ============================================================================

use uefi_rust_intergration::protocols::security::*;

/// Property: Hash algorithm GUIDs should be unique
#[test]
fn property_hash_algorithm_guids_unique() {
    let guids = [
        HASH_ALGORITHM_SHA1_GUID,
        HASH_ALGORITHM_SHA256_GUID,
        HASH_ALGORITHM_SHA384_GUID,
        HASH_ALGORITHM_SHA512_GUID,
    ];

    for i in 0..guids.len() {
        for j in i + 1..guids.len() {
            assert_ne!(
                guids[i], guids[j],
                "Hash algorithm GUIDs at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: Certificate type GUIDs should be unique
#[test]
fn property_cert_type_guids_unique() {
    let guids = [CERT_SHA256_GUID, CERT_RSA2048_GUID, CERT_X509_GUID];

    for i in 0..guids.len() {
        for j in i + 1..guids.len() {
            assert_ne!(
                guids[i], guids[j],
                "Certificate type GUIDs at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: Security protocol GUIDs should be unique
#[test]
fn property_security_protocol_guids_unique() {
    let guids = [
        SECURITY_ARCH_PROTOCOL_GUID,
        SECURITY2_ARCH_PROTOCOL_GUID,
        HASH_PROTOCOL_GUID,
        PKCS7_VERIFY_PROTOCOL_GUID,
        TPM2_PROTOCOL_GUID,
    ];

    for i in 0..guids.len() {
        for j in i + 1..guids.len() {
            assert_ne!(
                guids[i], guids[j],
                "Security protocol GUIDs at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: Hash output sizes should match algorithm specifications
#[test]
fn property_hash_output_sizes_correct() {
    let output: HashOutput = unsafe { core::mem::zeroed() };

    unsafe {
        // SHA-1: 160 bits = 20 bytes
        assert_eq!(output.sha1_hash.len(), 160 / 8);
        // SHA-224: 224 bits = 28 bytes
        assert_eq!(output.sha224_hash.len(), 224 / 8);
        // SHA-256: 256 bits = 32 bytes
        assert_eq!(output.sha256_hash.len(), 256 / 8);
        // SHA-384: 384 bits = 48 bytes
        assert_eq!(output.sha384_hash.len(), 384 / 8);
        // SHA-512: 512 bits = 64 bytes
        assert_eq!(output.sha512_hash.len(), 512 / 8);
    }
}

/// Property: Hash output union should be at least as large as largest member
#[test]
fn property_hash_output_union_size() {
    let union_size = core::mem::size_of::<HashOutput>();
    let sha512_size = 64;

    assert!(union_size >= sha512_size);
    assert_eq!(union_size, 64); // Should be exactly SHA-512 size
}

/// Property: TPM2 command codes should be unique
#[test]
fn property_tpm2_command_codes_unique() {
    use tpm2_commands::*;

    let codes = [
        TPM_CC_STARTUP,
        TPM_CC_SELF_TEST,
        TPM_CC_GET_CAPABILITY,
        TPM_CC_PCR_READ,
        TPM_CC_PCR_EXTEND,
        TPM_CC_GET_RANDOM,
    ];

    for i in 0..codes.len() {
        for j in i + 1..codes.len() {
            assert_ne!(
                codes[i], codes[j],
                "TPM2 command codes at indices {} and {} should be unique",
                i, j
            );
        }
    }
}

/// Property: TPM2 startup types should be distinct
#[test]
fn property_tpm2_startup_types_distinct() {
    use tpm2_commands::*;

    assert_ne!(TPM_SU_CLEAR, TPM_SU_STATE);
    assert_eq!(TPM_SU_CLEAR, 0x0000);
    assert_eq!(TPM_SU_STATE, 0x0001);
}

/// Property: TPM2 command header size should be fixed
#[test]
fn property_tpm2_command_header_size() {
    assert_eq!(core::mem::size_of::<Tpm2CommandHeader>(), 10);
    assert_eq!(core::mem::size_of::<Tpm2ResponseHeader>(), 10);
}

/// Property: Signature list structures should have fixed sizes
#[test]
fn property_signature_structures_fixed_size() {
    assert_eq!(core::mem::size_of::<SignatureData>(), 16); // GUID
    assert_eq!(core::mem::size_of::<SignatureList>(), 28); // GUID + 3 Uint32
}

/// Property: Secure Boot variable names should be null-terminated
#[test]
fn property_secure_boot_variable_names_null_terminated() {
    let variable_names = [
        IMAGE_SECURITY_DATABASE_VARIABLE,
        IMAGE_SECURITY_DATABASE1_VARIABLE,
        IMAGE_SECURITY_DATABASE2_VARIABLE,
        PLATFORM_KEY_VARIABLE,
        KEY_EXCHANGE_KEY_VARIABLE,
        SECURE_BOOT_MODE_VARIABLE,
    ];

    for name in &variable_names {
        assert_eq!(
            name.last(),
            Some(&0x0000),
            "Variable name should be null-terminated"
        );
    }
}

// ============================================================================
// Cross-Protocol Properties
// ============================================================================

/// Property: All protocol GUIDs across the system should be unique
#[test]
fn property_all_protocol_guids_globally_unique() {
    let all_guids = [
        // Storage
        SCSI_PASS_THRU_PROTOCOL_GUID,
        NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID,
        DISK_IO_PROTOCOL_GUID,
        PARTITION_INFO_PROTOCOL_GUID,
        // Security
        HASH_PROTOCOL_GUID,
        PKCS7_VERIFY_PROTOCOL_GUID,
        TPM2_PROTOCOL_GUID,
    ];

    for i in 0..all_guids.len() {
        for j in i + 1..all_guids.len() {
            assert_ne!(
                all_guids[i], all_guids[j],
                "All protocol GUIDs should be globally unique (indices {}, {})",
                i, j
            );
        }
    }
}

/// Property: Structure alignments should be appropriate for their content
#[test]
fn property_structure_alignments_appropriate() {
    // Structures with 64-bit fields should be 8-byte aligned
    assert!(core::mem::align_of::<MemoryDescriptor>() >= 8);
    assert!(core::mem::align_of::<GptPartitionEntry>() >= 8);

    // Structures with only 32-bit fields can be 4-byte aligned
    assert!(core::mem::align_of::<NvmeCommand>() >= 4);

    // Packed structures should be 1-byte aligned
    assert_eq!(core::mem::align_of::<MbrPartitionRecord>(), 1);
    assert_eq!(core::mem::align_of::<Tpm2CommandHeader>(), 1);
}

/// Property: Critical constants should have expected values
#[test]
fn property_critical_constants() {
    // Success is always 0
    assert_eq!(EFI_SUCCESS, 0);

    // TPL Application is lowest
    assert_eq!(TPL_APPLICATION, 4);

    // Page size is standard
    const EFI_PAGE_SIZE: usize = 4096;
    assert_eq!(EFI_PAGE_SIZE, 4096);
}

/// Property: Reversibility - encoding and decoding should be inverse operations
#[test]
fn property_lba_encoding_reversible() {
    let test_lbas = [0u32, 1, 100, 0xABCD, 0x12345678, u32::MAX];

    for &original_lba in &test_lbas {
        // Encode LBA into big-endian bytes
        let bytes = [
            (original_lba >> 24) as u8,
            (original_lba >> 16) as u8,
            (original_lba >> 8) as u8,
            original_lba as u8,
        ];

        // Decode back
        let decoded_lba = ((bytes[0] as u32) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | (bytes[3] as u32);

        assert_eq!(
            original_lba, decoded_lba,
            "LBA encoding should be reversible"
        );
    }
}

/// Property: Signature capacity calculation should be consistent
#[test]
fn property_signature_capacity_consistent() {
    let sig_sizes = [32u32, 48, 64, 128, 256];

    for &sig_size in &sig_sizes {
        let list_size = 1024u32;
        let header_size = core::mem::size_of::<SignatureList>() as u32;

        let capacity = (list_size - header_size) / sig_size;

        // Verify reconstruction
        let reconstructed_size = header_size + (capacity * sig_size);
        assert!(reconstructed_size <= list_size);
        assert!(list_size - reconstructed_size < sig_size);
    }
}

/// Property: All error codes should have the error bit set
#[test]
fn property_all_errors_have_error_bit() {
    const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

    let all_errors = [
        EFI_LOAD_ERROR,
        EFI_INVALID_PARAMETER,
        EFI_UNSUPPORTED,
        EFI_BAD_BUFFER_SIZE,
        EFI_BUFFER_TOO_SMALL,
        EFI_NOT_READY,
        EFI_DEVICE_ERROR,
        EFI_WRITE_PROTECTED,
        EFI_OUT_OF_RESOURCES,
        EFI_NOT_FOUND,
        EFI_TIMEOUT,
        EFI_ABORTED,
        EFI_SECURITY_VIOLATION,
    ];

    for &error in &all_errors {
        assert_ne!(
            error & ERROR_BIT,
            0,
            "Error code {:x} should have error bit set",
            error
        );
    }
}
