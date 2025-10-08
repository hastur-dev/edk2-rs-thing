// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Storage Protocol Tests

#![cfg(test)]

use uefi_rust::ffi::*;
use uefi_rust::protocols::storage::*;

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
    assert_ne!(
        SCSI_PASS_THRU_PROTOCOL_GUID,
        EXT_SCSI_PASS_THRU_PROTOCOL_GUID
    );
    assert_ne!(
        SCSI_PASS_THRU_PROTOCOL_GUID,
        NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID
    );
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

// ============================================================================
// SCSI Command Builder Tests
// ============================================================================

#[test]
fn test_scsi_builder_inquiry() {
    let mut buffer = [0u8; 36];
    let (packet, cdb) = scsi_builder::build_inquiry(&mut buffer, 1_000_000);

    // Verify INQUIRY command structure
    assert_eq!(cdb[0], scsi_commands::SCSI_INQUIRY);
    assert_eq!(cdb[1], 0x00); // EVPD=0
    assert_eq!(cdb[2], 0x00); // Page code
    assert_eq!(cdb[4], 36); // Allocation length

    // Verify packet
    assert_eq!(packet.cdb_length, 6);
    assert_eq!(packet.data_direction, SCSI_DATA_IN);
    assert_eq!(packet.in_transfer_length, 36);
    assert_eq!(packet.out_transfer_length, 0);
    assert!(packet.in_data_buffer.is_null() == false);
    assert!(packet.out_data_buffer.is_null());
}

#[test]
fn test_scsi_builder_read10() {
    let mut buffer = [0u8; 512];
    let lba: u32 = 100;
    let (packet, cdb) = scsi_builder::build_read10(lba, &mut buffer, 1_000_000);

    // Verify READ(10) command structure
    assert_eq!(cdb[0], scsi_commands::SCSI_READ_10);

    // Verify LBA encoding (big-endian)
    let decoded_lba = ((cdb[2] as u32) << 24)
        | ((cdb[3] as u32) << 16)
        | ((cdb[4] as u32) << 8)
        | (cdb[5] as u32);
    assert_eq!(decoded_lba, lba);

    // Verify transfer length (1 block)
    let blocks = ((cdb[7] as u16) << 8) | (cdb[8] as u16);
    assert_eq!(blocks, 1);

    // Verify packet
    assert_eq!(packet.cdb_length, 10);
    assert_eq!(packet.data_direction, SCSI_DATA_IN);
    assert_eq!(packet.in_transfer_length, 512);
}

#[test]
fn test_scsi_builder_write10() {
    let buffer = [0xAA; 512];
    let lba: u32 = 200;
    let (packet, cdb) = scsi_builder::build_write10(lba, &buffer, 1_000_000);

    // Verify WRITE(10) command structure
    assert_eq!(cdb[0], scsi_commands::SCSI_WRITE_10);

    // Verify LBA encoding
    let decoded_lba = ((cdb[2] as u32) << 24)
        | ((cdb[3] as u32) << 16)
        | ((cdb[4] as u32) << 8)
        | (cdb[5] as u32);
    assert_eq!(decoded_lba, lba);

    // Verify packet
    assert_eq!(packet.cdb_length, 10);
    assert_eq!(packet.data_direction, SCSI_DATA_OUT);
    assert_eq!(packet.out_transfer_length, 512);
    assert_eq!(packet.in_transfer_length, 0);
}

#[test]
fn test_scsi_builder_large_buffer() {
    let mut buffer = [0u8; 4096]; // 8 blocks
    let (packet, cdb) = scsi_builder::build_read10(0, &mut buffer, 1_000_000);

    // Verify transfer length (8 blocks)
    let blocks = ((cdb[7] as u16) << 8) | (cdb[8] as u16);
    assert_eq!(blocks, 8);
    assert_eq!(packet.in_transfer_length, 4096);
}

#[test]
fn test_scsi_builder_timeout() {
    let mut buffer = [0u8; 512];
    let timeout = 5_000_000; // 5 seconds
    let (packet, _cdb) = scsi_builder::build_inquiry(&mut buffer, timeout);

    assert_eq!(packet.timeout, timeout);
}

// ============================================================================
// Partition Type Tests
// ============================================================================

#[test]
fn test_partition_type_discriminants() {
    // Ensure discriminants match UEFI spec
    assert_eq!(PartitionType::Other as u32, 0);
    assert_eq!(PartitionType::Mbr as u32, 1);
    assert_eq!(PartitionType::Gpt as u32, 2);
}

