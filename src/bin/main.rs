// SPDX-License-Identifier: BSD-2-Clause-Patent
//! UEFI Application Entry Point

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use uefi_rust_intergration::*;

/// UEFI Application Entry Point
///
/// This is the standard UEFI entry point signature
#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: *mut Handle,
    system_table: *mut SystemTable,
) -> Status {
    unsafe {
        // Initialize system table reference
        let st = &*system_table;

        // Initialize the allocator with Boot Services
        let bs = st.boot_services();
        allocator::init_allocator(bs);

        // Print a message to the console
        print_string(st, "Hello from Rust UEFI Application!\r\n");

        // Test allocator by creating a vector
        let mut test_vec = Vec::new();
        test_vec.push(1u32);
        test_vec.push(2u32);
        test_vec.push(3u32);

        print_string(st, "Allocator test successful!\r\n");
        print_string(st, "Rust is running in UEFI!\r\n");

        // Wait for a moment before exiting
        let _ = (bs.stall)(5_000_000); // 5 seconds

        EFI_SUCCESS
    }
}

/// Helper function to print a string to UEFI console
unsafe fn print_string(st: &SystemTable, msg: &str) {
    let stdout = st.stdout();

    // Convert Rust string to UCS-2 (CHAR16)
    let buffer: alloc::vec::Vec<Char16> = msg
        .encode_utf16()
        .chain(core::iter::once(0)) // Null terminator
        .collect();

    let _ = (stdout.output_string)(stdout, buffer.as_ptr());
}
