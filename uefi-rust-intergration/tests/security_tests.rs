// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Security Protocol Tests

#![cfg(test)]

use uefi_rust::ffi::*;
use uefi_rust::protocols::security::*;

// ============================================================================
// Hash Algorithm GUID Tests
// ============================================================================

#[test]
fn test_hash_algorithm_guids_unique() {
    // Verify all hash algorithm GUIDs are unique
    assert_ne!(HASH_ALGORITHM_SHA1_GUID, HASH_ALGORITHM_SHA256_GUID);
    assert_ne!(HASH_ALGORITHM_SHA256_GUID, HASH_ALGORITHM_SHA384_GUID);
    assert_ne!(HASH_ALGORITHM_SHA384_GUID, HASH_ALGORITHM_SHA512_GUID);
    assert_ne!(HASH_ALGORITHM_SHA1_GUID, HASH_ALGORITHM_SHA512_GUID);
}

#[test]
fn test_hash_algorithm_guids_valid() {
    // Verify GUIDs are not all zeros
    assert_ne!(HASH_ALGORITHM_SHA1_GUID.data1, 0);
    assert_ne!(HASH_ALGORITHM_SHA256_GUID.data1, 0);
    assert_ne!(HASH_ALGORITHM_SHA384_GUID.data1, 0);
    assert_ne!(HASH_ALGORITHM_SHA512_GUID.data1, 0);
}

#[test]
fn test_sha256_guid_value() {
    // Verify SHA256 GUID matches spec
    assert_eq!(HASH_ALGORITHM_SHA256_GUID.data1, 0x51aa59de);
    assert_eq!(HASH_ALGORITHM_SHA256_GUID.data2, 0xfdf2);
    assert_eq!(HASH_ALGORITHM_SHA256_GUID.data3, 0x4ea3);
}

// ============================================================================
// Hash Output Union Tests
// ============================================================================

#[test]
fn test_hash_output_sizes() {
    let output: HashOutput = unsafe { core::mem::zeroed() };

    unsafe {
        assert_eq!(output.sha1_hash.len(), 20);
        assert_eq!(output.sha224_hash.len(), 28);
        assert_eq!(output.sha256_hash.len(), 32);
        assert_eq!(output.sha384_hash.len(), 48);
        assert_eq!(output.sha512_hash.len(), 64);
    }
}

#[test]
fn test_hash_output_union_size() {
    let size = core::mem::size_of::<HashOutput>();
    // Union should be as large as the largest member (SHA-512 = 64 bytes)
    assert_eq!(size, 64);
}

// ============================================================================
// Certificate Type GUID Tests
// ============================================================================

#[test]
fn test_certificate_type_guids_unique() {
    assert_ne!(CERT_SHA256_GUID, CERT_RSA2048_GUID);
    assert_ne!(CERT_RSA2048_GUID, CERT_X509_GUID);
    assert_ne!(CERT_SHA256_GUID, CERT_X509_GUID);
}

#[test]
fn test_cert_sha256_guid_value() {
    assert_eq!(CERT_SHA256_GUID.data1, 0xc1c41626);
    assert_eq!(CERT_SHA256_GUID.data2, 0x504c);
}

#[test]
fn test_cert_x509_guid_value() {
    assert_eq!(CERT_X509_GUID.data1, 0xa5c059a1);
    assert_eq!(CERT_X509_GUID.data2, 0x94e4);
}

// ============================================================================
// Security Protocol GUID Tests
// ============================================================================

#[test]
fn test_security_protocol_guids_unique() {
    assert_ne!(SECURITY_ARCH_PROTOCOL_GUID, SECURITY2_ARCH_PROTOCOL_GUID);
    assert_ne!(HASH_PROTOCOL_GUID, PKCS7_VERIFY_PROTOCOL_GUID);
    assert_ne!(TPM2_PROTOCOL_GUID, HASH_PROTOCOL_GUID);
}

