// SPDX-License-Identifier: BSD-2-Clause-Patent
//! ACPI Table Parsing

use crate::ffi::*;

/// ACPI Table GUIDs
pub const ACPI_TABLE_GUID: Guid = Guid::new(
    0xeb9d2d30,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub const ACPI_20_TABLE_GUID: Guid = Guid::new(
    0x8868e871,
    0xe4f1,
    0x11d3,
    [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
);

/// RSDP (Root System Description Pointer) structure for ACPI 1.0
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RsdpDescriptor10 {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

/// RSDP structure for ACPI 2.0+
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RsdpDescriptor20 {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

/// Type alias for RSDP (uses ACPI 2.0+ structure)
pub type Rsdp = RsdpDescriptor20;

/// ACPI Table Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

/// RSDT (Root System Description Table)
#[repr(C, packed)]
pub struct Rsdt {
    pub header: AcpiTableHeader,
    // Followed by array of 32-bit pointers to other ACPI tables
}

/// XSDT (Extended System Description Table)
#[repr(C, packed)]
pub struct Xsdt {
    pub header: AcpiTableHeader,
    // Followed by array of 64-bit pointers to other ACPI tables
}

/// FADT (Fixed ACPI Description Table)
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Fadt {
    pub header: AcpiTableHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub reserved: u8,
    pub preferred_pm_profile: u8,
    pub sci_int: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_cnt: u8,
    pub pm1a_evt_blk: u32,
    pub pm1b_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm1b_cnt_blk: u32,
    pub pm2_cnt_blk: u32,
    pub pm_tmr_blk: u32,
    pub gpe0_blk: u32,
    pub gpe1_blk: u32,
    pub pm1_evt_len: u8,
    pub pm1_cnt_len: u8,
    pub pm2_cnt_len: u8,
    pub pm_tmr_len: u8,
    pub gpe0_blk_len: u8,
    pub gpe1_blk_len: u8,
    pub gpe1_base: u8,
    pub cst_cnt: u8,
    pub p_lvl2_lat: u16,
    pub p_lvl3_lat: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alrm: u8,
    pub mon_alrm: u8,
    pub century: u8,
    pub iapc_boot_arch: u16,
    pub reserved2: u8,
    pub flags: u32,
    // Extended fields for ACPI 2.0+
}

/// MADT (Multiple APIC Description Table)
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Madt {
    pub header: AcpiTableHeader,
    pub local_apic_address: u32,
    pub flags: u32,
    // Followed by variable-length interrupt controller structures
}

/// ACPI Signature constants
pub const RSDP_SIGNATURE: &[u8; 8] = b"RSD PTR ";
pub const RSDT_SIGNATURE: &[u8; 4] = b"RSDT";
pub const XSDT_SIGNATURE: &[u8; 4] = b"XSDT";
pub const FADT_SIGNATURE: &[u8; 4] = b"FACP";
pub const MADT_SIGNATURE: &[u8; 4] = b"APIC";
pub const MCFG_SIGNATURE: &[u8; 4] = b"MCFG";
pub const HPET_SIGNATURE: &[u8; 4] = b"HPET";
pub const BGRT_SIGNATURE: &[u8; 4] = b"BGRT";

impl RsdpDescriptor10 {
    /// Verify RSDP 1.0 checksum
    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                core::mem::size_of::<RsdpDescriptor10>(),
            )
        };

        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    /// Check if signature is valid
    pub fn is_valid(&self) -> bool {
        self.signature == *RSDP_SIGNATURE && self.verify_checksum()
    }
}

impl RsdpDescriptor20 {
    /// Verify RSDP 2.0+ extended checksum
    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(self as *const _ as *const u8, self.length as usize)
        };

        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    /// Check if signature is valid
    pub fn is_valid(&self) -> bool {
        self.signature == *RSDP_SIGNATURE && self.verify_checksum()
    }
}

impl AcpiTableHeader {
    /// Verify table checksum
    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(self as *const _ as *const u8, self.length as usize)
        };

        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    /// Check if signature matches
    pub fn has_signature(&self, sig: &[u8; 4]) -> bool {
        self.signature == *sig
    }
}

impl Rsdt {
    /// Get the number of table entries
    pub fn entry_count(&self) -> usize {
        (self.header.length as usize - core::mem::size_of::<AcpiTableHeader>()) / 4
    }

    /// Get table pointer at index
    pub unsafe fn get_entry(&self, index: usize) -> Option<u32> {
        if index >= self.entry_count() {
            return None;
        }

        let entries =
            (self as *const _ as usize + core::mem::size_of::<AcpiTableHeader>()) as *const u32;
        Some(*entries.add(index))
    }
}

impl Xsdt {
    /// Get the number of table entries
    pub fn entry_count(&self) -> usize {
        (self.header.length as usize - core::mem::size_of::<AcpiTableHeader>()) / 8
    }

    /// Get table pointer at index
    pub unsafe fn get_entry(&self, index: usize) -> Option<u64> {
        if index >= self.entry_count() {
            return None;
        }

        let entries =
            (self as *const _ as usize + core::mem::size_of::<AcpiTableHeader>()) as *const u64;
        Some(*entries.add(index))
    }
}
