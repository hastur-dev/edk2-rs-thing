// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Storage Protocols - SCSI, NVMe, Disk I/O, Partitions

use crate::ffi::*;

/// EFI_SCSI_PASS_THRU_PROTOCOL_GUID
pub const SCSI_PASS_THRU_PROTOCOL_GUID: Guid = Guid::new(
    0xa59e8fcf,
    0xbda0,
    0x43bb,
    [0x90, 0xb1, 0xd3, 0x73, 0x2e, 0xca, 0xa8, 0x77],
);

/// EFI_EXT_SCSI_PASS_THRU_PROTOCOL_GUID
pub const EXT_SCSI_PASS_THRU_PROTOCOL_GUID: Guid = Guid::new(
    0x143b7632,
    0xb81b,
    0x4cb7,
    [0xab, 0xd3, 0xb6, 0x25, 0xa5, 0xb9, 0xbf, 0xfe],
);

/// EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID
pub const NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID: Guid = Guid::new(
    0x52c78312,
    0x8edc,
    0x4233,
    [0x98, 0xf2, 0x1a, 0x1a, 0xa5, 0xe3, 0x88, 0xa5],
);

/// EFI_DISK_IO_PROTOCOL_GUID
pub const DISK_IO_PROTOCOL_GUID: Guid = Guid::new(
    0xce345171,
    0xba0b,
    0x11d2,
    [0x8e, 0x4f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// EFI_DISK_IO2_PROTOCOL_GUID
pub const DISK_IO2_PROTOCOL_GUID: Guid = Guid::new(
    0x151c8eae,
    0x7f2c,
    0x472c,
    [0x9e, 0x54, 0x98, 0x28, 0x19, 0x4f, 0x6a, 0x88],
);

/// EFI_PARTITION_INFO_PROTOCOL_GUID
pub const PARTITION_INFO_PROTOCOL_GUID: Guid = Guid::new(
    0x8cf2f62c,
    0xbc9b,
    0x4821,
    [0x80, 0x8d, 0xec, 0x9e, 0xc4, 0x21, 0xa1, 0xa0],
);

/// SCSI Target ID
pub type ScsiTarget = Uint32;

/// SCSI LUN
pub type ScsiLun = Uint64;

/// SCSI Request Packet
#[repr(C)]
pub struct ScsiPassThruRequestPacket {
    pub timeout: Uint64,
    pub in_data_buffer: *mut core::ffi::c_void,
    pub out_data_buffer: *mut core::ffi::c_void,
    pub sense_data: *mut core::ffi::c_void,
    pub cdb: *mut core::ffi::c_void,
    pub in_transfer_length: Uint32,
    pub out_transfer_length: Uint32,
    pub cdb_length: Uint8,
    pub data_direction: Uint8,
    pub host_adapter_status: Uint8,
    pub target_status: Uint8,
    pub sense_data_length: Uint8,
}

/// SCSI Data Direction
pub const SCSI_DATA_IN: Uint8 = 0;
pub const SCSI_DATA_OUT: Uint8 = 1;

/// EFI_SCSI_PASS_THRU_PROTOCOL
#[repr(C)]
pub struct ScsiPassThruProtocol {
    pub mode: *mut ScsiPassThruMode,
    pub pass_thru: unsafe extern "efiapi" fn(
        this: *mut ScsiPassThruProtocol,
        target: ScsiTarget,
        lun: ScsiLun,
        packet: *mut ScsiPassThruRequestPacket,
        event: Event,
    ) -> Status,
    pub get_next_device: unsafe extern "efiapi" fn(
        this: *mut ScsiPassThruProtocol,
        target: *mut ScsiTarget,
        lun: *mut ScsiLun,
    ) -> Status,
    pub build_device_path: unsafe extern "efiapi" fn(
        this: *mut ScsiPassThruProtocol,
        target: ScsiTarget,
        lun: ScsiLun,
        device_path: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub get_target_lun: unsafe extern "efiapi" fn(
        this: *mut ScsiPassThruProtocol,
        device_path: *mut core::ffi::c_void,
        target: *mut ScsiTarget,
        lun: *mut ScsiLun,
    ) -> Status,
    pub reset_channel: unsafe extern "efiapi" fn(this: *mut ScsiPassThruProtocol) -> Status,
    pub reset_target: unsafe extern "efiapi" fn(
        this: *mut ScsiPassThruProtocol,
        target: ScsiTarget,
        lun: ScsiLun,
    ) -> Status,
}

/// SCSI Pass Thru Mode
#[repr(C)]
pub struct ScsiPassThruMode {
    pub adapter_id: Uint32,
    pub attributes: Uint32,
    pub io_align: Uint32,
}

/// Extended SCSI Pass Thru Request Packet
#[repr(C)]
pub struct ExtScsiPassThruRequestPacket {
    pub timeout: Uint64,
    pub in_data_buffer: *mut core::ffi::c_void,
    pub out_data_buffer: *mut core::ffi::c_void,
    pub sense_data: *mut core::ffi::c_void,
    pub cdb: *mut core::ffi::c_void,
    pub in_transfer_length: Uint32,
    pub out_transfer_length: Uint32,
    pub cdb_length: Uint8,
    pub data_direction: Uint8,
    pub host_adapter_status: Uint8,
    pub target_status: Uint8,
    pub sense_data_length: Uint8,
}

/// EFI_EXT_SCSI_PASS_THRU_PROTOCOL
#[repr(C)]
pub struct ExtScsiPassThruProtocol {
    pub mode: *mut ExtScsiPassThruMode,
    pub pass_thru: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        target: *const Uint8,
        lun: Uint64,
        packet: *mut ExtScsiPassThruRequestPacket,
        event: Event,
    ) -> Status,
    pub get_next_target_lun: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        target: *mut *mut Uint8,
        lun: *mut Uint64,
    ) -> Status,
    pub build_device_path: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        target: *const Uint8,
        lun: Uint64,
        device_path: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub get_target_lun: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        device_path: *mut core::ffi::c_void,
        target: *mut *mut Uint8,
        lun: *mut Uint64,
    ) -> Status,
    pub reset_channel: unsafe extern "efiapi" fn(this: *mut ExtScsiPassThruProtocol) -> Status,
    pub reset_target_lun: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        target: *const Uint8,
        lun: Uint64,
    ) -> Status,
    pub get_next_target: unsafe extern "efiapi" fn(
        this: *mut ExtScsiPassThruProtocol,
        target: *mut *mut Uint8,
    ) -> Status,
}

