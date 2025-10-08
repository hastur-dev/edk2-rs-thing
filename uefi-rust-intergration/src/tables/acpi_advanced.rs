// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Advanced ACPI Table Utilities and SDT Support

use crate::tables::acpi::*;

/// ACPI SDT (System Description Table) Header
#[repr(C)]
pub struct AcpiSdtHeader {
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

impl AcpiSdtHeader {
    /// Validate SDT checksum
    pub fn validate_checksum(&self) -> bool {
        let ptr = self as *const _ as *const u8;
        let length = self.length as usize;

        let mut sum: u8 = 0;
        for i in 0..length {
            sum = sum.wrapping_add(unsafe { *ptr.add(i) });
        }
        sum == 0
    }

    /// Get signature as string
    pub fn signature_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.signature)
    }

    /// Get OEM ID as string
    pub fn oem_id_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.oem_id)
    }
}

/// HPET (High Precision Event Timer) Table
#[repr(C)]
pub struct AcpiHpet {
    pub header: AcpiSdtHeader,
    pub hardware_rev_id: u8,
    pub comparator_count: u8,
    pub counter_size: u8,
    pub reserved: u8,
    pub legacy_replacement: u8,
    pub pci_vendor_id: u16,
    pub address_space_id: u8,
    pub register_bit_width: u8,
    pub register_bit_offset: u8,
    pub reserved2: u8,
    pub address: u64,
    pub hpet_number: u8,
    pub minimum_tick: u16,
    pub page_protection: u8,
}

/// MCFG (PCI Express Memory Mapped Configuration) Table
#[repr(C)]
pub struct AcpiMcfg {
    pub header: AcpiSdtHeader,
    pub reserved: u64,
    // Followed by configuration space entries
}

/// MCFG Configuration Space Entry
#[repr(C)]
pub struct McfgConfigSpaceEntry {
    pub base_address: u64,
    pub pci_segment_group: u16,
    pub start_bus_number: u8,
    pub end_bus_number: u8,
    pub reserved: u32,
}

/// BGRT (Boot Graphics Resource Table)
#[repr(C)]
pub struct AcpiBgrt {
    pub header: AcpiSdtHeader,
    pub version: u16,
    pub status: u8,
    pub image_type: u8,
    pub image_address: u64,
    pub image_offset_x: u32,
    pub image_offset_y: u32,
}

/// BERT (Boot Error Record Table)
#[repr(C)]
pub struct AcpiBert {
    pub header: AcpiSdtHeader,
    pub boot_error_region_length: u32,
    pub boot_error_region: u64,
}

/// EINJ (Error Injection Table)
#[repr(C)]
pub struct AcpiEinj {
    pub header: AcpiSdtHeader,
    pub injection_header_size: u32,
    pub injection_flags: u8,
    pub reserved: [u8; 3],
    pub injection_entry_count: u32,
    // Followed by injection entries
}

/// ERST (Error Record Serialization Table)
#[repr(C)]
pub struct AcpiErst {
    pub header: AcpiSdtHeader,
    pub serialization_header_size: u32,
    pub reserved: u32,
    pub instruction_entry_count: u32,
    // Followed by serialization instructions
}

/// FPDT (Firmware Performance Data Table)
#[repr(C)]
pub struct AcpiFpdt {
    pub header: AcpiSdtHeader,
    // Followed by performance records
}

/// GTDT (Generic Timer Description Table)
#[repr(C)]
pub struct AcpiGtdt {
    pub header: AcpiSdtHeader,
    pub cnt_control_base_physical_address: u64,
    pub reserved: u32,
    pub secure_el1_timer_gsiv: u32,
    pub secure_el1_timer_flags: u32,
    pub non_secure_el1_timer_gsiv: u32,
    pub non_secure_el1_timer_flags: u32,
    pub virtual_timer_gsiv: u32,
    pub virtual_timer_flags: u32,
    pub non_secure_el2_timer_gsiv: u32,
    pub non_secure_el2_timer_flags: u32,
    pub cnt_read_base_physical_address: u64,
    pub platform_timer_count: u32,
    pub platform_timer_offset: u32,
}

/// IORT (I/O Remapping Table)
#[repr(C)]
pub struct AcpiIort {
    pub header: AcpiSdtHeader,
    pub node_count: u32,
    pub node_offset: u32,
    pub reserved: u32,
}

