// SPDX-License-Identifier: BSD-2-Clause-Patent
//! EFI_STATUS codes

use super::types::Status;

// Success codes
pub const EFI_SUCCESS: Status = 0;

// Error bit
const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

// Error codes
pub const EFI_LOAD_ERROR: Status = ERROR_BIT | 1;
pub const EFI_INVALID_PARAMETER: Status = ERROR_BIT | 2;
pub const EFI_UNSUPPORTED: Status = ERROR_BIT | 3;
pub const EFI_BAD_BUFFER_SIZE: Status = ERROR_BIT | 4;
pub const EFI_BUFFER_TOO_SMALL: Status = ERROR_BIT | 5;
pub const EFI_NOT_READY: Status = ERROR_BIT | 6;
pub const EFI_DEVICE_ERROR: Status = ERROR_BIT | 7;
pub const EFI_WRITE_PROTECTED: Status = ERROR_BIT | 8;
pub const EFI_OUT_OF_RESOURCES: Status = ERROR_BIT | 9;
pub const EFI_VOLUME_CORRUPTED: Status = ERROR_BIT | 10;
pub const EFI_VOLUME_FULL: Status = ERROR_BIT | 11;
pub const EFI_NO_MEDIA: Status = ERROR_BIT | 12;
pub const EFI_MEDIA_CHANGED: Status = ERROR_BIT | 13;
pub const EFI_NOT_FOUND: Status = ERROR_BIT | 14;
pub const EFI_ACCESS_DENIED: Status = ERROR_BIT | 15;
pub const EFI_NO_RESPONSE: Status = ERROR_BIT | 16;
pub const EFI_NO_MAPPING: Status = ERROR_BIT | 17;
pub const EFI_TIMEOUT: Status = ERROR_BIT | 18;
pub const EFI_NOT_STARTED: Status = ERROR_BIT | 19;
pub const EFI_ALREADY_STARTED: Status = ERROR_BIT | 20;
pub const EFI_ABORTED: Status = ERROR_BIT | 21;
pub const EFI_ICMP_ERROR: Status = ERROR_BIT | 22;
pub const EFI_TFTP_ERROR: Status = ERROR_BIT | 23;
pub const EFI_PROTOCOL_ERROR: Status = ERROR_BIT | 24;
pub const EFI_INCOMPATIBLE_VERSION: Status = ERROR_BIT | 25;
pub const EFI_SECURITY_VIOLATION: Status = ERROR_BIT | 26;
pub const EFI_CRC_ERROR: Status = ERROR_BIT | 27;
pub const EFI_END_OF_MEDIA: Status = ERROR_BIT | 28;
pub const EFI_END_OF_FILE: Status = ERROR_BIT | 31;
pub const EFI_INVALID_LANGUAGE: Status = ERROR_BIT | 32;
pub const EFI_COMPROMISED_DATA: Status = ERROR_BIT | 33;

// Warning codes
pub const EFI_WARN_UNKNOWN_GLYPH: Status = 1;
pub const EFI_WARN_DELETE_FAILURE: Status = 2;
pub const EFI_WARN_WRITE_FAILURE: Status = 3;
pub const EFI_WARN_BUFFER_TOO_SMALL: Status = 4;
pub const EFI_WARN_STALE_DATA: Status = 5;

/// Check if status is an error
pub fn is_error(status: Status) -> bool {
    (status & ERROR_BIT) != 0
}

/// Check if status is success
pub fn is_success(status: Status) -> bool {
    status == EFI_SUCCESS
}