/// Extended SCSI Pass Thru Mode
#[repr(C)]
pub struct ExtScsiPassThruMode {
    pub adapter_id: Uint32,
    pub attributes: Uint32,
    pub io_align: Uint32,
}

/// NVMe Namespace ID
pub type NvmeNamespaceId = Uint32;

/// NVMe Command Packet
#[repr(C)]
pub struct NvmePassThruCommandPacket {
    pub command_timeout: Uint64,
    pub transfer_buffer: *mut core::ffi::c_void,
    pub transfer_length: Uint32,
    pub metadata_buffer: *mut core::ffi::c_void,
    pub metadata_length: Uint32,
    pub queue_type: Uint8,
    pub nvme_cmd: *mut NvmeCommand,
    pub nvme_completion: *mut NvmeCompletion,
}

/// NVMe Command
#[repr(C)]
pub struct NvmeCommand {
    pub cdw0: Uint32,
    pub flags: Uint8,
    pub nsid: Uint32,
    pub cdw2: Uint32,
    pub cdw3: Uint32,
    pub cdw10: Uint32,
    pub cdw11: Uint32,
    pub cdw12: Uint32,
    pub cdw13: Uint32,
    pub cdw14: Uint32,
    pub cdw15: Uint32,
}

/// NVMe Completion
#[repr(C)]
pub struct NvmeCompletion {
    pub dw0: Uint32,
    pub dw1: Uint32,
    pub dw2: Uint32,
    pub dw3: Uint32,
}

/// EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL
#[repr(C)]
pub struct NvmExpressPassThruProtocol {
    pub mode: *mut NvmExpressPassThruMode,
    pub pass_thru: unsafe extern "efiapi" fn(
        this: *mut NvmExpressPassThruProtocol,
        namespace_id: Uint32,
        packet: *mut NvmePassThruCommandPacket,
        event: Event,
    ) -> Status,
    pub get_next_namespace: unsafe extern "efiapi" fn(
        this: *mut NvmExpressPassThruProtocol,
        namespace_id: *mut Uint32,
    ) -> Status,
    pub build_device_path: unsafe extern "efiapi" fn(
        this: *mut NvmExpressPassThruProtocol,
        namespace_id: Uint32,
        device_path: *mut *mut core::ffi::c_void,
    ) -> Status,
    pub get_namespace: unsafe extern "efiapi" fn(
        this: *mut NvmExpressPassThruProtocol,
        device_path: *mut core::ffi::c_void,
        namespace_id: *mut Uint32,
    ) -> Status,
}