/// SRAT (System Resource Affinity Table)
#[repr(C)]
pub struct AcpiSrat {
    pub header: AcpiSdtHeader,
    pub reserved1: u32,
    pub reserved2: u64,
    // Followed by static resource allocation structures
}

/// SLIT (System Locality Information Table)
#[repr(C)]
pub struct AcpiSlit {
    pub header: AcpiSdtHeader,
    pub number_of_system_localities: u64,
    // Followed by distance matrix
}

/// DMAR (DMA Remapping Table)
#[repr(C)]
pub struct AcpiDmar {
    pub header: AcpiSdtHeader,
    pub host_address_width: u8,
    pub flags: u8,
    pub reserved: [u8; 10],
    // Followed by remapping structures
}

/// WAET (Windows ACPI Emulated Devices Table)
#[repr(C)]
pub struct AcpiWaet {
    pub header: AcpiSdtHeader,
    pub flags: u32,
}

/// ACPI Table Iterator
pub struct AcpiTableIterator<'a> {
    current_ptr: *const AcpiSdtHeader,
    end_ptr: *const AcpiSdtHeader,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl<'a> AcpiTableIterator<'a> {
    /// Create a new ACPI table iterator from RSDT
    pub unsafe fn from_rsdt(rsdt: &'a Rsdt) -> Self {
        let _start = rsdt as *const _ as *const u8;
        let _header_size = core::mem::size_of::<Rsdt>();
        let _table_start = _start.add(_header_size) as *const u32;
        let _table_count = (rsdt.header.length as usize - _header_size) / 4;

        // This is simplified - in reality we'd iterate through the table entries
        AcpiTableIterator {
            current_ptr: core::ptr::null(),
            end_ptr: core::ptr::null(),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Create a new ACPI table iterator from XSDT
    pub unsafe fn from_xsdt(xsdt: &'a Xsdt) -> Self {
        let _start = xsdt as *const _ as *const u8;
        let _header_size = core::mem::size_of::<Xsdt>();

        AcpiTableIterator {
            current_ptr: core::ptr::null(),
            end_ptr: core::ptr::null(),
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'a> Iterator for AcpiTableIterator<'a> {
    type Item = &'a AcpiSdtHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ptr >= self.end_ptr || self.current_ptr.is_null() {
            return None;
        }

        let current = unsafe { &*self.current_ptr };

        // Move to next table
        let next_ptr = unsafe {
            (self.current_ptr as *const u8).add(current.length as usize) as *const AcpiSdtHeader
        };
        self.current_ptr = next_ptr;

        Some(current)
    }
}

/// ACPI Table Finder
pub struct AcpiTableFinder;

impl AcpiTableFinder {
    /// Find a table by signature
    pub unsafe fn find_table<'a>(rsdp: &'a Rsdp, signature: &[u8; 4]) -> Option<&'a AcpiSdtHeader> {
        if rsdp.revision >= 2 {
            // Use XSDT
            let xsdt_ptr = rsdp.xsdt_address as *const Xsdt;
            if !xsdt_ptr.is_null() {
                return Self::find_in_xsdt(&*xsdt_ptr, signature);
            }
        }

        // Use RSDT
        let rsdt_ptr = rsdp.rsdt_address as *const Rsdt;
        if !rsdt_ptr.is_null() {
            return Self::find_in_rsdt(&*rsdt_ptr, signature);
        }

        None
    }

    /// Find table in RSDT
    unsafe fn find_in_rsdt<'a>(rsdt: &'a Rsdt, signature: &[u8; 4]) -> Option<&'a AcpiSdtHeader> {
        let header_size = core::mem::size_of::<AcpiSdtHeader>();
        let entry_count = (rsdt.header.length as usize - header_size) / 4;
        let entries = (rsdt as *const _ as *const u8).add(header_size) as *const u32;

        for i in 0..entry_count {
            let table_ptr = *entries.add(i) as *const AcpiSdtHeader;
            if !table_ptr.is_null() {
                let table = &*table_ptr;
                if &table.signature == signature {
                    return Some(table);
                }
            }
        }

        None
    }

    /// Find table in XSDT
    unsafe fn find_in_xsdt<'a>(xsdt: &'a Xsdt, signature: &[u8; 4]) -> Option<&'a AcpiSdtHeader> {
        let header_size = core::mem::size_of::<AcpiSdtHeader>();
        let entry_count = (xsdt.header.length as usize - header_size) / 8;
        let entries = (xsdt as *const _ as *const u8).add(header_size) as *const u64;

        for i in 0..entry_count {
            let table_ptr = *entries.add(i) as *const AcpiSdtHeader;
            if !table_ptr.is_null() {
                let table = &*table_ptr;
                if &table.signature == signature {
                    return Some(table);
                }
            }
        }

        None
    }

    /// Find HPET table
    pub unsafe fn find_hpet(rsdp: &Rsdp) -> Option<&AcpiHpet> {
        Self::find_table(rsdp, b"HPET").map(|h| &*(h as *const _ as *const AcpiHpet))
    }

    /// Find MCFG table
    pub unsafe fn find_mcfg(rsdp: &Rsdp) -> Option<&AcpiMcfg> {
        Self::find_table(rsdp, b"MCFG").map(|h| &*(h as *const _ as *const AcpiMcfg))
    }

    /// Find BGRT table
    pub unsafe fn find_bgrt(rsdp: &Rsdp) -> Option<&AcpiBgrt> {
        Self::find_table(rsdp, b"BGRT").map(|h| &*(h as *const _ as *const AcpiBgrt))
    }

    /// Find DMAR table
    pub unsafe fn find_dmar(rsdp: &Rsdp) -> Option<&AcpiDmar> {
        Self::find_table(rsdp, b"DMAR").map(|h| &*(h as *const _ as *const AcpiDmar))
    }
}

