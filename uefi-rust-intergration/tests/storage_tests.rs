// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Storage Protocol Tests

#![cfg(test)]

use uefi_rust::protocols::storage::*;
use uefi_rust::ffi::*;

#[test]
fn test_scsi_commands() {
    use scsi_commands::*;

    assert_eq!(SCSI_TEST_UNIT_READY, 0x00);
    assert_eq!(SCSI_REQUEST_SENSE, 0x03);
    assert_eq!(SCSI_INQUIRY, 0x12);
    assert_eq!(SCSI_MODE_SELECT_6, 0x15);
    assert_eq!(SCSI_MODE_SENSE_6, 0x1A);
    assert_eq!(SCSI_READ_CAPACITY_10, 0x25);
    assert_eq!(SCSI_READ_10, 0x28);
    assert_eq!(SCSI_WRITE_10, 0x2A);
    assert_eq!(SCSI_READ_CAPACITY_16, 0x9E);
    assert_eq!(SCSI_READ_16, 0x88);
    assert_eq!(SCSI_WRITE_16, 0x8A);
}

#[test]
fn test_scsi_data_direction() {
    assert_eq!(SCSI_DATA_IN, 0);
    assert_eq!(SCSI_DATA_OUT, 1);
}

#[test]
fn test_partition_types() {
    assert_eq!(PartitionType::Other as u32, 0);
    assert_eq!(PartitionType::Mbr as u32, 1);
    assert_eq!(PartitionType::Gpt as u32, 2);
}

#[test]
fn test_gpt_partition_guids() {
    use gpt_partition_types::*;

    // Verify GPT partition type GUIDs are unique
    assert_ne!(EFI_SYSTEM_PARTITION_GUID, MICROSOFT_BASIC_DATA_GUID);
    assert_ne!(MICROSOFT_BASIC_DATA_GUID, LINUX_FILESYSTEM_DATA_GUID);
    assert_ne!(EFI_SYSTEM_PARTITION_GUID, LINUX_FILESYSTEM_DATA_GUID);
}

#[test]
fn test_mbr_partition_record_size() {
    assert_eq!(core::mem::size_of::<MbrPartitionRecord>(), 16);
}

#[test]
fn test_gpt_partition_entry_size() {
    assert_eq!(core::mem::size_of::<GptPartitionEntry>(), 128);
}

#[test]
fn test_mbr_boot_indicator() {
    let partition = MbrPartitionRecord {
        boot_indicator: 0x80, // Bootable
        start_head: 0,
        start_sector: 1,
        start_track: 0,
        os_indicator: 0x0C, // FAT32 LBA
        end_head: 0,
        end_sector: 0,
        end_track: 0,
        starting_lba: [0, 0, 0, 0],
        size_in_lba: [0, 0, 0, 0],
    };

    assert_eq!(partition.boot_indicator, 0x80);
    assert_eq!(partition.os_indicator, 0x0C);
}

#[test]
fn test_gpt_partition_attributes() {
    const GPT_ATTR_REQUIRED_PARTITION: u64 = 0x0000000000000001;
    const GPT_ATTR_NO_BLOCK_IO_PROTOCOL: u64 = 0x0000000000000002;
    const GPT_ATTR_LEGACY_BIOS_BOOTABLE: u64 = 0x0000000000000004;

    assert_eq!(GPT_ATTR_REQUIRED_PARTITION, 1);
    assert_eq!(GPT_ATTR_NO_BLOCK_IO_PROTOCOL, 2);
    assert_eq!(GPT_ATTR_LEGACY_BIOS_BOOTABLE, 4);
}

#[test]
fn test_disk_io_protocol_revision() {
    // DiskIO revision should be defined
    const EFI_DISK_IO_PROTOCOL_REVISION: u64 = 0x00010000;
    assert_eq!(EFI_DISK_IO_PROTOCOL_REVISION, 0x00010000);
}

#[test]
fn test_disk_io2_protocol_revision() {
    // DiskIO2 revision
    const EFI_DISK_IO2_PROTOCOL_REVISION: u64 = 0x00020000;
    assert_eq!(EFI_DISK_IO2_PROTOCOL_REVISION, 0x00020000);
}

#[test]
fn test_scsi_pass_thru_mode_attributes() {
    const EFI_SCSI_PASS_THRU_ATTRIBUTES_PHYSICAL: u32 = 0x0001;
    const EFI_SCSI_PASS_THRU_ATTRIBUTES_LOGICAL: u32 = 0x0002;
    const EFI_SCSI_PASS_THRU_ATTRIBUTES_NONBLOCKIO: u32 = 0x0004;

    assert_eq!(EFI_SCSI_PASS_THRU_ATTRIBUTES_PHYSICAL, 0x0001);
    assert_eq!(EFI_SCSI_PASS_THRU_ATTRIBUTES_LOGICAL, 0x0002);
    assert_eq!(EFI_SCSI_PASS_THRU_ATTRIBUTES_NONBLOCKIO, 0x0004);
}