/// NVMe Pass Thru Mode
#[repr(C)]
pub struct NvmExpressPassThruMode {
    pub attributes: Uint32,
    pub io_align: Uint32,
    pub nvme_version: Uint32,
}

/// EFI_DISK_IO_PROTOCOL
#[repr(C)]
pub struct DiskIoProtocol {
    pub revision: Uint64,
    pub read_disk: unsafe extern "efiapi" fn(
        this: *mut DiskIoProtocol,
        media_id: Uint32,
        offset: Uint64,
        buffer_size: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write_disk: unsafe extern "efiapi" fn(
        this: *mut DiskIoProtocol,
        media_id: Uint32,
        offset: Uint64,
        buffer_size: Uintn,
        buffer: *const core::ffi::c_void,
    ) -> Status,
}

/// Disk I/O Token
#[repr(C)]
pub struct DiskIo2Token {
    pub event: Event,
    pub transaction_status: Status,
}

/// EFI_DISK_IO2_PROTOCOL
#[repr(C)]
pub struct DiskIo2Protocol {
    pub revision: Uint64,
    pub cancel: unsafe extern "efiapi" fn(this: *mut DiskIo2Protocol) -> Status,
    pub read_disk_ex: unsafe extern "efiapi" fn(
        this: *mut DiskIo2Protocol,
        media_id: Uint32,
        offset: Uint64,
        token: *mut DiskIo2Token,
        buffer_size: Uintn,
        buffer: *mut core::ffi::c_void,
    ) -> Status,
    pub write_disk_ex: unsafe extern "efiapi" fn(
        this: *mut DiskIo2Protocol,
        media_id: Uint32,
        offset: Uint64,
        token: *mut DiskIo2Token,
        buffer_size: Uintn,
        buffer: *const core::ffi::c_void,
    ) -> Status,
    pub flush_disk_ex: unsafe extern "efiapi" fn(
        this: *mut DiskIo2Protocol,
        token: *mut DiskIo2Token,
    ) -> Status,
}

/// Partition Type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PartitionType {
    Other = 0,
    Mbr = 1,
    Gpt = 2,
}

/// MBR Partition Record
#[repr(C)]
pub struct MbrPartitionRecord {
    pub boot_indicator: Uint8,
    pub start_head: Uint8,
    pub start_sector: Uint8,
    pub start_track: Uint8,
    pub os_indicator: Uint8,
    pub end_head: Uint8,
    pub end_sector: Uint8,
    pub end_track: Uint8,
    pub starting_lba: [Uint8; 4],
    pub size_in_lba: [Uint8; 4],
}

/// GPT Partition Entry
#[repr(C)]
pub struct GptPartitionEntry {
    pub partition_type_guid: Guid,
    pub unique_partition_guid: Guid,
    pub starting_lba: Uint64,
    pub ending_lba: Uint64,
    pub attributes: Uint64,
    pub partition_name: [Char16; 36],
}

/// Partition Info
#[repr(C)]
pub struct PartitionInfo {
    pub revision: Uint32,
    pub partition_type: PartitionType,
    pub system: Uint8,
    pub reserved: [Uint8; 7],
    pub info: PartitionInfoUnion,
}

/// Partition Info Union
#[repr(C)]
pub union PartitionInfoUnion {
    pub mbr: core::mem::ManuallyDrop<MbrPartitionRecord>,
    pub gpt: core::mem::ManuallyDrop<GptPartitionEntry>,
}

/// EFI_PARTITION_INFO_PROTOCOL
#[repr(C)]
pub struct PartitionInfoProtocol {
    pub revision: Uint64,
    pub info: PartitionInfo,
}

impl DiskIoProtocol {
    /// Read from disk at byte offset
    pub unsafe fn read_disk(
        &mut self,
        media_id: u32,
        offset: u64,
        buffer: &mut [u8],
    ) -> Status {
        (self.read_disk)(
            self,
            media_id,
            offset,
            buffer.len(),
            buffer.as_mut_ptr() as *mut _,
        )
    }

    /// Write to disk at byte offset
    pub unsafe fn write_disk(
        &mut self,
        media_id: u32,
        offset: u64,
        buffer: &[u8],
    ) -> Status {
        (self.write_disk)(
            self,
            media_id,
            offset,
            buffer.len(),
            buffer.as_ptr() as *const _,
        )
    }
}

impl DiskIo2Protocol {
    /// Read from disk asynchronously
    pub unsafe fn read_disk_ex(
        &mut self,
        media_id: u32,
        offset: u64,
        token: &mut DiskIo2Token,
        buffer: &mut [u8],
    ) -> Status {
        (self.read_disk_ex)(
            self,
            media_id,
            offset,
            token as *mut _,
            buffer.len(),
            buffer.as_mut_ptr() as *mut _,
        )
    }

