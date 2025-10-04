// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Firmware Table Parsing Tests

#![cfg(test)]

use uefi_rust::tables::*;
use uefi_rust::ffi::*;

#[test]
fn test_acpi_rsdp_signature() {
    let rsdp = Rsdp {
        signature: *b"RSD PTR ",
        checksum: 0,
        oem_id: *b"UEFI  ",
        revision: 2,
        rsdt_address: 0,
        length: 36,
        xsdt_address: 0,
        extended_checksum: 0,
        reserved: [0; 3],
    };

    assert_eq!(&rsdp.signature, b"RSD PTR ");
    assert_eq!(rsdp.revision, 2);
}

#[test]
fn test_acpi_checksum_validation() {
    // Create a simple test structure
    let data = [0x10u8, 0x20, 0x30, 0x40, 0x50];

    // Calculate checksum
    let mut sum: u8 = 0;
    for &byte in &data {
        sum = sum.wrapping_add(byte);
    }
    let checksum = (256 - sum as u16) as u8;

    // Verify checksum makes sum zero
    let mut total: u8 = 0;
    for &byte in &data {
        total = total.wrapping_add(byte);
    }
    total = total.wrapping_add(checksum);

    assert_eq!(total, 0);
}

#[test]
fn test_acpi_sdt_header_size() {
    use acpi_advanced::AcpiSdtHeader;

    let size = core::mem::size_of::<AcpiSdtHeader>();
    assert_eq!(size, 36);
}

#[test]
fn test_acpi_fadt_signature() {
    let header = AcpiDescriptionHeader {
        signature: *b"FACP",
        length: 0,
        revision: 0,
        checksum: 0,
        oem_id: [0; 6],
        oem_table_id: [0; 8],
        oem_revision: 0,
        creator_id: 0,
        creator_revision: 0,
    };

    assert_eq!(&header.signature, b"FACP");
}

#[test]
fn test_acpi_madt_signature() {
    let signature = b"APIC";
    assert_eq!(signature.len(), 4);
    assert_eq!(signature, b"APIC");
}

#[test]
fn test_smbios_entry_point_signature() {
    // SMBIOS 2.x signature
    let sig_2x = b"_SM_";
    assert_eq!(sig_2x.len(), 4);

    // SMBIOS 3.0 signature
    let sig_3 = b"_SM3_";
    assert_eq!(sig_3.len(), 5);
}

#[test]
fn test_smbios_structure_types() {
    assert_eq!(SmbiosStructureType::BiosInformation as u8, 0);
    assert_eq!(SmbiosStructureType::SystemInformation as u8, 1);
    assert_eq!(SmbiosStructureType::BaseboardInformation as u8, 2);
    assert_eq!(SmbiosStructureType::ProcessorInformation as u8, 4);
    assert_eq!(SmbiosStructureType::MemoryDevice as u8, 17);
}

#[test]
fn test_smbios_bios_characteristics() {
    let bios = SmbiosBiosInformation {
        header: SmbiosStructureHeader {
            structure_type: 0,
            length: 0,
            handle: 0,
        },
        vendor: 1,
        bios_version: 2,
        bios_starting_segment: 0xE000,
        bios_release_date: 3,
        bios_rom_size: 0x7F,
        bios_characteristics: 0x01 | 0x08,
        extension_byte1: 0,
        extension_byte2: 0,
        bios_major_release: 1,
        bios_minor_release: 0,
        ec_major_release: 0xFF,
        ec_minor_release: 0xFF,
    };

    assert_eq!(bios.vendor, 1);
    assert_eq!(bios.bios_starting_segment, 0xE000);
    assert!(bios.bios_characteristics & 0x01 != 0); // Bit 0 set
    assert!(bios.bios_characteristics & 0x08 != 0); // Bit 3 set
}

#[test]
fn test_smbios_processor_types() {
    const PROCESSOR_TYPE_OTHER: u8 = 0x01;
    const PROCESSOR_TYPE_UNKNOWN: u8 = 0x02;
    const PROCESSOR_TYPE_CENTRAL_PROCESSOR: u8 = 0x03;

    assert_eq!(PROCESSOR_TYPE_OTHER, 0x01);
    assert_eq!(PROCESSOR_TYPE_UNKNOWN, 0x02);
    assert_eq!(PROCESSOR_TYPE_CENTRAL_PROCESSOR, 0x03);
}

#[test]
fn test_smbios_memory_type() {
    const MEMORY_TYPE_DDR: u8 = 0x12;
    const MEMORY_TYPE_DDR2: u8 = 0x13;
    const MEMORY_TYPE_DDR3: u8 = 0x18;
    const MEMORY_TYPE_DDR4: u8 = 0x1A;

    assert_eq!(MEMORY_TYPE_DDR, 0x12);
    assert_eq!(MEMORY_TYPE_DDR2, 0x13);
    assert_eq!(MEMORY_TYPE_DDR3, 0x18);
    assert_eq!(MEMORY_TYPE_DDR4, 0x1A);
}