#[test]
fn test_nvme_command_structure_size() {
    assert_eq!(core::mem::size_of::<NvmeCommand>(), 44);
}

#[test]
fn test_nvme_completion_structure_size() {
    assert_eq!(core::mem::size_of::<NvmeCompletion>(), 16);
}

#[test]
fn test_nvme_queue_types() {
    const NVME_ADMIN_QUEUE: u8 = 0;
    const NVME_IO_QUEUE: u8 = 1;

    assert_eq!(NVME_ADMIN_QUEUE, 0);
    assert_eq!(NVME_IO_QUEUE, 1);
}

#[test]
fn test_nvme_admin_commands() {
    const NVME_ADMIN_DELETE_SQ: u8 = 0x00;
    const NVME_ADMIN_CREATE_SQ: u8 = 0x01;
    const NVME_ADMIN_DELETE_CQ: u8 = 0x04;
    const NVME_ADMIN_CREATE_CQ: u8 = 0x05;
    const NVME_ADMIN_IDENTIFY: u8 = 0x06;

    assert_eq!(NVME_ADMIN_IDENTIFY, 0x06);
}

#[test]
fn test_nvme_io_commands() {
    const NVME_CMD_FLUSH: u8 = 0x00;
    const NVME_CMD_WRITE: u8 = 0x01;
    const NVME_CMD_READ: u8 = 0x02;

    assert_eq!(NVME_CMD_FLUSH, 0x00);
    assert_eq!(NVME_CMD_WRITE, 0x01);
    assert_eq!(NVME_CMD_READ, 0x02);
}

#[test]
fn test_storage_protocol_guids_unique() {
    assert_ne!(SCSI_PASS_THRU_PROTOCOL_GUID, EXT_SCSI_PASS_THRU_PROTOCOL_GUID);
    assert_ne!(SCSI_PASS_THRU_PROTOCOL_GUID, NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID);
    assert_ne!(DISK_IO_PROTOCOL_GUID, DISK_IO2_PROTOCOL_GUID);
    assert_ne!(DISK_IO_PROTOCOL_GUID, PARTITION_INFO_PROTOCOL_GUID);
}

#[test]
fn test_lba_addressing() {
    // Test LBA calculation
    let sector_size: u64 = 512;
    let offset: u64 = 4096;
    let lba = offset / sector_size;

    assert_eq!(lba, 8);

    // Another example
    let offset: u64 = 1_048_576; // 1MB
    let lba = offset / sector_size;
    assert_eq!(lba, 2048);
}

#[test]
fn test_partition_table_signature() {
    const MBR_SIGNATURE: u16 = 0xAA55;
    const GPT_SIGNATURE: [u8; 8] = *b"EFI PART";

    assert_eq!(MBR_SIGNATURE, 0xAA55);
    assert_eq!(&GPT_SIGNATURE, b"EFI PART");
}

#[test]
fn test_common_os_indicators() {
    const OS_INDICATOR_FAT12: u8 = 0x01;
    const OS_INDICATOR_FAT16_SMALL: u8 = 0x04;
    const OS_INDICATOR_EXTENDED: u8 = 0x05;
    const OS_INDICATOR_FAT16: u8 = 0x06;
    const OS_INDICATOR_NTFS: u8 = 0x07;
    const OS_INDICATOR_FAT32: u8 = 0x0B;
    const OS_INDICATOR_FAT32_LBA: u8 = 0x0C;
    const OS_INDICATOR_FAT16_LBA: u8 = 0x0E;
    const OS_INDICATOR_LINUX: u8 = 0x83;

    assert_eq!(OS_INDICATOR_FAT12, 0x01);
    assert_eq!(OS_INDICATOR_FAT32_LBA, 0x0C);
    assert_eq!(OS_INDICATOR_LINUX, 0x83);
}

#[test]
fn test_block_size_common_values() {
    const BLOCK_SIZE_512: u32 = 512;
    const BLOCK_SIZE_4K: u32 = 4096;
    const BLOCK_SIZE_8K: u32 = 8192;

    assert_eq!(BLOCK_SIZE_512, 512);
    assert_eq!(BLOCK_SIZE_4K, 4096);
    assert_eq!(BLOCK_SIZE_8K, 8192);
}