    /// Write to disk asynchronously
    pub unsafe fn write_disk_ex(
        &mut self,
        media_id: u32,
        offset: u64,
        token: &mut DiskIo2Token,
        buffer: &[u8],
    ) -> Status {
        (self.write_disk_ex)(
            self,
            media_id,
            offset,
            token as *mut _,
            buffer.len(),
            buffer.as_ptr() as *const _,
        )
    }

    /// Flush disk cache
    pub unsafe fn flush_disk_ex(&mut self, token: &mut DiskIo2Token) -> Status {
        (self.flush_disk_ex)(self, token as *mut _)
    }
}

impl ScsiPassThruProtocol {
    /// Send SCSI command
    pub unsafe fn pass_thru(
        &mut self,
        target: u32,
        lun: u64,
        packet: &mut ScsiPassThruRequestPacket,
    ) -> Status {
        (self.pass_thru)(self, target, lun, packet as *mut _, core::ptr::null_mut())
    }

    /// Get next SCSI device
    pub unsafe fn get_next_device(&mut self, target: &mut u32, lun: &mut u64) -> Status {
        (self.get_next_device)(self, target as *mut _, lun as *mut _)
    }
}

impl NvmExpressPassThruProtocol {
    /// Send NVMe command
    pub unsafe fn pass_thru(
        &mut self,
        namespace_id: u32,
        packet: &mut NvmePassThruCommandPacket,
    ) -> Status {
        (self.pass_thru)(self, namespace_id, packet as *mut _, core::ptr::null_mut())
    }

    /// Get next NVMe namespace
    pub unsafe fn get_next_namespace(&mut self, namespace_id: &mut u32) -> Status {
        (self.get_next_namespace)(self, namespace_id as *mut _)
    }
}

/// Common SCSI Commands
pub mod scsi_commands {
    pub const SCSI_TEST_UNIT_READY: u8 = 0x00;
    pub const SCSI_REQUEST_SENSE: u8 = 0x03;
    pub const SCSI_INQUIRY: u8 = 0x12;
    pub const SCSI_MODE_SELECT_6: u8 = 0x15;
    pub const SCSI_MODE_SENSE_6: u8 = 0x1A;
    pub const SCSI_START_STOP_UNIT: u8 = 0x1B;
    pub const SCSI_READ_CAPACITY_10: u8 = 0x25;
    pub const SCSI_READ_10: u8 = 0x28;
    pub const SCSI_WRITE_10: u8 = 0x2A;
    pub const SCSI_READ_CAPACITY_16: u8 = 0x9E;
    pub const SCSI_READ_16: u8 = 0x88;
    pub const SCSI_WRITE_16: u8 = 0x8A;
}

/// GPT Partition Type GUIDs
pub mod gpt_partition_types {
    use crate::ffi::Guid;

    /// Unused entry
    pub const UNUSED_ENTRY_GUID: Guid = Guid::new(
        0x00000000,
        0x0000,
        0x0000,
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );

    /// EFI System Partition
    pub const EFI_SYSTEM_PARTITION_GUID: Guid = Guid::new(
        0xc12a7328,
        0xf81f,
        0x11d2,
        [0xba, 0x4b, 0x00, 0xa0, 0xc9, 0x3e, 0xc9, 0x3b],
    );

    /// Microsoft Basic Data
    pub const MICROSOFT_BASIC_DATA_GUID: Guid = Guid::new(
        0xebd0a0a2,
        0xb9e5,
        0x4433,
        [0x87, 0xc0, 0x68, 0xb6, 0xb7, 0x26, 0x99, 0xc7],
    );

    /// Linux Filesystem Data
    pub const LINUX_FILESYSTEM_DATA_GUID: Guid = Guid::new(
        0x0fc63daf,
        0x8483,
        0x4772,
        [0x8e, 0x79, 0x3d, 0x69, 0xd8, 0x47, 0x7d, 0xe4],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scsi_commands() {
        assert_eq!(scsi_commands::SCSI_INQUIRY, 0x12);
        assert_eq!(scsi_commands::SCSI_READ_10, 0x28);
        assert_eq!(scsi_commands::SCSI_WRITE_10, 0x2A);
    }

    #[test]
    fn test_partition_type() {
        assert_eq!(PartitionType::Mbr as u32, 1);
        assert_eq!(PartitionType::Gpt as u32, 2);
    }
}