#[test]
fn test_security_protocol_guids_valid() {
    assert_ne!(SECURITY_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(SECURITY2_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(HASH_PROTOCOL_GUID.data1, 0);
    assert_ne!(PKCS7_VERIFY_PROTOCOL_GUID.data1, 0);
    assert_ne!(TPM2_PROTOCOL_GUID.data1, 0);
}

// ============================================================================
// Signature Structure Tests
// ============================================================================

#[test]
fn test_signature_data_size() {
    let size = core::mem::size_of::<SignatureData>();
    // Should be at least GUID size
    assert_eq!(size, 16);
}

#[test]
fn test_signature_list_size() {
    let size = core::mem::size_of::<SignatureList>();
    // GUID + 3 x Uint32 = 16 + 12 = 28
    assert_eq!(size, 28);
}

#[test]
fn test_signature_list_fields() {
    let sig_list = SignatureList {
        signature_type: CERT_SHA256_GUID,
        signature_list_size: 100,
        signature_header_size: 0,
        signature_size: 48, // GUID + SHA256 = 16 + 32
    };

    assert_eq!(sig_list.signature_type, CERT_SHA256_GUID);
    assert_eq!(sig_list.signature_list_size, 100);
    assert_eq!(sig_list.signature_size, 48);
}

// ============================================================================
// TPM 2.0 Protocol Tests
// ============================================================================

#[test]
fn test_tpm2_version_size() {
    let size = core::mem::size_of::<Tpm2Version>();
    assert_eq!(size, 2); // major + minor = 2 bytes
}

#[test]
fn test_tpm2_version_fields() {
    let version = Tpm2Version { major: 1, minor: 2 };

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
}

#[test]
fn test_tpm2_boot_service_capability_size() {
    let size = core::mem::size_of::<Tpm2BootServiceCapability>();
    // Should be at least 32 bytes
    assert!(size >= 32);
}

#[test]
fn test_tpm2_command_header_size() {
    let size = core::mem::size_of::<Tpm2CommandHeader>();
    // tag (2) + param_size (4) + command_code (4) = 10 bytes
    assert_eq!(size, 10);
}

#[test]
fn test_tpm2_response_header_size() {
    let size = core::mem::size_of::<Tpm2ResponseHeader>();
    // tag (2) + param_size (4) + response_code (4) = 10 bytes
    assert_eq!(size, 10);
}

// ============================================================================
// TPM 2.0 Command Code Tests
// ============================================================================

#[test]
fn test_tpm2_command_codes_unique() {
    use tpm2_commands::*;

    assert_ne!(TPM_CC_STARTUP, TPM_CC_SELF_TEST);
    assert_ne!(TPM_CC_GET_CAPABILITY, TPM_CC_PCR_READ);
    assert_ne!(TPM_CC_PCR_EXTEND, TPM_CC_GET_RANDOM);
}

#[test]
fn test_tpm2_command_code_values() {
    use tpm2_commands::*;

    assert_eq!(TPM_CC_STARTUP, 0x00000144);
    assert_eq!(TPM_CC_SELF_TEST, 0x00000143);
    assert_eq!(TPM_CC_GET_CAPABILITY, 0x0000017A);
    assert_eq!(TPM_CC_PCR_READ, 0x0000017E);
    assert_eq!(TPM_CC_PCR_EXTEND, 0x00000182);
    assert_eq!(TPM_CC_GET_RANDOM, 0x0000017B);
}

#[test]
fn test_tpm2_startup_types() {
    use tpm2_commands::*;

    assert_eq!(TPM_SU_CLEAR, 0x0000);
    assert_eq!(TPM_SU_STATE, 0x0001);
    assert_ne!(TPM_SU_CLEAR, TPM_SU_STATE);
}

#[test]
fn test_tpm2_session_tag() {
    use tpm2_commands::*;

    assert_eq!(TPM_ST_NO_SESSIONS, 0x8001);
}

// ============================================================================
// TPM 2.0 Command Building Tests
// ============================================================================

#[test]
fn test_tpm2_startup_command_format() {
    use tpm2_commands::*;

    let mut cmd = [0u8; 12];
    cmd[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    cmd[2..6].copy_from_slice(&12u32.to_be_bytes());
    cmd[6..10].copy_from_slice(&TPM_CC_STARTUP.to_be_bytes());
    cmd[10..12].copy_from_slice(&TPM_SU_CLEAR.to_be_bytes());

    // Verify tag
    let tag = u16::from_be_bytes([cmd[0], cmd[1]]);
    assert_eq!(tag, TPM_ST_NO_SESSIONS);

    // Verify size
    let size = u32::from_be_bytes([cmd[2], cmd[3], cmd[4], cmd[5]]);
    assert_eq!(size, 12);

    // Verify command code
    let code = u32::from_be_bytes([cmd[6], cmd[7], cmd[8], cmd[9]]);
    assert_eq!(code, TPM_CC_STARTUP);

    // Verify startup type
    let startup_type = u16::from_be_bytes([cmd[10], cmd[11]]);
    assert_eq!(startup_type, TPM_SU_CLEAR);
}

#[test]
fn test_tpm2_response_parsing() {
    // Simulate TPM2 response
    let mut response = [0u8; 10];
    response[0..2].copy_from_slice(&0x8001u16.to_be_bytes()); // tag
    response[2..6].copy_from_slice(&10u32.to_be_bytes()); // size
    response[6..10].copy_from_slice(&0u32.to_be_bytes()); // response code (success)

    // Parse response
    let tag = u16::from_be_bytes([response[0], response[1]]);
    let size = u32::from_be_bytes([response[2], response[3], response[4], response[5]]);
    let response_code = u32::from_be_bytes([response[6], response[7], response[8], response[9]]);

    assert_eq!(tag, 0x8001);
    assert_eq!(size, 10);
    assert_eq!(response_code, 0); // Success
}

// ============================================================================
// Secure Boot Variable Name Tests
// ============================================================================

#[test]
fn test_secure_boot_variable_names() {
    // Verify variable names are null-terminated UCS-2
    assert_eq!(IMAGE_SECURITY_DATABASE_VARIABLE.last(), Some(&0x0000));
    assert_eq!(PLATFORM_KEY_VARIABLE.last(), Some(&0x0000));
    assert_eq!(KEY_EXCHANGE_KEY_VARIABLE.last(), Some(&0x0000));
    assert_eq!(SECURE_BOOT_MODE_VARIABLE.last(), Some(&0x0000));
}

#[test]
fn test_secure_boot_db_variable_content() {
    // "db\0" in UCS-2
    assert_eq!(IMAGE_SECURITY_DATABASE_VARIABLE, &[0x0064, 0x0062, 0x0000]);
}

#[test]
fn test_secure_boot_dbx_variable_content() {
    // "dbx\0" in UCS-2
    assert_eq!(
        IMAGE_SECURITY_DATABASE1_VARIABLE,
        &[0x0064, 0x0062, 0x0078, 0x0000]
    );
}

#[test]
fn test_secure_boot_pk_variable_content() {
    // "PK\0" in UCS-2
    assert_eq!(PLATFORM_KEY_VARIABLE, &[0x0050, 0x004B, 0x0000]);
}

#[test]
fn test_secure_boot_kek_variable_content() {
    // "KEK\0" in UCS-2
    assert_eq!(KEY_EXCHANGE_KEY_VARIABLE, &[0x004B, 0x0045, 0x004B, 0x0000]);
}

// ============================================================================
// Structure Alignment Tests
// ============================================================================

#[test]
fn test_security_structure_alignments() {
    assert_eq!(
        core::mem::align_of::<SignatureData>(),
        core::mem::align_of::<Guid>()
    );
    assert_eq!(
        core::mem::align_of::<SignatureList>(),
        core::mem::align_of::<Guid>()
    );
    assert_eq!(core::mem::align_of::<HashOutput>(), 1); // Union of arrays
}

#[test]
fn test_tpm2_structure_alignments() {
    // Packed structures should have alignment of 1
    assert_eq!(core::mem::align_of::<Tpm2CommandHeader>(), 1);
    assert_eq!(core::mem::align_of::<Tpm2ResponseHeader>(), 1);
}

// ============================================================================
// Hash Size Tests
// ============================================================================

#[test]
fn test_hash_sizes_match_algorithms() {
    // SHA-1: 160 bits = 20 bytes
    let sha1_size = 20;
    // SHA-224: 224 bits = 28 bytes
    let sha224_size = 28;
    // SHA-256: 256 bits = 32 bytes
    let sha256_size = 32;
    // SHA-384: 384 bits = 48 bytes
    let sha384_size = 48;
    // SHA-512: 512 bits = 64 bytes
    let sha512_size = 64;

    let output: HashOutput = unsafe { core::mem::zeroed() };
    unsafe {
        assert_eq!(output.sha1_hash.len(), sha1_size);
        assert_eq!(output.sha224_hash.len(), sha224_size);
        assert_eq!(output.sha256_hash.len(), sha256_size);
        assert_eq!(output.sha384_hash.len(), sha384_size);
        assert_eq!(output.sha512_hash.len(), sha512_size);
    }
}

// ============================================================================
// Signature List Calculation Tests
// ============================================================================

#[test]
fn test_signature_list_capacity_calculation() {
    let sig_list = SignatureList {
        signature_type: CERT_SHA256_GUID,
        signature_list_size: 1024,
        signature_header_size: 0,
        signature_size: 48, // GUID (16) + SHA256 (32)
    };

    // Calculate how many signatures fit
    let header_size = core::mem::size_of::<SignatureList>();
    let data_size = sig_list.signature_list_size as usize - header_size;
    let sig_count = data_size / sig_list.signature_size as usize;

    assert_eq!(header_size, 28);
    assert_eq!(data_size, 996);
    assert_eq!(sig_count, 20); // 996 / 48 = 20 signatures
}

#[test]
fn test_signature_list_with_header() {
    let sig_list = SignatureList {
        signature_type: CERT_X509_GUID,
        signature_list_size: 2048,
        signature_header_size: 64, // Optional header
        signature_size: 256,       // Variable X.509 cert
    };

    let list_header_size = core::mem::size_of::<SignatureList>();
    let sig_header_size = sig_list.signature_header_size as usize;
    let data_size = sig_list.signature_list_size as usize - list_header_size - sig_header_size;
    let sig_count = data_size / sig_list.signature_size as usize;

    assert_eq!(list_header_size, 28);
    assert_eq!(data_size, 1956); // 2048 - 28 - 64
    assert_eq!(sig_count, 7); // 1956 / 256
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_zero_size_signature_list() {
    let sig_list = SignatureList {
        signature_type: CERT_SHA256_GUID,
        signature_list_size: 0,
        signature_header_size: 0,
        signature_size: 0,
    };

    assert_eq!(sig_list.signature_list_size, 0);
}

#[test]
fn test_minimal_signature_list() {
    let sig_list = SignatureList {
        signature_type: CERT_SHA256_GUID,
        signature_list_size: 28, // Just the header
        signature_header_size: 0,
        signature_size: 0,
    };

    assert_eq!(
        sig_list.signature_list_size,
        core::mem::size_of::<SignatureList>() as u32
    );
}

// ============================================================================
// TPM PCR Tests
// ============================================================================

#[test]
fn test_pcr_count() {
    // TPM 2.0 typically has 24 PCRs (0-23)
    const TPM2_PCR_COUNT: u8 = 24;
    assert_eq!(TPM2_PCR_COUNT, 24);
}

#[test]
fn test_pcr_sha256_size() {
    // PCR values in SHA-256 mode are 32 bytes
    const PCR_SHA256_SIZE: usize = 32;
    assert_eq!(PCR_SHA256_SIZE, 32);
}

#[test]
fn test_pcr_index_calculation() {
    // Test PCR selection bit calculation
    let pcr_index = 7u32;
    let byte_index = (pcr_index / 8) as u8;
    let bit_index = (pcr_index % 8) as u8;

    assert_eq!(byte_index, 0);
    assert_eq!(bit_index, 7);

    let pcr_index = 15u32;
    let byte_index = (pcr_index / 8) as u8;
    let bit_index = (pcr_index % 8) as u8;

    assert_eq!(byte_index, 1);
    assert_eq!(bit_index, 7);
}

// ============================================================================
// Protocol Revision Tests
// ============================================================================

#[test]
fn test_typical_protocol_revisions() {
    // Common protocol revision format: major.minor
    const REVISION_1_0: u64 = 0x00010000;
    const REVISION_2_0: u64 = 0x00020000;

    assert_eq!(REVISION_1_0, 0x00010000);
    assert_eq!(REVISION_2_0, 0x00020000);
    assert_ne!(REVISION_1_0, REVISION_2_0);
}

// ============================================================================
// Realistic Scenario Tests
// ============================================================================

#[test]
fn test_realistic_signature_db_entry() {
    // Simulate a signature database entry for SHA256 hash
    let sig_list = SignatureList {
        signature_type: CERT_SHA256_GUID,
        signature_list_size: 76, // 28 (header) + 48 (one signature)
        signature_header_size: 0,
        signature_size: 48, // 16 (GUID) + 32 (SHA256)
    };

    let header_size = core::mem::size_of::<SignatureList>();
    let data_size = sig_list.signature_list_size as usize - header_size;
    let sig_count = data_size / sig_list.signature_size as usize;

    assert_eq!(sig_count, 1); // Exactly one signature
    assert_eq!(data_size, 48);
}

#[test]
fn test_realistic_x509_cert_entry() {
    // Simulate X.509 certificate in db
    let cert_size = 1024; // Typical cert size
    let sig_list = SignatureList {
        signature_type: CERT_X509_GUID,
        signature_list_size: (28 + 16 + cert_size) as u32,
        signature_header_size: 0,
        signature_size: (16 + cert_size) as u32,
    };

    let header_size = core::mem::size_of::<SignatureList>();
    let data_size = sig_list.signature_list_size as usize - header_size;
    let sig_count = data_size / sig_list.signature_size as usize;

    assert_eq!(sig_count, 1);
}

#[test]
fn test_realistic_tpm_pcr_extend() {
    use tpm2_commands::*;

    // Simulate PCR extend command
    let pcr_index = 7u32;
    let digest = [0xAAu8; 32]; // SHA256 digest

    // Command would be: tag, size, command_code, pcr_index, digest_count, hash_alg, digest
    let expected_size = 10 + 4 + 4 + 2 + 32; // Simplified

    assert_eq!(digest.len(), 32);
    assert!(expected_size > 0);
}

#[test]
fn test_multiple_hash_algorithms() {
    // Test that different hash algorithms can coexist
    let sha256_guid = HASH_ALGORITHM_SHA256_GUID;
    let sha384_guid = HASH_ALGORITHM_SHA384_GUID;
    let sha512_guid = HASH_ALGORITHM_SHA512_GUID;

    assert_ne!(sha256_guid, sha384_guid);
    assert_ne!(sha384_guid, sha512_guid);
    assert_ne!(sha256_guid, sha512_guid);

    // Verify sizes
    let output: HashOutput = unsafe { core::mem::zeroed() };
    unsafe {
        assert_eq!(output.sha256_hash.len(), 32);
        assert_eq!(output.sha384_hash.len(), 48);
        assert_eq!(output.sha512_hash.len(), 64);
    }
}