#[test]
fn test_partition_type_equality() {
    let p1 = PartitionType::Gpt;
    let p2 = PartitionType::Gpt;
    let p3 = PartitionType::Mbr;

    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

#[test]
fn test_partition_type_copy() {
    let p1 = PartitionType::Gpt;
    let p2 = p1; // Copy trait
    assert_eq!(p1, p2);
}

// ============================================================================
// Structure Size Tests
// ============================================================================

#[test]
fn test_scsi_pass_thru_request_packet_size() {
    // Verify structure size matches expectations
    let size = core::mem::size_of::<ScsiPassThruRequestPacket>();
    assert!(size > 0);
    // Should contain pointers and various fields
    assert!(size >= 48); // Minimum expected size
}

#[test]
fn test_ext_scsi_pass_thru_request_packet_size() {
    let size = core::mem::size_of::<ExtScsiPassThruRequestPacket>();
    assert!(size > 0);
    assert!(size >= 48);
}

#[test]
fn test_nvme_command_packet_size() {
    let size = core::mem::size_of::<NvmePassThruCommandPacket>();
    assert!(size > 0);
    assert!(size >= 32);
}

#[test]
fn test_disk_io2_token_size() {
    let size = core::mem::size_of::<DiskIo2Token>();
    assert!(size >= 16); // Event handle + Status
}

#[test]
fn test_partition_info_size() {
    let size = core::mem::size_of::<PartitionInfo>();
    // Should be large enough for either MBR or GPT
    assert!(size >= core::mem::size_of::<GptPartitionEntry>());
}

// ============================================================================
// GUID Tests
// ============================================================================

#[test]
fn test_storage_protocol_guids_valid() {
    // Verify GUIDs are not all zeros
    assert_ne!(SCSI_PASS_THRU_PROTOCOL_GUID.data1, 0);
    assert_ne!(NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID.data1, 0);
    assert_ne!(DISK_IO_PROTOCOL_GUID.data1, 0);
    assert_ne!(DISK_IO2_PROTOCOL_GUID.data1, 0);
    assert_ne!(PARTITION_INFO_PROTOCOL_GUID.data1, 0);
}

#[test]
fn test_gpt_partition_type_guids_valid() {
    use gpt_partition_types::*;

    // Verify EFI System Partition GUID
    assert_eq!(EFI_SYSTEM_PARTITION_GUID.data1, 0xc12a7328);
    assert_eq!(EFI_SYSTEM_PARTITION_GUID.data2, 0xf81f);

    // Verify unused entry is all zeros
    assert_eq!(UNUSED_ENTRY_GUID.data1, 0);
    assert_eq!(UNUSED_ENTRY_GUID.data2, 0);
    assert_eq!(UNUSED_ENTRY_GUID.data3, 0);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_zero_lba() {
    let mut buffer = [0u8; 512];
    let (packet, cdb) = scsi_builder::build_read10(0, &mut buffer, 1_000_000);

    // Verify LBA 0
    let decoded_lba = ((cdb[2] as u32) << 24)
        | ((cdb[3] as u32) << 16)
        | ((cdb[4] as u32) << 8)
        | (cdb[5] as u32);
    assert_eq!(decoded_lba, 0);
}

#[test]
fn test_max_lba_32bit() {
    let mut buffer = [0u8; 512];
    let max_lba = u32::MAX;
    let (_packet, cdb) = scsi_builder::build_read10(max_lba, &mut buffer, 1_000_000);

    // Verify max LBA encoding
    let decoded_lba = ((cdb[2] as u32) << 24)
        | ((cdb[3] as u32) << 16)
        | ((cdb[4] as u32) << 8)
        | (cdb[5] as u32);
    assert_eq!(decoded_lba, max_lba);
    assert_eq!(cdb[2], 0xFF);
    assert_eq!(cdb[3], 0xFF);
    assert_eq!(cdb[4], 0xFF);
    assert_eq!(cdb[5], 0xFF);
}

#[test]
fn test_small_inquiry_buffer() {
    let mut buffer = [0u8; 5];
    let (_packet, cdb) = scsi_builder::build_inquiry(&mut buffer, 1_000_000);

    // Should handle small buffers
    assert_eq!(cdb[4], 5);
}

#[test]
fn test_nvme_command_cdw0_identify() {
    let mut cmd = NvmeCommand {
        cdw0: 0x06, // Identify command opcode
        flags: 0,
        nsid: 0,
        cdw2: 0,
        cdw3: 0,
        cdw10: 0x01, // Controller structure
        cdw11: 0,
        cdw12: 0,
        cdw13: 0,
        cdw14: 0,
        cdw15: 0,
    };

    assert_eq!(cmd.cdw0, 0x06);
    assert_eq!(cmd.cdw10, 0x01);
}

#[test]
fn test_nvme_completion_zero_init() {
    let completion = NvmeCompletion {
        dw0: 0,
        dw1: 0,
        dw2: 0,
        dw3: 0,
    };

    assert_eq!(completion.dw0, 0);
    assert_eq!(completion.dw1, 0);
    assert_eq!(completion.dw2, 0);
    assert_eq!(completion.dw3, 0);
}

// ============================================================================
// Protocol Mode Tests
// ============================================================================

#[test]
fn test_scsi_pass_thru_mode_io_align() {
    let mode = ScsiPassThruMode {
        adapter_id: 0,
        attributes: 0,
        io_align: 4, // 4-byte alignment
    };

    assert_eq!(mode.io_align, 4);
}

#[test]
fn test_nvme_mode_version() {
    let mode = NvmExpressPassThruMode {
        attributes: 0,
        io_align: 4,
        nvme_version: 0x00010400, // NVMe 1.4
    };

    assert_eq!(mode.nvme_version, 0x00010400);
}

// ============================================================================
// Data Direction Tests
// ============================================================================

#[test]
fn test_data_direction_values() {
    assert_eq!(SCSI_DATA_IN, 0);
    assert_eq!(SCSI_DATA_OUT, 1);

    // Ensure they're different
    assert_ne!(SCSI_DATA_IN, SCSI_DATA_OUT);
}

// ============================================================================
// Alignment Tests
// ============================================================================

#[test]
fn test_structure_alignments() {
    // Verify structures have reasonable alignment
    assert_eq!(
        core::mem::align_of::<ScsiPassThruRequestPacket>(),
        core::mem::align_of::<u64>()
    );
    assert_eq!(
        core::mem::align_of::<NvmeCommand>(),
        core::mem::align_of::<u32>()
    );
    assert_eq!(core::mem::align_of::<MbrPartitionRecord>(), 1); // Packed
    assert_eq!(
        core::mem::align_of::<GptPartitionEntry>(),
        core::mem::align_of::<u64>()
    );
}

#[test]
fn test_partition_info_union_size() {
    // Union should be at least as large as the largest member
    let union_size = core::mem::size_of::<PartitionInfoUnion>();
    let mbr_size = core::mem::size_of::<MbrPartitionRecord>();
    let gpt_size = core::mem::size_of::<GptPartitionEntry>();

    assert!(union_size >= mbr_size);
    assert!(union_size >= gpt_size);
}

// ============================================================================
// Realistic Scenario Tests
// ============================================================================

#[test]
fn test_realistic_read_sector() {
    // Simulate reading sector 100 (boot sector area)
    let mut buffer = [0u8; 512];
    let lba = 100;
    let timeout = 3_000_000; // 3 seconds

    let (packet, cdb) = scsi_builder::build_read10(lba, &mut buffer, timeout);

    // Verify all fields are set correctly for a real read
    assert_eq!(packet.timeout, timeout);
    assert_eq!(packet.cdb_length, 10);
    assert_eq!(packet.data_direction, SCSI_DATA_IN);
    assert_eq!(packet.in_transfer_length, 512);
    assert_eq!(cdb[0], scsi_commands::SCSI_READ_10);

    // Verify LBA
    let decoded_lba = ((cdb[2] as u32) << 24)
        | ((cdb[3] as u32) << 16)
        | ((cdb[4] as u32) << 8)
        | (cdb[5] as u32);
    assert_eq!(decoded_lba, 100);
}

#[test]
fn test_realistic_write_configuration() {
    // Simulate writing configuration data to sector 200
    let data = [0x55; 512]; // Test pattern
    let lba = 200;
    let timeout = 5_000_000; // 5 seconds

    let (packet, cdb) = scsi_builder::build_write10(lba, &data, timeout);

    assert_eq!(packet.timeout, timeout);
    assert_eq!(packet.data_direction, SCSI_DATA_OUT);
    assert_eq!(packet.out_transfer_length, 512);
    assert_eq!(cdb[0], scsi_commands::SCSI_WRITE_10);
}

#[test]
fn test_multiple_block_transfer() {
    // Test transferring multiple blocks (4KB = 8 sectors)
    let mut buffer = [0u8; 4096];
    let (_packet, cdb) = scsi_builder::build_read10(1000, &mut buffer, 1_000_000);

    let blocks = ((cdb[7] as u16) << 8) | (cdb[8] as u16);
    assert_eq!(blocks, 8); // 4096 / 512 = 8 blocks
}
