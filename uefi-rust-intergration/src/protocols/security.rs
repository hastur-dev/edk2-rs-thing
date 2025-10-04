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

/// Secure Boot helper functions
pub mod secure_boot {
    use super::*;
    use crate::runtime_services::Variable;

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

    /// Get Platform Key (PK)
    pub unsafe fn get_platform_key(
        vars: &Variable,
        buffer: &mut [u8],
    ) -> Result<(u32, usize), Status> {
        vars.get(
            PLATFORM_KEY_VARIABLE.as_ptr(),
            &crate::tables::acpi::EFI_IMAGE_SECURITY_DATABASE_GUID,
            buffer,
        )
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
            if self.offset + list_size > self.data.len() {
                return None;
            }

            self.offset += list_size;
            Some(sig_list)
        }
    }
}
