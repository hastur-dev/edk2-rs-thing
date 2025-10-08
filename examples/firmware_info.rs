// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Firmware Information Example
//!
//! This example demonstrates how to read firmware tables (ACPI, SMBIOS)
//! and display system information.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use uefi_rust::protocols::*;
use uefi_rust::tables::*;
use uefi_rust::*;

#[no_mangle]
pub extern "C" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    // Initialize allocator
    unsafe {
        uefi_rust::allocator::init(system_table);
    }

    // Get console output
    let st = unsafe { &mut *system_table };
    let con_out = unsafe { &mut *st.con_out };

    // Print banner
    print_string(con_out, "Firmware Information Tool\r\n");
    print_string(con_out, "=========================\r\n\r\n");

    // Display EFI System Table info
    print_string(con_out, "EFI System Table\r\n");
    print_string(con_out, "----------------\r\n");
    print_string(con_out, "Firmware Vendor: ");
    print_ucs2_string(con_out, st.firmware_vendor);
    print_string(con_out, "\r\n");

    print_string(con_out, "Firmware Revision: ");
    print_hex(con_out, st.firmware_revision);
    print_string(con_out, "\r\n");

    print_string(con_out, "Header Revision: ");
    print_hex(con_out, st.hdr.revision);
    print_string(con_out, "\r\n\r\n");

    // Search for ACPI tables
    print_string(con_out, "ACPI Tables\r\n");
    print_string(con_out, "-----------\r\n");

    let config_table_count = st.number_of_table_entries;
    let config_table =
        unsafe { core::slice::from_raw_parts(st.configuration_table, config_table_count) };

    let mut found_acpi = false;
    for entry in config_table {
        if entry.vendor_guid == acpi::ACPI_TABLE_GUID
            || entry.vendor_guid == acpi::ACPI_20_TABLE_GUID
        {
            print_string(con_out, "ACPI Table found at: ");
            print_hex(con_out, entry.vendor_table as usize as u32);
            print_string(con_out, "\r\n");

            // Try to parse RSDP
            let rsdp_ptr = entry.vendor_table as *const acpi::Rsdp;
            if !rsdp_ptr.is_null() {
                let rsdp = unsafe { &*rsdp_ptr };

                // Validate signature
                let sig = core::str::from_utf8(&rsdp.signature);
                if let Ok(sig_str) = sig {
                    print_string(con_out, "  Signature: ");
                    print_string(con_out, sig_str);
                    print_string(con_out, "\r\n");
                }

                print_string(con_out, "  Revision: ");
                print_number(con_out, rsdp.revision as u32);
                print_string(con_out, "\r\n");

                let oem_id = core::str::from_utf8(&rsdp.oem_id);
                if let Ok(oem_str) = oem_id {
                    print_string(con_out, "  OEM ID: ");
                    print_string(con_out, oem_str);
                    print_string(con_out, "\r\n");
                }

                found_acpi = true;
            }
        }
    }

    if !found_acpi {
        print_string(con_out, "No ACPI tables found\r\n");
    }

    print_string(con_out, "\r\n");

    // Search for SMBIOS tables
    print_string(con_out, "SMBIOS Tables\r\n");
    print_string(con_out, "--------------\r\n");

    let mut found_smbios = false;
    for entry in config_table {
        if entry.vendor_guid == smbios::SMBIOS_TABLE_GUID
            || entry.vendor_guid == smbios::SMBIOS3_TABLE_GUID
        {
            print_string(con_out, "SMBIOS Table found at: ");
            print_hex(con_out, entry.vendor_table as usize as u32);
            print_string(con_out, "\r\n");

            found_smbios = true;

            // Note: Full SMBIOS parsing would go here
            // For this example, we just show it was found
        }
    }

    if !found_smbios {
        print_string(con_out, "No SMBIOS tables found\r\n");
    }

    print_string(con_out, "\r\n");

    // Display memory information
    print_string(con_out, "Memory Information\r\n");
    print_string(con_out, "------------------\r\n");

    let mut memory_map_size: usize = 0;
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let boot_services = unsafe { &mut *st.boot_services };

    // First call to get size
    let _ = unsafe {
        (boot_services.get_memory_map)(
            &mut memory_map_size,
            core::ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        )
    };

    print_string(con_out, "Memory map size: ");
    print_number(con_out, memory_map_size as u32);
    print_string(con_out, " bytes\r\n");

    print_string(con_out, "Descriptor size: ");
    print_number(con_out, descriptor_size as u32);
    print_string(con_out, " bytes\r\n");

    print_string(con_out, "Descriptor version: ");
    print_number(con_out, descriptor_version);
    print_string(con_out, "\r\n\r\n");

    print_string(con_out, "Firmware info display complete!\r\n");

    EFI_SUCCESS
}

fn print_string(con_out: &mut SimpleTextOutputProtocol, s: &str) {
    let mut buf = [0u16; 256];
    let len = s.len().min(255);

    for (i, byte) in s.bytes().take(len).enumerate() {
        buf[i] = byte as u16;
    }
    buf[len] = 0;

    unsafe {
        (con_out.output_string)(con_out, buf.as_ptr() as *mut Char16);
    }
}

fn print_ucs2_string(con_out: &mut SimpleTextOutputProtocol, s: *const Char16) {
    if s.is_null() {
        return;
    }

    unsafe {
        (con_out.output_string)(con_out, s as *mut Char16);
    }
}

fn print_number(con_out: &mut SimpleTextOutputProtocol, n: u32) {
    let mut buf = [0u16; 32];
    let mut num = n;

    if num == 0 {
        buf[0] = '0' as u16;
        buf[1] = 0;
    } else {
        let mut digits = [0u16; 10];
        let mut count = 0;

        while num > 0 {
            digits[count] = (num % 10) as u16 + '0' as u16;
            num /= 10;
            count += 1;
        }

        for j in 0..count {
            buf[j] = digits[count - 1 - j];
        }
        buf[count] = 0;
    }

    unsafe {
        (con_out.output_string)(con_out, buf.as_ptr() as *mut Char16);
    }
}

fn print_hex(con_out: &mut SimpleTextOutputProtocol, n: u32) {
    let mut buf = [0u16; 16];
    let hex_chars = b"0123456789ABCDEF";

    buf[0] = '0' as u16;
    buf[1] = 'x' as u16;

    for i in 0..8 {
        let nibble = ((n >> (28 - i * 4)) & 0xF) as usize;
        buf[i + 2] = hex_chars[nibble] as u16;
    }
    buf[10] = 0;

    unsafe {
        (con_out.output_string)(con_out, buf.as_ptr() as *mut Char16);
    }
}
