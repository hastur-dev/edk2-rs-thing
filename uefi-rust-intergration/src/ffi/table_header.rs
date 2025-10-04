// SPDX-License-Identifier: BSD-2-Clause-Patent
//! EFI_TABLE_HEADER definition

use super::types::*;

/// EFI_TABLE_HEADER
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TableHeader {
    pub signature: Uint64,
    pub revision: Uint32,
    pub header_size: Uint32,
    pub crc32: Uint32,
    pub reserved: Uint32,
}

impl TableHeader {
    /// Verify the table header signature
    pub fn verify_signature(&self, expected: u64) -> bool {
        self.signature == expected
    }
}
