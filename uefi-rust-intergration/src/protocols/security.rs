// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Security Protocols - Secure Boot and Authentication

use crate::ffi::*;

/// EFI_SECURITY_ARCH_PROTOCOL_GUID
pub const SECURITY_ARCH_PROTOCOL_GUID: Guid = Guid::new(
    0xA46423E3,
    0x4617,
    0x49f1,
    [0xB9, 0xFF, 0xD1, 0xBF, 0xA9, 0x11, 0x58, 0x39],
);

/// EFI_SECURITY2_ARCH_PROTOCOL_GUID
pub const SECURITY2_ARCH_PROTOCOL_GUID: Guid = Guid::new(
    0x94ab2f58,
    0x1438,
    0x4ef1,
    [0x91, 0x52, 0x18, 0x94, 0x1a, 0x3a, 0x0e, 0x68],
);

/// EFI_SECURITY2_ARCH_PROTOCOL
#[repr(C)]
pub struct Security2ArchProtocol {
    pub file_authentication: unsafe extern "efiapi" fn(
        this: *mut Security2ArchProtocol,
        file: *const core::ffi::c_void,
        file_buffer: *const core::ffi::c_void,
        file_size: Uintn,
        boot_policy: Boolean,
    ) -> Status,
}

/// EFI_HASH_ALGORITHM_SHA1_GUID
pub const HASH_ALGORITHM_SHA1_GUID: Guid = Guid::new(
    0x2ae9d80f,
    0x3fb2,
    0x4095,
    [0xb7, 0xb1, 0xe9, 0x31, 0x57, 0xb9, 0x46, 0xb6],
);

/// EFI_HASH_ALGORITHM_SHA256_GUID
pub const HASH_ALGORITHM_SHA256_GUID: Guid = Guid::new(
    0x51aa59de,
    0xfdf2,
    0x4ea3,
    [0xbc, 0x63, 0x87, 0x5f, 0xb7, 0x84, 0x2e, 0xe9],
);

/// EFI_HASH_ALGORITHM_SHA384_GUID
pub const HASH_ALGORITHM_SHA384_GUID: Guid = Guid::new(
    0xefa96432,
    0xde33,
    0x4dd2,
    [0xae, 0xe6, 0x32, 0x8c, 0x33, 0xdf, 0x77, 0x7a],
);

/// EFI_HASH_ALGORITHM_SHA512_GUID
pub const HASH_ALGORITHM_SHA512_GUID: Guid = Guid::new(
    0xcaa4381e,
    0x750c,
    0x4770,
    [0xb8, 0x70, 0x7a, 0x23, 0xb4, 0xe4, 0x21, 0x30],
);

/// EFI_HASH_PROTOCOL_GUID
pub const HASH_PROTOCOL_GUID: Guid = Guid::new(
    0xC5184932,
    0xDBA5,
    0x46DB,
    [0xA5, 0xBA, 0xCC, 0x0B, 0xDA, 0x9C, 0x14, 0x35],
);

/// EFI_HASH_OUTPUT
#[repr(C)]
pub union HashOutput {
    pub sha1_hash: [u8; 20],
    pub sha224_hash: [u8; 28],
    pub sha256_hash: [u8; 32],
    pub sha384_hash: [u8; 48],
    pub sha512_hash: [u8; 64],
}

/// EFI_HASH_PROTOCOL
#[repr(C)]
pub struct HashProtocol {
    pub get_hash_size: unsafe extern "efiapi" fn(
        this: *mut HashProtocol,
        hash_algorithm: *const Guid,
        hash_size: *mut Uintn,
    ) -> Status,
    pub hash: unsafe extern "efiapi" fn(
        this: *mut HashProtocol,
        hash_algorithm: *const Guid,
        extend: Boolean,
        message: *const Uint8,
        message_size: Uint64,
        hash: *mut HashOutput,
    ) -> Status,
}

