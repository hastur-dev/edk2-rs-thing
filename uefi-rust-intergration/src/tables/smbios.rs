// SPDX-License-Identifier: BSD-2-Clause-Patent
//! SMBIOS Table Parsing

use crate::ffi::*;

/// SMBIOS Table GUID
pub const SMBIOS_TABLE_GUID: Guid = Guid::new(
    0xeb9d2d31,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

/// SMBIOS 3.0 Table GUID
pub const SMBIOS3_TABLE_GUID: Guid = Guid::new(
    0xf2fd1544,
    0x9794,
    0x4a2c,
    [0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94],
);

/// SMBIOS Entry Point Structure (32-bit)
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SmbiosEntryPoint {
    pub anchor_string: [u8; 4],         // "_SM_"
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub max_structure_size: u16,
    pub entry_point_revision: u8,
    pub formatted_area: [u8; 5],
    pub intermediate_anchor: [u8; 5],   // "_DMI_"
    pub intermediate_checksum: u8,
    pub structure_table_length: u16,
    pub structure_table_address: u32,
    pub number_of_structures: u16,
    pub bcd_revision: u8,
}

/// SMBIOS 3.0 Entry Point Structure (64-bit)
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Smbios3EntryPoint {
    pub anchor_string: [u8; 5],         // "_SM3_"
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub docrev: u8,
    pub entry_point_revision: u8,
    pub reserved: u8,
    pub structure_table_max_size: u32,
    pub structure_table_address: u64,
}

/// SMBIOS Structure Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SmbiosHeader {
    pub struct_type: u8,
    pub length: u8,
    pub handle: u16,
}

/// SMBIOS Type 0: BIOS Information
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BiosInformation {
    pub header: SmbiosHeader,
    pub vendor: u8,                     // String number
    pub bios_version: u8,               // String number
    pub bios_starting_segment: u16,
    pub bios_release_date: u8,          // String number
    pub bios_rom_size: u8,
    pub bios_characteristics: u64,
    // Additional fields may follow depending on version
}

/// SMBIOS Type 1: System Information
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SystemInformation {
    pub header: SmbiosHeader,
    pub manufacturer: u8,               // String number
    pub product_name: u8,               // String number
    pub version: u8,                    // String number
    pub serial_number: u8,              // String number
    pub uuid: [u8; 16],
    pub wake_up_type: u8,
    pub sku_number: u8,                 // String number
    pub family: u8,                     // String number
}

/// SMBIOS Type 2: Baseboard Information
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BaseboardInformation {
    pub header: SmbiosHeader,
    pub manufacturer: u8,               // String number
    pub product: u8,                    // String number
    pub version: u8,                    // String number
    pub serial_number: u8,              // String number
    pub asset_tag: u8,                  // String number
    pub feature_flags: u8,
    pub location_in_chassis: u8,        // String number
    pub chassis_handle: u16,
    pub board_type: u8,
    pub number_of_contained_handles: u8,
}

/// SMBIOS Type 4: Processor Information
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorInformation {
    pub header: SmbiosHeader,
    pub socket_designation: u8,         // String number
    pub processor_type: u8,
    pub processor_family: u8,
    pub processor_manufacturer: u8,     // String number
    pub processor_id: u64,
    pub processor_version: u8,          // String number
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: u8,
    pub processor_upgrade: u8,
    // Additional fields may follow
}

/// SMBIOS Type 17: Memory Device
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryDevice {
    pub header: SmbiosHeader,
    pub physical_memory_array_handle: u16,
    pub memory_error_info_handle: u16,
    pub total_width: u16,
    pub data_width: u16,
    pub size: u16,
    pub form_factor: u8,
    pub device_set: u8,
    pub device_locator: u8,             // String number
    pub bank_locator: u8,               // String number
    pub memory_type: u8,
    pub type_detail: u16,
    // Additional fields may follow
}