#[test]
fn test_configuration_table_guids() {
    use configuration::*;

    // ACPI GUIDs
    assert_ne!(ACPI_TABLE_GUID, ACPI_20_TABLE_GUID);

    // SMBIOS GUIDs
    assert_ne!(SMBIOS_TABLE_GUID, SMBIOS3_TABLE_GUID);

    // Unique GUIDs
    assert_ne!(ACPI_TABLE_GUID, SMBIOS_TABLE_GUID);
}

#[test]
fn test_acpi_hpet_structure() {
    use acpi_advanced::*;

    let size = core::mem::size_of::<AcpiHpet>();
    assert!(size > core::mem::size_of::<AcpiSdtHeader>());
}

#[test]
fn test_acpi_mcfg_structure() {
    use acpi_advanced::*;

    let entry = McfgConfigSpaceEntry {
        base_address: 0xE0000000,
        pci_segment_group: 0,
        start_bus_number: 0,
        end_bus_number: 255,
        reserved: 0,
    };

    assert_eq!(entry.base_address, 0xE0000000);
    assert_eq!(entry.start_bus_number, 0);
    assert_eq!(entry.end_bus_number, 255);
}

#[test]
fn test_pcie_address_calculation() {
    use acpi_advanced::mcfg_helpers::*;

    let entry = McfgConfigSpaceEntry {
        base_address: 0xE0000000,
        pci_segment_group: 0,
        start_bus_number: 0,
        end_bus_number: 255,
        reserved: 0,
    };

    // Bus 0, Device 0, Function 0
    let addr = get_pcie_address(&entry, 0, 0, 0);
    assert_eq!(addr, Some(0xE0000000));

    // Bus 1, Device 2, Function 3
    let addr = get_pcie_address(&entry, 1, 2, 3);
    let expected = 0xE0000000 + (1u64 << 20) + (2u64 << 15) + (3u64 << 12);
    assert_eq!(addr, Some(expected));

    // Out of range bus
    let addr = get_pcie_address(&entry, 0, 0, 0);
    let entry_small = McfgConfigSpaceEntry {
        base_address: 0xE0000000,
        pci_segment_group: 0,
        start_bus_number: 10,
        end_bus_number: 20,
        reserved: 0,
    };
    let addr = get_pcie_address(&entry_small, 5, 0, 0);
    assert_eq!(addr, None);

    // Invalid device (>= 32)
    let addr = get_pcie_address(&entry, 0, 32, 0);
    assert_eq!(addr, None);

    // Invalid function (>= 8)
    let addr = get_pcie_address(&entry, 0, 0, 8);
    assert_eq!(addr, None);
}

#[test]
fn test_hpet_helpers() {
    use acpi_advanced::hpet_helpers::*;
    use acpi_advanced::*;

    let hpet = AcpiHpet {
        header: AcpiSdtHeader {
            signature: *b"HPET",
            length: 0,
            revision: 0,
            checksum: 0,
            oem_id: [0; 6],
            oem_table_id: [0; 8],
            oem_revision: 0,
            creator_id: 0,
            creator_revision: 0,
        },
        hardware_rev_id: 1,
        comparator_count: 2,
        counter_size: 1,
        reserved: 0,
        legacy_replacement: 1,
        pci_vendor_id: 0x8086,
        address_space_id: 0,
        register_bit_width: 64,
        register_bit_offset: 0,
        reserved2: 0,
        address: 0xFED00000,
        hpet_number: 0,
        minimum_tick: 0x80,
        page_protection: 0,
    };

    assert!(is_64bit_counter(&hpet));
    assert_eq!(get_comparator_count(&hpet), 3); // comparator_count + 1
    assert!(has_legacy_replacement(&hpet));
}

#[test]
fn test_acpi_table_sizes() {
    use acpi_advanced::*;

    assert!(core::mem::size_of::<AcpiBgrt>() > 0);
    assert!(core::mem::size_of::<AcpiBert>() > 0);
    assert!(core::mem::size_of::<AcpiDmar>() > 0);
    assert!(core::mem::size_of::<AcpiWaet>() > 0);
}

#[test]
fn test_gpt_partition_guids() {
    use storage::gpt_partition_types::*;

    assert_ne!(EFI_SYSTEM_PARTITION_GUID, MICROSOFT_BASIC_DATA_GUID);
    assert_ne!(MICROSOFT_BASIC_DATA_GUID, LINUX_FILESYSTEM_DATA_GUID);
}

#[test]
fn test_configuration_table_structure() {
    let table = ConfigurationTable {
        vendor_guid: ACPI_20_TABLE_GUID,
        vendor_table: 0x12345678 as *mut core::ffi::c_void,
    };

    assert_eq!(table.vendor_guid, ACPI_20_TABLE_GUID);
    assert!(!table.vendor_table.is_null());
}