impl HashProtocol {
    /// Get the size of a hash
    pub unsafe fn get_hash_size(&mut self, algorithm: &Guid) -> Result<usize, Status> {
        let mut size = 0;
        let status = (self.get_hash_size)(self, algorithm as *const _, &mut size);

        if status == EFI_SUCCESS {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Compute a hash
    pub unsafe fn hash(
        &mut self,
        algorithm: &Guid,
        message: &[u8],
        extend: bool,
    ) -> Result<HashOutput, Status> {
        let mut hash_output = core::mem::zeroed();

        let status = (self.hash)(
            self,
            algorithm as *const _,
            extend as Boolean,
            message.as_ptr(),
            message.len() as u64,
            &mut hash_output,
        );

        if status == EFI_SUCCESS {
            Ok(hash_output)
        } else {
            Err(status)
        }
    }
}

/// EFI_PKCS7_VERIFY_PROTOCOL_GUID
pub const PKCS7_VERIFY_PROTOCOL_GUID: Guid = Guid::new(
    0x47889fb2,
    0xd671,
    0x4fab,
    [0xa0, 0xca, 0xdf, 0x0e, 0x44, 0xdf, 0x70, 0xd6],
);

/// EFI_PKCS7_VERIFY_PROTOCOL
#[repr(C)]
pub struct Pkcs7VerifyProtocol {
    pub verify_buffer: unsafe extern "efiapi" fn(
        this: *mut Pkcs7VerifyProtocol,
        p7_data: *const core::ffi::c_void,
        p7_length: Uintn,
        trusted_cert: *const core::ffi::c_void,
        cert_length: Uintn,
        image_hash: *const core::ffi::c_void,
        image_hash_size: Uintn,
    ) -> Status,
    pub verify_signature: unsafe extern "efiapi" fn(
        this: *mut Pkcs7VerifyProtocol,
        p7_data: *const core::ffi::c_void,
        p7_length: Uintn,
        trusted_cert: *const core::ffi::c_void,
        trusted_cert_length: Uintn,
        in_data: *const core::ffi::c_void,
        data_length: Uintn,
    ) -> Status,
}

impl Pkcs7VerifyProtocol {
    /// Verify a PKCS7 signature on a hash
    pub unsafe fn verify_buffer(
        &mut self,
        p7_data: &[u8],
        trusted_cert: &[u8],
        image_hash: &[u8],
    ) -> Status {
        (self.verify_buffer)(
            self,
            p7_data.as_ptr() as *const _,
            p7_data.len(),
            trusted_cert.as_ptr() as *const _,
            trusted_cert.len(),
            image_hash.as_ptr() as *const _,
            image_hash.len(),
        )
    }

    /// Verify a PKCS7 signature on data
    pub unsafe fn verify_signature(
        &mut self,
        p7_data: &[u8],
        trusted_cert: &[u8],
        data: &[u8],
    ) -> Status {
        (self.verify_signature)(
            self,
            p7_data.as_ptr() as *const _,
            p7_data.len(),
            trusted_cert.as_ptr() as *const _,
            trusted_cert.len(),
            data.as_ptr() as *const _,
            data.len(),
        )
    }
}

/// EFI_IMAGE_SECURITY_DATABASE Variable Name
pub const IMAGE_SECURITY_DATABASE_VARIABLE: &[u16] = &[
    0x0064, 0x0062, 0x0000  // "db\0"
];

/// EFI_IMAGE_SECURITY_DATABASE1 Variable Name
pub const IMAGE_SECURITY_DATABASE1_VARIABLE: &[u16] = &[
    0x0064, 0x0062, 0x0078, 0x0000  // "dbx\0"
];

/// EFI_IMAGE_SECURITY_DATABASE2 Variable Name
pub const IMAGE_SECURITY_DATABASE2_VARIABLE: &[u16] = &[
    0x0064, 0x0062, 0x0074, 0x0000  // "dbt\0"
];

/// Platform Key Variable Name
pub const PLATFORM_KEY_VARIABLE: &[u16] = &[
    0x0050, 0x004B, 0x0000  // "PK\0"
];

/// Key Exchange Key Variable Name
pub const KEY_EXCHANGE_KEY_VARIABLE: &[u16] = &[
    0x004B, 0x0045, 0x004B, 0x0000  // "KEK\0"
];

/// Secure Boot Mode Variable Name
pub const SECURE_BOOT_MODE_VARIABLE: &[u16] = &[
    0x0053, 0x0065, 0x0063, 0x0075, 0x0072, 0x0065, 0x0042, 0x006F,
    0x006F, 0x0074, 0x0000  // "SecureBoot\0"
];

/// EFI_SIGNATURE_DATA
#[repr(C)]
pub struct SignatureData {
    pub signature_owner: Guid,
    // Followed by signature data
}

/// EFI_SIGNATURE_LIST
#[repr(C)]
pub struct SignatureList {
    pub signature_type: Guid,
    pub signature_list_size: Uint32,
    pub signature_header_size: Uint32,
    pub signature_size: Uint32,
    // Followed by signature header and data
}

/// Certificate Type GUIDs
pub const CERT_SHA256_GUID: Guid = Guid::new(
    0xc1c41626,
    0x504c,
    0x4092,
    [0xac, 0xa9, 0x41, 0xf9, 0x36, 0x93, 0x43, 0x28],
);

pub const CERT_RSA2048_GUID: Guid = Guid::new(
    0x3c5766e8,
    0x269c,
    0x4e34,
    [0xaa, 0x14, 0xed, 0x77, 0x6e, 0x85, 0xb3, 0xb6],
);

pub const CERT_X509_GUID: Guid = Guid::new(
    0xa5c059a1,
    0x94e4,
    0x4aa7,
    [0x87, 0xb5, 0xab, 0x15, 0x5c, 0x2b, 0xf0, 0x72],
);

// ============================================================================
// TPM 2.0 Protocol
// ============================================================================

/// EFI_TPM2_PROTOCOL_GUID
pub const TPM2_PROTOCOL_GUID: Guid = Guid::new(
    0x607f766c,
    0x7455,
    0x42be,
    [0x93, 0x0b, 0xe4, 0xd7, 0x6d, 0xb2, 0x72, 0x0f],
);

/// TPM2 Submit Command
pub type Tpm2SubmitCommand = unsafe extern "efiapi" fn(
    this: *mut Tpm2Protocol,
    input_parameter_block_size: Uint32,
    input_parameter_block: *const Uint8,
    output_parameter_block_size: Uint32,
    output_parameter_block: *mut Uint8,
) -> Status;

/// TPM2 Get Active PCR Banks
pub type Tpm2GetActivePcrBanks = unsafe extern "efiapi" fn(
    this: *mut Tpm2Protocol,
    active_pcr_banks: *mut Uint32,
) -> Status;

/// EFI_TPM2_PROTOCOL
#[repr(C)]
pub struct Tpm2Protocol {
    pub get_capability: unsafe extern "efiapi" fn(
        this: *mut Tpm2Protocol,
        out: *mut Tpm2BootServiceCapability,
    ) -> Status,
    pub get_event_log: unsafe extern "efiapi" fn(
        this: *mut Tpm2Protocol,
        event_log_format: *mut Uint32,
        event_log_location: *mut *mut core::ffi::c_void,
        event_log_last_entry: *mut *mut core::ffi::c_void,
        event_log_truncated: *mut Boolean,
    ) -> Status,
    pub hash_log_extend_event: unsafe extern "efiapi" fn(
        this: *mut Tpm2Protocol,
        flags: Uint64,
        data_to_hash: *mut core::ffi::c_void,
        data_to_hash_len: Uint64,
        event: *mut core::ffi::c_void,
    ) -> Status,
    pub submit_command: Tpm2SubmitCommand,
    pub get_active_pcr_banks: Tpm2GetActivePcrBanks,
}

/// TPM2 Boot Service Capability
#[repr(C)]
pub struct Tpm2BootServiceCapability {
    pub size: Uint8,
    pub structure_version: Tpm2Version,
    pub protocol_version: Tpm2Version,
    pub hash_algorithm_bitmap: Uint32,
    pub supported_event_logs: Uint32,
    pub tpm_present_flag: Boolean,
    pub max_command_size: Uint16,
    pub max_response_size: Uint16,
    pub manufacturer_id: Uint32,
    pub number_of_pcr_banks: Uint32,
    pub active_pcr_banks: Uint32,
}

/// TPM2 Version
#[repr(C)]
pub struct Tpm2Version {
    pub major: Uint8,
    pub minor: Uint8,
}

/// TPM2 Command Header
#[repr(C, packed)]
pub struct Tpm2CommandHeader {
    pub tag: Uint16,
    pub param_size: Uint32,
    pub command_code: Uint32,
}

/// TPM2 Response Header
#[repr(C, packed)]
pub struct Tpm2ResponseHeader {
    pub tag: Uint16,
    pub param_size: Uint32,
    pub response_code: Uint32,
}

/// TPM2 Command Codes
pub mod tpm2_commands {
    pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
    pub const TPM_CC_STARTUP: u32 = 0x00000144;
    pub const TPM_CC_SELF_TEST: u32 = 0x00000143;
    pub const TPM_CC_GET_CAPABILITY: u32 = 0x0000017A;
    pub const TPM_CC_PCR_READ: u32 = 0x0000017E;
    pub const TPM_CC_PCR_EXTEND: u32 = 0x00000182;
    pub const TPM_CC_GET_RANDOM: u32 = 0x0000017B;

    pub const TPM_SU_CLEAR: u16 = 0x0000;
    pub const TPM_SU_STATE: u16 = 0x0001;
}

// ============================================================================
// Secure Boot Helper Module
// ============================================================================

/// Secure Boot helper functions
pub mod secure_boot {
    use super::*;
    use crate::runtime_services::Variable;
    use crate::runtime_services::EFI_IMAGE_SECURITY_DATABASE_GUID;

    /// Check if Secure Boot is enabled
    pub unsafe fn is_secure_boot_enabled(vars: &Variable) -> bool {
        let mut buffer = [0u8; 1];
        if let Ok((_, size)) = vars.get(
            SECURE_BOOT_MODE_VARIABLE.as_ptr(),
            &crate::runtime_services::EFI_GLOBAL_VARIABLE_GUID,
            &mut buffer,
        ) {
            size > 0 && buffer[0] == 1
        } else {
            false
        }
    }

    /// Check if system is in Setup Mode
    pub unsafe fn is_setup_mode(vars: &Variable) -> bool {
        let setup_mode_var: &[u16] = &[0x0053, 0x0065, 0x0074, 0x0075, 0x0070, 0x004D, 0x006F, 0x0064, 0x0065, 0x0000]; // "SetupMode\0"
        let mut buffer = [0u8; 1];
        if let Ok((_, size)) = vars.get(
            setup_mode_var.as_ptr(),
            &crate::runtime_services::EFI_GLOBAL_VARIABLE_GUID,
            &mut buffer,
        ) {
            size > 0 && buffer[0] == 1
        } else {
            false
        }
    }

    /// Get Platform Key (PK)
    pub unsafe fn get_platform_key(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            PLATFORM_KEY_VARIABLE.as_ptr(),
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
    }

    /// Get Key Exchange Keys (KEK)
    pub unsafe fn get_kek(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            KEY_EXCHANGE_KEY_VARIABLE.as_ptr(),
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
    }

    /// Get signature database (db)
    pub unsafe fn get_signature_database(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            IMAGE_SECURITY_DATABASE_VARIABLE.as_ptr(),
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
    }

    /// Get forbidden signature database (dbx)
    pub unsafe fn get_forbidden_database(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            IMAGE_SECURITY_DATABASE1_VARIABLE.as_ptr(),
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
    }

    /// Get timestamp signature database (dbt)
    pub unsafe fn get_timestamp_database(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            IMAGE_SECURITY_DATABASE2_VARIABLE.as_ptr(),
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
    }

    /// Check if a hash exists in signature database
    pub unsafe fn is_hash_in_database(
        vars: &Variable,
        hash: &[u8],
        database_name: *const u16,
    ) -> bool {
        let mut buffer = [0u8; 4096];
        if let Ok((_, size)) = vars.get(
            database_name,
            &EFI_IMAGE_SECURITY_DATABASE_GUID,
            &mut buffer,
        ) {
            let data = &buffer[..size];
            for sig_list in SignatureListIter::new(data) {
                if sig_list.signature_type == CERT_SHA256_GUID && hash.len() == 32 {
                    let sig_count = (sig_list.signature_list_size as usize
                        - core::mem::size_of::<SignatureList>()
                        - sig_list.signature_header_size as usize)
                        / sig_list.signature_size as usize;

                    let sig_data_offset = core::mem::size_of::<SignatureList>()
                        + sig_list.signature_header_size as usize;

                    for i in 0..sig_count {
                        let sig_offset = sig_data_offset + (i * sig_list.signature_size as usize);
                        let sig_ptr = (sig_list as *const _ as *const u8).add(sig_offset + core::mem::size_of::<Guid>());
                        let sig_hash = core::slice::from_raw_parts(sig_ptr, 32);

                        if sig_hash == hash {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Enumerate signature database
    pub struct SignatureListIter<'a> {
        data: &'a [u8],
        offset: usize,
    }

    impl<'a> SignatureListIter<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            SignatureListIter { data, offset: 0 }
        }
    }

    impl<'a> Iterator for SignatureListIter<'a> {
        type Item = &'a SignatureList;

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset + core::mem::size_of::<SignatureList>() > self.data.len() {
                return None;
            }

            let sig_list = unsafe {
                &*((self.data.as_ptr() as usize + self.offset) as *const SignatureList)
            };

            let list_size = sig_list.signature_list_size as usize;
            if self.offset + list_size > self.data.len() || list_size == 0 {
                return None;
            }

            self.offset += list_size;
            Some(sig_list)
        }
    }
}

// ============================================================================
// Safe Wrappers
// ============================================================================

/// Safe wrapper for PKCS7 Verify Protocol
pub struct SafePkcs7Verify<'a> {
    protocol: &'a mut Pkcs7VerifyProtocol,
}

impl<'a> SafePkcs7Verify<'a> {
    /// Create a new safe wrapper
    pub fn new(protocol: &'a mut Pkcs7VerifyProtocol) -> Self {
        Self { protocol }
    }

    /// Verify a PKCS7 signature on a hash
    pub fn verify_buffer(
        &mut self,
        p7_data: &[u8],
        trusted_cert: &[u8],
        image_hash: &[u8],
    ) -> Result<(), Status> {
        let status = unsafe {
            self.protocol.verify_buffer(p7_data, trusted_cert, image_hash)
        };
        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Verify a PKCS7 signature on data
    pub fn verify_signature(
        &mut self,
        p7_data: &[u8],
        trusted_cert: &[u8],
        data: &[u8],
    ) -> Result<(), Status> {
        let status = unsafe {
            self.protocol.verify_signature(p7_data, trusted_cert, data)
        };
        if status == EFI_SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper for Hash Protocol
pub struct SafeHashProtocol<'a> {
    protocol: &'a mut HashProtocol,
}

impl<'a> SafeHashProtocol<'a> {
    /// Create a new safe wrapper
    pub fn new(protocol: &'a mut HashProtocol) -> Self {
        Self { protocol }
    }

    /// Get hash size for an algorithm
    pub fn get_hash_size(&mut self, algorithm: &Guid) -> Result<usize, Status> {
        unsafe { self.protocol.get_hash_size(algorithm) }
    }

    /// Compute a hash
    pub fn hash(&mut self, algorithm: &Guid, message: &[u8]) -> Result<HashOutput, Status> {
        unsafe { self.protocol.hash(algorithm, message, false) }
    }

    /// Compute SHA256 hash
    pub fn sha256(&mut self, message: &[u8]) -> Result<[u8; 32], Status> {
        let output = self.hash(&HASH_ALGORITHM_SHA256_GUID, message)?;
        Ok(unsafe { output.sha256_hash })
    }

    /// Compute SHA384 hash
    pub fn sha384(&mut self, message: &[u8]) -> Result<[u8; 48], Status> {
        let output = self.hash(&HASH_ALGORITHM_SHA384_GUID, message)?;
        Ok(unsafe { output.sha384_hash })
    }

    /// Compute SHA512 hash
    pub fn sha512(&mut self, message: &[u8]) -> Result<[u8; 64], Status> {
        let output = self.hash(&HASH_ALGORITHM_SHA512_GUID, message)?;
        Ok(unsafe { output.sha512_hash })
    }
}

/// Safe wrapper for TPM2 Protocol
pub struct SafeTpm2<'a> {
    protocol: &'a mut Tpm2Protocol,
}

impl<'a> SafeTpm2<'a> {
    /// Create a new safe wrapper
    pub fn new(protocol: &'a mut Tpm2Protocol) -> Self {
        Self { protocol }
    }

    /// Get TPM2 capabilities
    pub fn get_capability(&mut self) -> Result<Tpm2BootServiceCapability, Status> {
        let mut cap = unsafe { core::mem::zeroed() };
        let status = unsafe { (self.protocol.get_capability)(self.protocol, &mut cap) };
        if status == EFI_SUCCESS {
            Ok(cap)
        } else {
            Err(status)
        }
    }

    /// Submit a TPM2 command
    pub fn submit_command(
        &mut self,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<usize, Status> {
        let status = unsafe {
            (self.protocol.submit_command)(
                self.protocol,
                input.len() as u32,
                input.as_ptr(),
                output.len() as u32,
                output.as_mut_ptr(),
            )
        };

        if status == EFI_SUCCESS {
            // Parse response size from TPM2 response header
            if output.len() >= 10 {
                let size = u32::from_be_bytes([output[2], output[3], output[4], output[5]]);
                Ok(size as usize)
            } else {
                Ok(output.len())
            }
        } else {
            Err(status)
        }
    }

    /// Get active PCR banks
    pub fn get_active_pcr_banks(&mut self) -> Result<u32, Status> {
        let mut banks = 0;
        let status = unsafe {
            (self.protocol.get_active_pcr_banks)(self.protocol, &mut banks)
        };
        if status == EFI_SUCCESS {
            Ok(banks)
        } else {
            Err(status)
        }
    }

    /// Send TPM2 Startup command
    pub fn startup(&mut self, startup_type: u16) -> Result<(), Status> {
        let mut cmd = [0u8; 12];
        // Build command
        cmd[0..2].copy_from_slice(&tpm2_commands::TPM_ST_NO_SESSIONS.to_be_bytes());
        cmd[2..6].copy_from_slice(&12u32.to_be_bytes()); // Command size
        cmd[6..10].copy_from_slice(&tpm2_commands::TPM_CC_STARTUP.to_be_bytes());
        cmd[10..12].copy_from_slice(&startup_type.to_be_bytes());

        let mut response = [0u8; 10];
        self.submit_command(&cmd, &mut response)?;

        // Check response code
        let response_code = u32::from_be_bytes([response[6], response[7], response[8], response[9]]);
        if response_code == 0 {
            Ok(())
        } else {
            Err(EFI_DEVICE_ERROR)
        }
    }

    /// Send TPM2 PCR Read command (basic version)
    pub fn pcr_read(&mut self, pcr_index: u32) -> Result<[u8; 32], Status> {
        // Simplified PCR read - real implementation would be more complex
        let mut cmd = [0u8; 20];
        cmd[0..2].copy_from_slice(&tpm2_commands::TPM_ST_NO_SESSIONS.to_be_bytes());
        cmd[2..6].copy_from_slice(&20u32.to_be_bytes());
        cmd[6..10].copy_from_slice(&tpm2_commands::TPM_CC_PCR_READ.to_be_bytes());
        cmd[10..14].copy_from_slice(&1u32.to_be_bytes()); // PCR selection count
        cmd[14..16].copy_from_slice(&0x000Bu16.to_be_bytes()); // SHA256
        cmd[16] = 3u8; // Size of select
        cmd[17] = (pcr_index / 8) as u8;
        cmd[18] = 0;
        cmd[19] = 0;

        let mut response = [0u8; 128];
        let size = self.submit_command(&cmd, &mut response)?;

        if size >= 42 {
            let mut pcr_value = [0u8; 32];
            pcr_value.copy_from_slice(&response[10..42]);
            Ok(pcr_value)
        } else {
            Err(EFI_DEVICE_ERROR)
        }
    }
}
