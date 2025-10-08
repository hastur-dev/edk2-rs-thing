// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Configuration Table Access

use crate::ffi::*;
use crate::system_table::SystemTable;

/// EFI_CONFIGURATION_TABLE
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConfigurationTable {
    pub vendor_guid: Guid,
    pub vendor_table: *mut core::ffi::c_void,
}

impl ConfigurationTable {
    /// Check if this table matches a given GUID
    pub fn matches_guid(&self, guid: &Guid) -> bool {
        self.vendor_guid == *guid
    }

    /// Get the vendor table as a typed pointer
    ///
    /// # Safety
    /// Caller must ensure the type T matches the actual table type
    pub unsafe fn as_table<T>(&self) -> Option<&'static T> {
        if self.vendor_table.is_null() {
            None
        } else {
            Some(&*(self.vendor_table as *const T))
        }
    }
}

/// Configuration table iterator
pub struct ConfigurationTableIter<'a> {
    tables: &'a [ConfigurationTable],
    index: usize,
}

impl<'a> ConfigurationTableIter<'a> {
    /// Create a new configuration table iterator
    pub fn new(system_table: &'a SystemTable) -> Self {
        let tables = unsafe {
            if system_table.configuration_table.is_null()
                || system_table.number_of_table_entries == 0
            {
                &[]
            } else {
                core::slice::from_raw_parts(
                    system_table.configuration_table,
                    system_table.number_of_table_entries,
                )
            }
        };

        ConfigurationTableIter { tables, index: 0 }
    }

    /// Find a table by GUID
    pub fn find_by_guid(&self, guid: &Guid) -> Option<&'a ConfigurationTable> {
        self.tables.iter().find(|t| t.matches_guid(guid))
    }
}

impl<'a> Iterator for ConfigurationTableIter<'a> {
    type Item = &'a ConfigurationTable;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tables.len() {
            let table = &self.tables[self.index];
            self.index += 1;
            Some(table)
        } else {
            None
        }
    }
}

/// Helper functions for common configuration tables
pub mod config_helpers {
    use super::*;
    use crate::tables::acpi::*;
    use crate::tables::smbios::*;

    /// Find ACPI RSDP 2.0 table
    pub fn find_acpi_20_table(system_table: &SystemTable) -> Option<&RsdpDescriptor20> {
        let iter = ConfigurationTableIter::new(system_table);
        iter.find_by_guid(&ACPI_20_TABLE_GUID)
            .and_then(|t| unsafe { t.as_table::<RsdpDescriptor20>() })
    }

    /// Find ACPI RSDP 1.0 table
    pub fn find_acpi_10_table(system_table: &SystemTable) -> Option<&RsdpDescriptor10> {
        let iter = ConfigurationTableIter::new(system_table);
        iter.find_by_guid(&ACPI_TABLE_GUID)
            .and_then(|t| unsafe { t.as_table::<RsdpDescriptor10>() })
    }

    /// Find SMBIOS 3.0 table
    pub fn find_smbios3_table(system_table: &SystemTable) -> Option<&Smbios3EntryPoint> {
        let iter = ConfigurationTableIter::new(system_table);
        iter.find_by_guid(&SMBIOS3_TABLE_GUID)
            .and_then(|t| unsafe { t.as_table::<Smbios3EntryPoint>() })
    }

    /// Find SMBIOS table
    pub fn find_smbios_table(system_table: &SystemTable) -> Option<&SmbiosEntryPoint> {
        let iter = ConfigurationTableIter::new(system_table);
        iter.find_by_guid(&SMBIOS_TABLE_GUID)
            .and_then(|t| unsafe { t.as_table::<SmbiosEntryPoint>() })
    }
}