// SMBIOS Type constants
pub const SMBIOS_TYPE_BIOS_INFO: u8 = 0;
pub const SMBIOS_TYPE_SYSTEM_INFO: u8 = 1;
pub const SMBIOS_TYPE_BASEBOARD_INFO: u8 = 2;
pub const SMBIOS_TYPE_SYSTEM_ENCLOSURE: u8 = 3;
pub const SMBIOS_TYPE_PROCESSOR_INFO: u8 = 4;
pub const SMBIOS_TYPE_MEMORY_CONTROLLER: u8 = 5;
pub const SMBIOS_TYPE_MEMORY_MODULE: u8 = 6;
pub const SMBIOS_TYPE_CACHE_INFO: u8 = 7;
pub const SMBIOS_TYPE_PORT_CONNECTOR: u8 = 8;
pub const SMBIOS_TYPE_SYSTEM_SLOTS: u8 = 9;
pub const SMBIOS_TYPE_MEMORY_DEVICE: u8 = 17;
pub const SMBIOS_TYPE_END_OF_TABLE: u8 = 127;

impl SmbiosEntryPoint {
    /// Verify checksum
    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                self.length as usize,
            )
        };

        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    /// Check if this is a valid SMBIOS entry point
    pub fn is_valid(&self) -> bool {
        self.anchor_string == *b"_SM_" && self.verify_checksum()
    }

    /// Get SMBIOS version
    pub fn version(&self) -> (u8, u8) {
        (self.major_version, self.minor_version)
    }
}

impl Smbios3EntryPoint {
    /// Verify checksum
    pub fn verify_checksum(&self) -> bool {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const _ as *const u8,
                self.length as usize,
            )
        };

        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    /// Check if this is a valid SMBIOS 3.0 entry point
    pub fn is_valid(&self) -> bool {
        self.anchor_string == *b"_SM3_" && self.verify_checksum()
    }

    /// Get SMBIOS version
    pub fn version(&self) -> (u8, u8) {
        (self.major_version, self.minor_version)
    }
}

impl SmbiosHeader {
    /// Get the total size of this structure including strings
    pub unsafe fn total_size(&self) -> usize {
        let mut ptr = (self as *const _ as usize + self.length as usize) as *const u8;

        // Skip formatted area
        // Strings follow, terminated by double null
        loop {
            // Check for double null terminator
            if *ptr == 0 && *ptr.add(1) == 0 {
                return (ptr as usize - self as *const _ as usize) + 2;
            }

            // Skip to next string
            while *ptr != 0 {
                ptr = ptr.add(1);
            }
            ptr = ptr.add(1); // Skip the null terminator
        }
    }

    /// Get a string by index (1-based)
    pub unsafe fn get_string(&self, index: u8) -> Option<&[u8]> {
        if index == 0 {
            return None;
        }

        let mut ptr = (self as *const _ as usize + self.length as usize) as *const u8;
        let mut current_index = 1u8;

        loop {
            // Check for double null (end of strings)
            if *ptr == 0 {
                return None;
            }

            if current_index == index {
                // Found the string, calculate length
                let start = ptr;
                let mut len = 0;
                while *ptr.add(len) != 0 {
                    len += 1;
                }
                return Some(core::slice::from_raw_parts(start, len));
            }

            // Skip to next string
            while *ptr != 0 {
                ptr = ptr.add(1);
            }
            ptr = ptr.add(1);
            current_index += 1;
        }
    }
}

/// SMBIOS Table Iterator
pub struct SmbiosIterator {
    current: *const SmbiosHeader,
    end: *const u8,
}

impl SmbiosIterator {
    /// Create a new SMBIOS iterator
    pub unsafe fn new(table_start: *const u8, table_length: usize) -> Self {
        SmbiosIterator {
            current: table_start as *const SmbiosHeader,
            end: table_start.add(table_length),
        }
    }
}

impl Iterator for SmbiosIterator {
    type Item = *const SmbiosHeader;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current as *const u8 >= self.end {
                return None;
            }

            let header = &*self.current;

            if header.struct_type == SMBIOS_TYPE_END_OF_TABLE {
                return None;
            }

            let current = self.current;
            let total_size = header.total_size();

            // Move to next structure
            self.current = (self.current as usize + total_size) as *const SmbiosHeader;

            Some(current)
        }
    }
}
