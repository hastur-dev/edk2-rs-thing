// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Block I/O Protocol

use crate::ffi::*;

/// EFI_BLOCK_IO_PROTOCOL_GUID
pub const BLOCK_IO_PROTOCOL_GUID: Guid = Guid::new(
    0x964e5b21,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_BLOCK_IO_PROTOCOL_REVISION
pub const EFI_BLOCK_IO_PROTOCOL_REVISION: Uint64 = 0x00010000;
pub const EFI_BLOCK_IO_PROTOCOL_REVISION2: Uint64 = 0x00020001;
pub const EFI_BLOCK_IO_PROTOCOL_REVISION3: Uint64 = 0x0002001F;

/// EFI_BLOCK_IO_MEDIA
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockIoMedia {
    pub media_id: Uint32,
    pub removable_media: Boolean,
    pub media_present: Boolean,
    pub logical_partition: Boolean,
    pub read_only: Boolean,
    pub write_caching: Boolean,
    pub block_size: Uint32,
    pub io_align: Uint32,
    pub last_block: Uint64,
    // Revision 2
    pub lowest_aligned_lba: Uint64,
    pub logical_blocks_per_physical_block: Uint32,
    // Revision 3
    pub optimal_transfer_length_granularity: Uint32,
}

/// EFI_BLOCK_IO_PROTOCOL
#[repr(C)]
pub struct BlockIoProtocol {
    pub revision: Uint64,
    pub media: *mut BlockIoMedia,
    pub reset: unsafe extern "efiapi" fn(
        this: *mut BlockIoProtocol,
        extended_verification: Boolean,
    ) -> Status,
    pub read_blocks: unsafe extern "efiapi" fn(
        this: *mut BlockIoProtocol,
        media_id: Uint32,
        lba: Uint64,
        buffer_size: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write_blocks: unsafe extern "efiapi" fn(
        this: *mut BlockIoProtocol,
        media_id: Uint32,
        lba: Uint64,
        buffer_size: Uintn,
        buffer: *const core::ffi::c_void,
    ) -> Status,
    pub flush_blocks: unsafe extern "efiapi" fn(
        this: *mut BlockIoProtocol,
    ) -> Status,
}

impl BlockIoProtocol {
    /// Reset the block device
    pub unsafe fn reset(&mut self, extended_verification: bool) -> Status {
        (self.reset)(self, extended_verification as Boolean)
    }

    /// Read blocks
    pub unsafe fn read_blocks(
        &mut self,
        media_id: u32,
        lba: u64,
        buffer_size: usize,
        buffer: *mut core::ffi::c_void,
    ) -> Status {
        (self.read_blocks)(self, media_id, lba, buffer_size, buffer)
    }

    /// Write blocks
    pub unsafe fn write_blocks(
        &mut self,
        media_id: u32,
        lba: u64,
        buffer_size: usize,
        buffer: *const core::ffi::c_void,
    ) -> Status {
        (self.write_blocks)(self, media_id, lba, buffer_size, buffer)
    }

    /// Flush blocks
    pub unsafe fn flush_blocks(&mut self) -> Status {
        (self.flush_blocks)(self)
    }

    /// Get media information
    pub unsafe fn media_info(&self) -> Option<&BlockIoMedia> {
        if self.media.is_null() {
            None
        } else {
            Some(&*self.media)
        }
    }
}