/// MCFG Helper Functions
pub mod mcfg_helpers {
    use super::*;

    /// Get MCFG configuration space entries
    pub unsafe fn get_config_spaces(mcfg: &AcpiMcfg) -> &[McfgConfigSpaceEntry] {
        let header_size = core::mem::size_of::<AcpiMcfg>();
        let entry_size = core::mem::size_of::<McfgConfigSpaceEntry>();
        let entry_count = (mcfg.header.length as usize - header_size) / entry_size;

        let entries_ptr =
            (mcfg as *const _ as *const u8).add(header_size) as *const McfgConfigSpaceEntry;
        core::slice::from_raw_parts(entries_ptr, entry_count)
    }

    /// Get PCIe MMIO address for a device
    pub fn get_pcie_address(
        entry: &McfgConfigSpaceEntry,
        bus: u8,
        device: u8,
        function: u8,
    ) -> Option<u64> {
        if bus < entry.start_bus_number || bus > entry.end_bus_number {
            return None;
        }

        if device >= 32 || function >= 8 {
            return None;
        }

        let offset = ((bus as u64) << 20) | ((device as u64) << 15) | ((function as u64) << 12);
        Some(entry.base_address + offset)
    }
}

/// HPET Helper Functions
pub mod hpet_helpers {
    use super::*;

    /// Check if HPET counter is 64-bit
    pub fn is_64bit_counter(hpet: &AcpiHpet) -> bool {
        hpet.counter_size != 0
    }

    /// Get number of comparators
    pub fn get_comparator_count(hpet: &AcpiHpet) -> u8 {
        hpet.comparator_count + 1
    }

    /// Check if legacy replacement is enabled
    pub fn has_legacy_replacement(hpet: &AcpiHpet) -> bool {
        hpet.legacy_replacement != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdt_header_size() {
        assert_eq!(core::mem::size_of::<AcpiSdtHeader>(), 36);
    }

    #[test]
    fn test_mcfg_entry_size() {
        assert_eq!(core::mem::size_of::<McfgConfigSpaceEntry>(), 16);
    }

    #[test]
    fn test_pcie_address_calculation() {
        let entry = McfgConfigSpaceEntry {
            base_address: 0xE0000000,
            pci_segment_group: 0,
            start_bus_number: 0,
            end_bus_number: 255,
            reserved: 0,
        };

        // Bus 0, Device 0, Function 0
        let addr = mcfg_helpers::get_pcie_address(&entry, 0, 0, 0);
        assert_eq!(addr, Some(0xE0000000));

        // Bus 1, Device 2, Function 3
        let addr = mcfg_helpers::get_pcie_address(&entry, 1, 2, 3);
        let expected = 0xE0000000 + (1 << 20) + (2 << 15) + (3 << 12);
        assert_eq!(addr, Some(expected));
    }
}
