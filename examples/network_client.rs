// SPDX-License-Identifier: BSD-2-Clause-Patent
//! Network Client Example
//!
//! This example demonstrates how to use the TCP/UDP protocols to create
//! a simple network client in UEFI.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::Vec;
use uefi_rust::boot_services::*;
use uefi_rust::protocols::*;
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
    print_string(con_out, "Network Client Example\r\n");
    print_string(con_out, "======================\r\n\r\n");

    // Locate TCP4 protocol
    let boot_services = unsafe { &mut *st.boot_services };
    let mut tcp4_protocol: *mut Tcp4Protocol = core::ptr::null_mut();

    let status = unsafe {
        (boot_services.locate_protocol)(
            &TCP4_PROTOCOL_GUID,
            core::ptr::null_mut(),
            &mut tcp4_protocol as *mut _ as *mut *mut core::ffi::c_void,
        )
    };

    if status != EFI_SUCCESS {
        print_string(con_out, "Failed to locate TCP4 protocol\r\n");
        return status;
    }

    print_string(con_out, "TCP4 protocol located successfully\r\n");

    // Configure TCP4 connection
    let mut config = Tcp4ConfigData {
        type_of_service: 0,
        time_to_live: 64,
        access_point: Tcp4AccessPoint {
            use_default_address: 1,
            station_address: Ipv4Address { addr: [0, 0, 0, 0] },
            subnet_mask: Ipv4Address { addr: [0, 0, 0, 0] },
            station_port: 0,
            remote_address: Ipv4Address {
                addr: [192, 168, 1, 1],
            },
            remote_port: 80,
            active_flag: 1,
        },
        control_option: core::ptr::null_mut(),
    };

    let status = unsafe {
        let tcp4 = &mut *tcp4_protocol;
        tcp4.configure(Some(&config))
    };

    if status == EFI_SUCCESS {
        print_string(con_out, "TCP4 configured successfully\r\n");
        print_string(con_out, "Remote: 192.168.1.1:80\r\n");
    } else {
        print_string(con_out, "Failed to configure TCP4\r\n");
        return status;
    }

    // Create connection token
    let mut connection_token = Tcp4CompletionToken {
        event: core::ptr::null_mut(),
        status: EFI_SUCCESS,
    };

    // Note: In a real implementation, you would:
    // 1. Create an event for the connection token
    // 2. Call tcp4.connect()
    // 3. Wait for the event to be signaled
    // 4. Transmit and receive data
    // 5. Close the connection properly

    print_string(con_out, "\r\nExample demonstrates TCP4 protocol usage\r\n");
    print_string(con_out, "In a real app, you would:\r\n");
    print_string(con_out, "  1. Create events for async operations\r\n");
    print_string(con_out, "  2. Connect to remote host\r\n");
    print_string(con_out, "  3. Transmit HTTP request\r\n");
    print_string(con_out, "  4. Receive HTTP response\r\n");
    print_string(con_out, "  5. Close connection\r\n");

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
